use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct Executor<'a, T, U>
{
	count: u32,
	next: u32,
	ready_handler: Option<&'a mut T>,
	executor_data: Vec<ExecutorData<U>>,
	executors: Vec<thread::JoinHandle<()>>
}

struct ExecutorData<T>
{
	to: mpsc::Sender<T>,
	from: mpsc::Receiver<T>
}

pub trait Work
{
	fn work(&mut self);

	fn dump(&self);
}

pub trait Ready
{
	fn ready<T>(&mut self, work: T) -> bool where T: Work;
}

impl<'a, T: Ready, U: Send + Work + 'static> Executor<'a, T, U>
{
	pub fn new(count: u32) -> Executor<'a, T, U>
	{
		let mut exec = Executor::<T, U> { count: count, next: 0, ready_handler: None,
					executor_data: Vec::with_capacity(count as usize),
			   	      	executors: Vec::with_capacity(count as usize) };

		for _ in 0..exec.count {
			let (tx1, rx1) = mpsc::channel::<U>();
			let (tx2, rx2) = mpsc::channel::<U>();

			exec.executor_data.push(ExecutorData { to: tx1, from: rx2 });
			exec.executors.push(std::thread::spawn(move || {
				loop {
					let data = rx1.recv();

					if data.is_ok() {
						let mut data = data.unwrap();

						data.work();
						let _ = tx2.send(data);
					} else {
						break;
					}
				}
			}));
		}

		exec
	}

	pub fn set_ready_handler(&mut self, handler: &'a mut T)
	{
		self.ready_handler = Some(handler);
	}

	pub fn unset_ready_handler(&mut self)
	{
		self.ready_handler = None;
	}

	pub fn execute(&mut self, work: U)
	{
		let _ = self.executor_data[(self.next % self.count) as usize].to.send(work);
		self.next += 1;
	}

	pub fn execute_at(&mut self, work: U, id: u32)
	{
		let _ = self.executor_data[id as usize].to.send(work);
	}

	pub fn handle_ready(&mut self) -> bool
	{
		for data in self.executor_data.iter_mut() {
			let work = data.from.recv_timeout(Duration::from_secs(0));

			if work.is_ok() && self.ready_handler.is_some() {
				if self.ready_handler.as_mut().unwrap().ready(work.unwrap()) {
					return true;
				}
			}
		}

		false
	}
}
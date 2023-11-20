use remoteexecutor;

struct Work
{
	computation1: u32,
	computation2: u32
}

impl Work
{
	pub fn new(init: u32) -> Work
	{
		let work = Work { computation1: init, computation2: init };

		work
	}
}

impl remoteexecutor::Work for Work
{
	fn work(&mut self) {
		self.computation1 += 1234;
		self.computation2 += 1337;
	}

	fn dump(&self) {
		println!("work done, comp1: {}, comp2: {}", self.computation1, self.computation2);
	}
}

struct Ready
{
	work_count: u32
}

impl Ready
{
	fn new() -> Ready
	{
		let ready = Ready { work_count: 0 };

		ready
	}
}

impl remoteexecutor::Ready for Ready
{
	fn ready<T>(&mut self, work: T) -> bool where T: remoteexecutor::Work {
		work.dump();

		if self.work_count < 2 {
			self.work_count += 1;
			return false
		}

		true
	}
}

pub fn main()
{
	let w1 = Work::new(1000);
	let w2 = Work::new(2000);
	let w3 = Work::new(3000);
	let mut handler = Ready::new();
	let mut exec = remoteexecutor::Executor::<Ready, Work>::new(4);

	exec.set_ready_handler(&mut handler);
	exec.execute(w1);
	exec.execute(w2);
	exec.execute(w3);

	loop {
		if exec.handle_ready() {
			break;
		}
	}
}
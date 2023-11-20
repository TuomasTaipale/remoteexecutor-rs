# remoteexecutor

Simple framework for issuing work to a set of workers.

## Compilation

These instructions assume that the project has been cloned into directory named `remoteexecutor`.

Build with `cargo`:

    cd remoteexecutor
    cargo build

Examples can be built with:

    cargo build --examples
    ./target/debug/examples/<executable>

## Installation

Dependency to `remoteexecutor` can be declared for example with a GitHub link:

    [dependencies]
    regex = { git = "https://github.com/TuomasTaipale/remoteexecutor-rs.git" }


Note that `cargo` command allows plethora of additional options. For more info:

    cargo --help

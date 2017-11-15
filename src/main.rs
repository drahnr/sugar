#[macro_use]
extern crate error_chain;

extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use futures::stream::Stream;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;

use futures::future::Future;
use futures::{Async,Poll};

use std::vec::Vec;

error_chain! {
    // Automatic conversions between this error chain and other
    // error types not defined by the `error_chain!`. These will be
    // wrapped in a new error with, in the first case, the
    // `ErrorKind::Fmt` variant. The description and cause will
    // forward to the description and cause of the original error.
    //
    // Optionally, some attributes can be added to a variant.
    //
    // This section can be empty.
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error) #[cfg(unix)];
    }

    // Define additional `ErrorKind` variants.  Define custom responses with the
    // `description` and `display` calls.
    errors {
        Stuff(t: String) {
            description("invalid toolchain name")
            display("invalid toolchain name: '{}'", t)
        }

        // You can also add commas after description/display.
        // This may work better with some editor auto-indentation modes:
        Things(v: String) {
            description("unknown toolchain version"), // note the ,
            display("unknown toolchain version: '{}'", v), // trailing comma is allowed
        }
    }
}


enum Action {
    Carry,
    Occupy,
    DillyDally,
}


struct SuggarDispenser;

impl Stream for SuggarDispenser {
    type Item = ();
    type Error = Error;
    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(Async::Ready(SuggarCube::new()))
    }
}

struct SuggarCube;

impl SuggarCube {
    fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
enum State<'a> {
    Idle,
    Work(&'a Action),
}

#[derive(Debug)]
struct Ant<'a> {
    state : State<'a>,
}


#[derive(Debug)]
struct Swarm<'a> {
    minions : Vec<Ant<'a>>,
}

impl Swarm {
    fn new() -> Self {
        Self {
            minions : vec![],
        }
    }
}

impl<'a> Future for Swarm<'a> {

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        type Item = ();
        type Error = Error;

        for minion in self.minions {
            minion.poll();
        }
    }
}

fn main() {
    let mut core = Core::new().unwrap();

    let connections = listener.incoming();
    let welcomes = connections.and_then(|(socket, _peer_addr)| {

        tokio_io::io::write_all(socket, b"Hello, world!\n")
    });
    let server = welcomes.for_each(|(_socket, _welcome)| {
        Ok(())
    });

    core.run(server).unwrap();

    println!("Hello, world!");
}

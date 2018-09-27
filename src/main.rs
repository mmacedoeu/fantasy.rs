//! * Breath of Fantasy
//! This is the main binary of the solution.
//! It follows 12 Factors [principles](https://en.wikipedia.org/wiki/Twelve-Factor_App_methodology)
//! It just handles exit codes and delegates
//! everything else to the client library
//! by separating the binary from library you can reuse
//! this code on every project despite what it intends to do
//! leveraging reusability

#![cfg_attr(feature="flame_it", feature(plugin, custom_attribute))]
#![cfg_attr(feature="flame_it", plugin(flamer))]

#[cfg(feature="flame_it")]
extern crate flame;
extern crate client;
extern crate failure;

#[cfg(feature="flame_it")]
use std::fs::File;

pub trait ExitCode {
    /// Returns the value to use as the exit status.
    fn code(self) -> i32;
}

impl ExitCode for i32 {
    fn code(self) -> i32 {
        self
    }
}

impl ExitCode for () {
    fn code(self) -> i32 {
        0
    }
}

fn main() {
    use std::io::Write;

    ::std::process::exit(match run() {
        Ok(ret) => {
            #[cfg(feature="flame_it")]
            {
                // Dump the report to disk
                flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();

                // Or read and process the data yourself!
                let _spans = flame::spans();

                // println!("{:?}", _spans);                
            };
            ExitCode::code(ret)
        }
        Err(ref e) => {
            write!(&mut ::std::io::stderr(), "{}", e).expect("Error writing to stderr");
            1
        }
    });
}

// Use of the forwarding pattern here to call the inner client library run
// method, with the exception from the pattern there is no real object:
// https://en.wikipedia.org/wiki/Forwarding_(object-oriented_programming)
fn run() -> Result<(), failure::Error> {
    client::run(::std::env::args())
}

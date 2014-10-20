#![crate_name = "static_file"]
#![deny(missing_doc)]
#![feature(phase)]

//! Static file-serving handler.

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

extern crate http;
extern crate iron;
#[phase(plugin, link)]
extern crate log;
extern crate mount;


pub use static_handler::Static;


mod static_handler;

#![allow(unused)]
#![allow(non_upper_case_globals)]

use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

pub fn main() {
    let mut registry = lib::Registry::default();
    registry.push(lib::Node::new());
    println!("{}", config::url);
    let listener = TcpListener::bind(config::bind).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle(&mut registry, stream);
    }
}

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub http);

/// HTTP requests handler (via LALRPOPped parser)
fn handle(registry: &mut lib::Registry, mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req = buf_reader.lines().next().unwrap().unwrap();
    println!("{}", req);
    http::GETParser::new()
        .parse(registry, &mut stream, &req)
        .unwrap();
}

const OK_200: &[u8] = b"HTTP 200 OK\n";
const TEXT_PLAIN: &[u8] = b"Content-Type: text/plain\n";
const TEXT_HTML: &[u8] = b"Content-Type: text/html\n";
const IMAGE_PNG: &[u8] = b"Content-Type: image/png\n";

const INDEX_HEAD: &[u8] = include_bytes!("../static/index.html.head");
const INDEX_TAIL: &[u8] = include_bytes!("../static/index.html.tail");
const LOGO_PNG: &[u8] = include_bytes!("../static/logo.png");

pub fn serve(mut stream: &TcpStream, ctype: &[u8], data: &[u8]) {
    stream.write(OK_200).unwrap();
    stream.write(ctype).unwrap();
    stream.write(b"\n").unwrap();
    stream.write(data).unwrap();
}

// #[macro_use]
// extern crate lazy_static;

// lazy_static! {
//     pub static ref registry: lib::Registry = { lib::Registry::default() };
// }

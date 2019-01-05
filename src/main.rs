#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;

extern crate socket2;

use std::io::prelude::*;
use std::fs::File;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = r#"
A networked filesystem operating over multicast.
The goal is to make sharing of files over a LAN as simple and trivial as possible.

Usage:
  mcastfs client [--group4=<group4>] [--port=<port>] <client_cmd_args>...
  mcastfs serve  [--group4=<group4>] [--port=<port>] <directories>...
  mcastfs (-h | --help)
  mcastfs --version

Options:
  -h --help              Show this screen.
  --version              Show version.
  --group4=<group4>  Multicast IP [default: "239.1.0.1"].
  --port=<port>          Multicast Port [default: 2112].
"#;

#[derive(Debug, Deserialize)]
struct Args {
    arg_directories: Vec<String>,     // ["/home/me/MyDocs"]
    arg_client_cmd_args: Vec<String>, // ["ls"], ["ls", "-alh"], etc.
    flag_group4: String,              // "239.1.0.1"
    flag_port: u16,                   // 2112
    flag_version: bool,
    cmd_client: bool,
    cmd_serve: bool,
}

fn main() {
  let args: Args = Docopt::new(USAGE)
                      .and_then(|d| d.deserialize())
                      .unwrap_or_else(|e| e.exit());
  if args.flag_version {
    println!("mcastfs version {}", VERSION);
    return;
  }
  
  sanitize_args(&args);
  
  if args.cmd_client {
    do_client(&args);
  }
  else if args.cmd_serve {
    do_serve(&args);
  }
  
}

fn sanitize_args(args: &Args) {
  let group4: Ipv4Addr = args.flag_group4.parse().unwrap_or(Ipv4Addr::new(0, 0, 0, 0));
  if ! group4.is_multicast() {
    println!("Cannot use '{}' because it is not a multicast IP address.", args.flag_group4);
    ::std::process::exit(1);
  }
}

fn do_client(args: &Args) {
  
}

fn do_serve(args: &Args) {
  
}


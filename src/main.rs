#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;

use std::io::prelude::*;
use std::fs::File;


const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = r#"
A networked filesystem operating over multicast.
The goal is to make sharing of files over a LAN as simple and trivial as possible.

Usage:
  mcastfs client [--group=<mcast_group>] <client_cmd_args>...
  mcastfs serve  [--group=<mcast_group>] <directories>...
  mcastfs (-h | --help)
  mcastfs --version

Options:
  -h --help              Show this screen.
  --version              Show version.
  --group=<mcast_group>  Multicast IP [default: "239.1.0.1"].
"#;

#[derive(Debug, Deserialize)]
struct Args {
    arg_directories: Vec<String>,     // ["/home/me/MyDocs"]
    arg_client_cmd_args: Vec<String>, // ["ls"], ["ls", "-alh"], etc.
    flag_mcast_group: String,         // "239.1.0.1"
    flag_version: bool,
}

fn main() {
  let args: Args = Docopt::new(USAGE)
                      .and_then(|d| d.deserialize())
                      .unwrap_or_else(|e| e.exit());
  if args.flag_version {
    println!("mcastfs version {}", VERSION);
    return;
  }
  
  
  
  
}
#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;

extern crate serde_json;

use serde_json::{Value, Error};

extern crate socket2;

extern crate hostname;

use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use std::time::Duration;

use socket2::{SockAddr, Domain, Protocol, Socket, Type};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = r#"
A networked filesystem operating over multicast.
The goal is to make sharing of files over a LAN as simple and trivial as possible.

Usage:
  mcastfs client [--group4=<group4>] [--port=<port>] <args>...
  mcastfs serve  [--group4=<group4>] [--port=<port>] [--hostname=<hostname>] <args>...
  mcastfs (-h | --help)
  mcastfs --version

Options:
  -h --help              Show this screen.
  --version              Show version.
  --group4=<group4>      Multicast IP [default: 224.0.21.12].
  --port=<port>          Multicast Port [default: 2112].
  --hostname=<hostname>  Hostname override.
"#;

#[derive(Debug, Deserialize)]
struct Args {
    arg_args: Vec<String>,     // ["/home/me/MyDocs"] for server, ["ls", "-alh"] for client
    flag_group4: String,              // "239.1.0.1"
    flag_port: u16,                   // 2112
    flag_hostname: Option<String>,
    flag_version: bool,
    cmd_client: bool,
    cmd_serve: bool,
}

fn main() {
  let mut args: Args = Docopt::new(USAGE)
                      .and_then(|d| d.deserialize())
                      .unwrap_or_else(|e| e.exit());
  if args.flag_version {
    println!("mcastfs version {}", VERSION);
    return;
  }
  //println!("args={:#?}", args);
  
  if args.flag_hostname.is_none() {
    // Only detect hostname if not given one
    args.flag_hostname = hostname::get_hostname();
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
  let socket = UdpSocket::bind("0.0.0.0:2121").expect("couldn't bind to 0.0.0.0:2121");
  socket.set_read_timeout(Some(Duration::from_millis(1200))).expect("Could not set timeout to 1200ms");
  
  let json_str = serde_json::to_string(&args.arg_args).expect("Could not go from Vec<String> to JSON");
  
  socket.send_to(&json_str.as_bytes(), format!("{}:{}", args.flag_group4, args.flag_port)).expect("");
  
  let mut buf = [0u8; 8 * 1024];
  match socket.recv_from(&mut buf) {
    Ok((len, _remote_addr)) => {
      let data = &buf[..len];
      let data = String::from_utf8_lossy(data);
      
      println!("Got back from server: {}", data);
      
    }
    Err(err) => {
      println!("Client got an error: {}", err);
    }
  }
  
}

fn do_serve(args: &Args) {
  let socket = Socket::new(Domain::ipv4(), Type::dgram(), Some(Protocol::udp())).expect("Could not construct socket");
  
  socket.set_read_timeout(Some(Duration::from_millis(200))).expect("Could not set 200ms conn timeout");
  
  let ipv4_addr: Ipv4Addr = args.flag_group4.parse().expect("Could not parse given multicast IP");
  socket.join_multicast_v4(&ipv4_addr, &Ipv4Addr::new(0, 0, 0, 0)).expect("Could not join multicast");
  
  let ipv4_socket = SocketAddrV4::new(ipv4_addr, args.flag_port);
  
  socket.bind(&SockAddr::from(ipv4_socket)).expect("Could not bind");
  
  let mut buf = [0u8; 1024 * 8]; // receive buffer
  
  loop {
    match socket.recv_from(&mut buf) {
      Ok((len, remote_addr)) => {
        let data = &buf[..len];
        let parsed_json = String::from_utf8_lossy(data);
        
        //println!("server: got data: {} from: {:?}", parsed_json, remote_addr);
        
        let client_args: Vec<String> = serde_json::from_str(&parsed_json).expect("Could not parse Vec<String> from json");
        
        perform_command(&args, &client_args, &remote_addr);
        
      }
      Err(_err) => {
        // Usually timeouts
        //println!("server: got an error: {}", err);
      }
    }
  }

  
}

fn perform_command(args: &Args, client_args: &Vec<String>, remote_addr: &SockAddr) {
  if client_args.len() < 1 {
    return;
  }
  
  let socket = UdpSocket::bind("0.0.0.0:2122").expect("couldn't bind to 0.0.0.0:2122");
  
  
  match client_args[0].as_str() {
    "ls" => {
      socket.send_to("Some data".as_bytes(), remote_addr.as_inet().expect("Not an ipv4 client") ).expect("");
      
    }
    
    _unk => {
      println!("unknown client_args={:?}", client_args);
    }
  }
  
}


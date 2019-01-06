
# mcastfs

An unfinished but still nifty utility.

I have many times where I want to setup a server to constantly make some files available
(usually regularly generated reports, log files, whatever) and I want to access them over the local network.
There are no security concerns, as the sort of networks I'm operating on are ones I own/manage.

`mcastfs` is a project designed to make it dead simple to make public files and directories over a LAN.
Servers listen to a multicast address, clients multicast requests, servers unicast responsed back to the client.

# Current capabilities

 - [x] Serve directories and files
 - [x] List and view contents of directories and files

# TODO capabilities

 - [ ] Write a FUSE filesystem driver for semi-permanent client setups
 - [ ] Write a forking server impl
 - [ ] Make binary file transfers a possibility
 - [ ] For large files (>50mb) implement the rsync delta transfer algorithm, make clients pick file to do deltas against

# Usage

## Server

```bash
cargo build --release
./target/release/mcastfs serve ./src/
```

## Client

```bash
cargo build --release
./target/release/mcastfs client ls
# All servers will report like:
From host azure-angel:
4.1 kB - "./src"
8.0 kB - "./src/main.rs"

```

## Compiling a static binary

```bash
cargo build --release --target=x86_64-unknown-linux-musl
ldd target/x86_64-unknown-linux-musl/release/mcastfs
  not a dynamic executable
# target/x86_64-unknown-linux-musl/release/mcastfs can be copied and run
# on any 64 bit linux system without needing dependencies
```

## Help text

```
A networked filesystem operating over multicast.
The goal is to make sharing of files over a LAN as simple and trivial as possible.

Usage:
  mcastfs client [--group4=<group4>] [--port=<port>] [--ctimeout=<ctimeout>] <args>...
  mcastfs serve  [--group4=<group4>] [--port=<port>] [--hostname=<hostname>] <args>...
  mcastfs (-h | --help)
  mcastfs --version

Options:
  -h --help              Show this screen.
  --version              Show version.
  --group4=<group4>      Multicast IP [default: 224.0.21.12].
  --port=<port>          Multicast Port [default: 2112].
  --hostname=<hostname>  Hostname override.
  --ctimeout=<ctimeout>  Client receive timeout ms [default: 400]
```

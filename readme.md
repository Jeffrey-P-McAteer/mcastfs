
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



# OneWayPingRS
## "One Way Ping Application"

### "The Argument for OneWayPingRS"

OneWayPingRS is an attempt to measure latency between unidirectional data transmissions of varying sizes. It’s inspired by OWAMP’s one way ping too, and the limitations of that tool.  OWAMP is very good for measuring one way ping, but has a few limitations that prompted me to design a new application. This document will serve a few purposes: To explain OneWayPingRS and how it compares to OWAMP’s one way ping, the design requirements and goals, and the control flow of the program. 

So what makes OneWayPingRS different from OWAMP? OneWayPingRS and OWAMP have the same fundamental goals: to measure unidirectional ping. However they are designed for different use cases and different audiences. When using OWAMP, you have to deal with lots of setup on both the client and host side, setting up an NTP server and connecting to that NTP server on the client, building, compiling and installing OWAMP and setting up its configuration files. It’s not as simple as iperf when it comes to setup(although you can use iperf to measure some latency), and can’t be embedded into other programs as a library. These limitations are the core reason why OneWayPingRS is under development. 

## Design Requirements: 
- Needs to be based around a client and a server. 
- A server will be the “master clock source” so the client will become synced to the server. 
- The client will always be sending the data to the server. 
- The client  will be the device actually processing the overall delay
- The server will only report the timestamp of when the data has completely finished being received and send that over to the client for final processing. 
- Cannot rely on an NTP server.
- To make this more easily embeddable and usable standalone,  we will omit the requirement of an NTP server to handle request latencies. 
- In lieu of that, we will sync the two clocks by having the offset between the two devices calculated. That calculation will then allow the client’s clock to sync to the host’s clock. When the host receives the data, all it needs to do is report its timestamp back and the client will handle the rest. 
- Needs to be embeddable into another application. 
- Since this is mostly going to be used as a testing tool, I want to provide it as a library that can be put into other tools.  The initial use case for this tool is to measure media transfer latency between devices on a local network, so it’s imperative that this can be embedded into other programs. 
- Current target languages are rust, c++ and python but support for other languages is encouraged by the community. 

## Control Flow: Client Standalone Application
1) Start up the OneWayPingRS with the correct arguments: 
  -c to set application into client mode
  -i for ip address
    -i192.168.1.1
  -p for port
    -p2020
2) Attempt to connect to the server, block until the time has been synced up between the client and server
3) Prompt the User, or parse out console output for message size and total packets sent
4) Send out packets. 
5) Record Data
6) Print out results

## Control Flow: Server Standalone Application
1) Start up the OneWayPingRS with the correct arguments: 
  -c to set application into client mode
  -i for ip address
    -i192.168.1.1
  -p for port
    -p2020
2) Wait until client has attempted to connect, send timestamp for adjustment
3) Log when packages are sent from Client to Server
4) Ship out all timestamp data after transmission to Client for processing
5) Receive processed data from Client to print to screen for latency details

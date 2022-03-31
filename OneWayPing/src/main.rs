use std::net::UdpSocket;
use std::net::SocketAddr;
use std::env;
use chrono::prelude::*;
extern crate local_ip; 

fn main() {
    let base_arguments = get_arguments();
    
    // Whether or not this will be a server or client
    match base_arguments.is_client {
        true=>start_client(base_arguments), 
        false=>start_server(base_arguments),
    }
}

struct BaseArguments{
    is_client: bool,
    port: u16, 
    ip_addr: String, 
    total_messages: u64, 
    message_size_bytes: u64
}

fn get_arguments() -> BaseArguments{
    let args_input: Vec<String> = env::args().collect();
    
    let mut check_is_client = false; 
    let mut invalid_input = true;
    let mut parsed_port: u16 = 3030; 
    let mut parsed_ip: String = String::from(""); 
    let mut parsed_message_total: u64 = 0; 
    let mut parsed_message_length: u64 = 0; 
    for strings in &args_input{
        if strings == "-c" {
            check_is_client = true; 
            invalid_input = false; 
        }
        if strings == "-s" {
            invalid_input = false; 
        }
        if strings.get(0..2).unwrap() == "-p"{
            let port_str = strings.get(2..(strings.chars().count())).unwrap();
            parsed_port = port_str.parse::<u16>().unwrap();
        }

        if strings.get(0..2).unwrap() == "-i"{
            parsed_ip = strings.get(2..(strings.chars().count())).unwrap().to_string();
        }

        if strings.get(0..2).unwrap() == "-m"{
            let msg_num_str = strings.get(2..(strings.chars().count())).unwrap();
            parsed_message_total = msg_num_str.parse::<u64>().unwrap();
        }
        if strings.get(0..2).unwrap() == "-l"{
            let msg_num_str = strings.get(2..(strings.chars().count())).unwrap();
            parsed_message_length = msg_num_str.parse::<u64>().unwrap();
        }
    }

    if invalid_input{
        panic!("Could not parse OneWayPingRS mode... closing.");
    }

    let base_arguments = BaseArguments{ 
        is_client: check_is_client, 
        port: parsed_port,
        ip_addr: parsed_ip, 
        total_messages: parsed_message_total, 
        message_size_bytes: parsed_message_length
    };

    if base_arguments.port < 1024{
        panic!("Can't choose port below 1024");
    }
    if base_arguments.total_messages <= 0{
        panic!("invalid total message count"); 
    }

    return base_arguments;
}

pub fn get_unix_timestamp_us() -> i64 {
    let now = Utc::now();
    now.timestamp_nanos() as i64
}

fn as_i64_le(array: &[u8; 8]) -> i64 {
    ((array[0] as i64) <<  0) +
    ((array[1] as i64) <<  8) +
    ((array[2] as i64) << 16) +
    ((array[3] as i64) << 24) +
    ((array[4] as i64) << 32) +
    ((array[5] as i64) << 40) +
    ((array[6] as i64) << 48) +
    ((array[7] as i64) << 56)
}

fn as_u64_le(array: &[u8; 8]) -> u64 {
    ((array[0] as u64) <<  0) +
    ((array[1] as u64) <<  8) +
    ((array[2] as u64) << 16) +
    ((array[3] as u64) << 24) +
    ((array[4] as u64) << 32) +
    ((array[5] as u64) << 40) +
    ((array[6] as u64) << 48) +
    ((array[7] as u64) << 56)
}

fn setup_connection(socket:  &UdpSocket, ip_port_str: String, total_messages: u64, message_length: u64) -> i64{
    // Send Total Message number to host
    let total_messages_bytestream = total_messages.to_le_bytes();
    let message_size_bytesstream = message_length.to_le_bytes();
    // Send over message sizes and lengths
    socket.send_to(&total_messages_bytestream, &ip_port_str).unwrap();
    socket.send_to(&message_size_bytesstream, &ip_port_str).unwrap();

    // Get current time stamp from device, calculate offset timestamp for one way ping. 
    let mut buf = [0; 8]; 
    socket.recv_from(&mut buf).unwrap(); 
    let timestamp = as_i64_le(&buf); 
    timestamp - get_unix_timestamp_us()
}

fn handle_client(socket: &UdpSocket) -> (u64, u64, SocketAddr){
    let mut buf = [0; 8];
    socket.recv_from(&mut buf).unwrap();
    let num_messages = as_u64_le(&buf);

    let (_ , src) = socket.recv_from(&mut buf).unwrap();
    let msg_length = as_u64_le(&buf);
    
    let timestamp_bytearray = get_unix_timestamp_us().to_le_bytes();
    socket.send_to(&timestamp_bytearray, &src).expect("Couldn't send data");
    return (num_messages, msg_length, src); 
}

fn start_server(base_arguments: BaseArguments){
    println!("One Way Ping Server: getting ready...");
    println!("Waiting accepting Clients");

    let ip = local_ip::get().unwrap();
    let port_string = base_arguments.port.to_string();
    let ip_port_str = String::from(ip.to_string() + &":".to_owned()  + port_string.as_str());
    let udp_socket = UdpSocket::bind(ip_port_str).unwrap();

    udp_socket.set_nonblocking(false).unwrap(); 
    let  (num_messages, msg_length, client_socket) = handle_client(&udp_socket);
    let mut timestamps_ns_arr: Vec<i64> = Vec::with_capacity(num_messages as usize);
    println!("Client Socket: {}, Num Messages {}", client_socket, num_messages);

    let num_transfers_per_msg = msg_length/512 + 1; 
    let mut num_transfers_current_msg = 0; 
    let mut total_transfers = 0; 
    loop{
        let mut buf: [u8; 600] = [0; 600];
        udp_socket.recv(&mut buf).unwrap();
        
        total_transfers += 1; 
        num_transfers_current_msg += 1; 

        if num_transfers_current_msg == num_transfers_per_msg {
            timestamps_ns_arr[total_transfers] = get_unix_timestamp_us();
            num_transfers_current_msg = 0; 
        }
        // Completed Transmisstion
        if total_transfers == num_messages as usize * msg_length as usize{
            break; 
        }
    }
}

fn start_client(base_arguments: BaseArguments){
    println!("One Way Ping Client: getting ready...");
    println!("Establishing Connection with server");

    let ip = local_ip::get().unwrap();
    let port_string = base_arguments.port.to_string();
    let ip_port_str = String::from(ip.to_string() + &":".to_owned() + port_string.as_str());
    println!("{}", ip_port_str);
    let socket = UdpSocket::bind(ip_port_str).unwrap();

    let ip_port_str = base_arguments.ip_addr.as_str().to_owned() + ":" + port_string.as_str();
    socket.set_nonblocking(false).unwrap();

    let time_offset = setup_connection(&socket, ip_port_str, base_arguments.total_messages, base_arguments.message_size_bytes); 

    println!("Time Offset {}", time_offset);
}
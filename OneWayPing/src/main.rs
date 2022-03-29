use std::net::UdpSocket;
use std::io::prelude::*;
use std::io::prelude::*;
use std::env;


fn main() {
    let base_arguments = get_arguments();
    
    // Whether or not this will be a server or client
    match base_arguments.is_client {
        true=>get_arguments_client(base_arguments), 
        false=>get_arguments_server(base_arguments),
    }
}

struct BaseArguments{
    args: Vec<String>, 
    is_client: bool,
    port: u16
}

fn get_arguments() -> BaseArguments{
    let args_input: Vec<String> = env::args().collect();
    
    let mut check_is_client = false; 
    let mut invalid_input = true;
    for strings in &args_input{
        if strings == "-c" {
            check_is_client = true; 
            invalid_input = false; 
        }
        if strings == "-s" {
            invalid_input = false; 
        }
        match strings.get(0..2) {
            
        }
    }

    if invalid_input{
        panic!("Could not parse OneWayPingRS mode... closing.");
    }

    let base_arguments = BaseArguments{
        args: args_input, 
        is_client: check_is_client
    };
    return base_arguments;
}

fn get_arguments_server(base_arguments: BaseArguments){
    println!("One Way Ping Server: getting ready...");

}

fn get_arguments_client(base_arguments: BaseArguments){
    println!("One Way Ping Client: getting ready...");
}

fn server(){

}

fn client(){

}
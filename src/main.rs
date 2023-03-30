//use std::net::TcpListener;
use sysinfo::{System, SystemExt, NetworkExt};

fn main() {
    let  sys = System::new_all();
    for(interface_nam,data) in sys.networks(){
        if data.mac_address().to_string() != "00:00:00:00:00:00" {
            println!("{}: {} (mac)",interface_nam,data.mac_address());
        }

    }
    //
    // let listener = TcpListener::bind("127.0.0.1:5010").unwrap();

    // for _stream in listener.incoming(){
    //     let stream = _stream.unwrap();
        
    //     println!("connected {}", stream.ttl().unwrap());
    // }
}

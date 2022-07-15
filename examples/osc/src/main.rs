use bmd_speededitor;
use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;

static FROM_ADDR: &str = "127.0.0.1:0";
static TO_ADDR: &str = "127.0.0.1:5000";

fn main() {
    let host_addr = SocketAddrV4::from_str(FROM_ADDR).unwrap();
    let to_addr = SocketAddrV4::from_str(TO_ADDR).unwrap();
    let socket = UdpSocket::bind(host_addr).unwrap();

    match bmd_speededitor::new() {
        Ok(mut se) => {
            let s = socket.try_clone().unwrap();
            let s2 = socket.try_clone().unwrap();
            let s3 = socket.try_clone().unwrap();
            se.on_key_down(move |_, key| {
                println!("key down event: {}", key);
                send_osc(&s, to_addr, &key.to_owned().to_string(), 1.0);
                Ok(())
            });
            se.on_key_up(move |_, key| {
                println!("key up event: {}", key);
                send_osc(&s2, to_addr, &key.to_owned().to_string(), 0.0);
                Ok(())
            });
            se.on_jog(move |_, mode, value| {
                println!("jog event: {} / {}", mode, value);
                send_osc(&s3, to_addr, "jog".to_string().as_str(), value as f32);
                Ok(())
            });
            se.run().unwrap();
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}

fn send_osc(socket: &UdpSocket, to_addr: SocketAddrV4, addr: &str, value: f32) {
    let message = OscMessage {
        addr: format!("/{}", addr),
        args: vec![OscType::Float(value)],
    };
    let buf = encoder::encode(&OscPacket::Message(message)).unwrap();
    socket.send_to(&buf, to_addr).unwrap();
}

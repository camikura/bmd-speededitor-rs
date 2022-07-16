use bmd_speededitor::{self, Key};
use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;

static FROM_ADDR: &str = "127.0.0.1:0";
static TO_ADDR: &str = "127.0.0.1:5000";

pub enum SpeedEditorEvent {
    KeyEvent(Key, bool),
    JogEvent(u8, i32),
}

fn main() {
    let host_addr = SocketAddrV4::from_str(FROM_ADDR).unwrap();
    let to_addr = SocketAddrV4::from_str(TO_ADDR).unwrap();

    let mut se = bmd_speededitor::new().unwrap();
    let (key_tx, rx): (SyncSender<SpeedEditorEvent>, Receiver<SpeedEditorEvent>) = sync_channel(0);
    let jog_tx = key_tx.clone();

    let handle = thread::spawn(move || {
        se.on_key(move |key, down| {
            key_tx.send(SpeedEditorEvent::KeyEvent(key, down)).unwrap();
            Ok(())
        });
        se.on_jog(move |mode, value| {
            jog_tx
                .send(SpeedEditorEvent::JogEvent(mode, value))
                .unwrap();
            Ok(())
        });
        se.run().unwrap();
    });

    let key_socket = UdpSocket::bind(host_addr).unwrap();
    let jog_socket = key_socket.try_clone().unwrap();
    for e in rx {
        match e {
            SpeedEditorEvent::KeyEvent(key, down) => {
                send_osc(
                    &key_socket,
                    to_addr,
                    &key.to_owned().to_string(),
                    if down { 1.0 } else { 0.0 },
                );
            }
            SpeedEditorEvent::JogEvent(_, value) => {
                send_osc(
                    &jog_socket,
                    to_addr,
                    "jog".to_string().as_str(),
                    value as f32,
                );
            }
        }
    }

    handle.join().unwrap();
}

fn send_osc(socket: &UdpSocket, to_addr: SocketAddrV4, addr: &str, value: f32) {
    let message = OscMessage {
        addr: format!("/{}", addr),
        args: vec![OscType::Float(value)],
    };
    let buf = encoder::encode(&OscPacket::Message(message)).unwrap();
    socket.send_to(&buf, to_addr).unwrap();
}

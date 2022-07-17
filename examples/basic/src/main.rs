use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;

use bmd_speededitor::{self, Key};

pub enum SpeedEditorEvent {
    KeyEvent(Key, bool),
    JogEvent(u8, i32),
}

fn main() {
    let (key_tx, rx): (SyncSender<SpeedEditorEvent>, Receiver<SpeedEditorEvent>) = sync_channel(0);
    let jog_tx = key_tx.clone();

    let mut se = bmd_speededitor::new().unwrap();
    se.on_connected(|| {
        println!("Connected to the device");
        Ok(())
    });
    se.on_disconnected(|| {
        println!("Disconnected from the device");
        Ok(())
    });
    se.on_keys(|keys| {
        println!("current keys are: {:?}", keys);
        Ok(())
    });
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
    se.on_unknown(|data| {
        println!("unknown event: {:?}", data);
        Ok(())
    });

    let handle = thread::spawn(move || {
        se.run().unwrap();
    });

    for e in rx {
        match e {
            SpeedEditorEvent::KeyEvent(key, down) => {
                println!("key event: {} {}", key, down);
            }
            SpeedEditorEvent::JogEvent(mode, value) => {
                println!("jog: {} {}", mode, value);
            }
        }
    }

    handle.join().unwrap();
}

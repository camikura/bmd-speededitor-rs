use bmd_speededitor::{self};

fn main() {
    match bmd_speededitor::new() {
        Ok(mut se) => {
            se.connected_callback = || {
                println!("Connected to the device");
                Ok(())
            };
            se.disconnected_callback = || {
                println!("Disconnected from the device");
                Ok(())
            };
            se.keys_callback = |keys| {
                println!("current keys are: {:?}", keys);
                Ok(())
            };
            se.key_down_callback = |key| {
                println!("key down event: {}", key);
                Ok(())
            };
            se.key_up_callback = |key| {
                println!("key up event: {}", key);
                Ok(())
            };
            se.jog_callback = |value| {
                println!("jog event: {}", value);
                Ok(())
            };
            se.unknown_callback = |data| {
                println!("unknown event: {:?}", data);
                Ok(())
            };
            se.run().unwrap();
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}

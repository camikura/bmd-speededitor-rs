use bmd_speededitor::{self};

fn main() {
    match bmd_speededitor::new() {
        Ok(mut se) => {
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

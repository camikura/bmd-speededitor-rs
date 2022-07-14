use bmd_speededitor::{self, Key, KeyLed};

fn main() {
    match bmd_speededitor::new() {
        Ok(mut se) => {
            se.connected_handler = |se| {
                println!("Connected to the device");
                se.to_owned()
                    .set_leds(vec![KeyLed::CloseUp, KeyLed::Cut], true)?;
                Ok(())
            };
            se.disconnected_handler = || {
                println!("Disconnected from the device");
                Ok(())
            };
            se.keys_handler = |_, keys| {
                println!("current keys are: {:?}", keys);
                Ok(())
            };
            se.key_down_handler = |se, key| {
                println!("key down event: {}", key);
                match key {
                    Key::Cam1 => se.to_owned().set_key_led(KeyLed::Cam1, true)?,
                    Key::Cam2 => se.to_owned().set_all_key_leds(true)?,
                    _ => {}
                }
                Ok(())
            };
            se.key_up_handler = |se, key| {
                println!("key up event: {}", key);
                match key {
                    Key::Cam1 => se.to_owned().set_key_led(KeyLed::Cam1, false)?,
                    Key::Cam2 => se.to_owned().set_all_key_leds(false)?,
                    _ => {}
                }
                Ok(())
            };
            se.jog_handler = |_, mode, value| {
                println!("jog event: {} / {}", mode, value);
                Ok(())
            };
            se.unknown_handler = |_, data| {
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

use bmd_speededitor::{self, Key, KeyLed};

fn main() {
    match bmd_speededitor::new() {
        Ok(mut se) => {
            se.on_connected(|se| {
                se.to_owned()
                    .set_leds(vec![KeyLed::CloseUp, KeyLed::Cut], true)?;
                println!("Connected to the device");
                Ok(())
            });
            se.on_disconnected(|| {
                println!("Disconnected from the device");
                Ok(())
            });
            se.on_keys(|_, keys| {
                println!("current keys are: {:?}", keys);
                Ok(())
            });
            se.on_key_down(|se, key| {
                println!("key down event: {}", key);
                match key {
                    Key::Cam1 => se.to_owned().set_key_led(KeyLed::Cam1, true)?,
                    Key::Cam2 => se.to_owned().set_all_key_leds(true)?,
                    _ => {}
                }
                Ok(())
            });
            se.on_key_up(|se, key| {
                println!("key up event: {}", key);
                match key {
                    Key::Cam1 => se.to_owned().set_key_led(KeyLed::Cam1, false)?,
                    Key::Cam2 => se.to_owned().set_all_key_leds(false)?,
                    _ => {}
                }
                Ok(())
            });
            se.on_jog(|_, mode, value| {
                println!("jog event: {} / {}", mode, value);
                Ok(())
            });
            se.on_unknown(|_, data| {
                println!("unknown event: {:?}", data);
                Ok(())
            });
            se.run().unwrap();
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}

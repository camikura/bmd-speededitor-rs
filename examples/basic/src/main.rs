use bmd_speededitor;

fn main() {
    match bmd_speededitor::new() {
        Ok(se) => {
            se.run().unwrap();
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}

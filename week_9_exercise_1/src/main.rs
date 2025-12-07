use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    
    for i in 1..=3 {
        let sender = tx.clone();
        let prefix = format!("T{}:", i);

        thread::spawn(move || {
            for j in 1..=5 {
                let message = format!("{} message {}", prefix, j);
                sender.send(message).unwrap();

                
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    
    drop(tx);

    
    for received in rx {
        println!("{}", received);
    }
}

use std::fs::{OpenOptions, File};
use std::io::{self, Write, Read};
use std::time::SystemTime;

fn main() {
    loop {
        println!("\n--- Note Taking App ---");
        println!("1) Add a note");
        println!("2) View all notes");
        println!("3) Exit");
        print!("Choose an option: ");

        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_note(),
            "2" => show_notes(),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Try again."),
        }
    }
}

fn add_note() {
    print!("Enter your note: ");
    io::stdout().flush().unwrap();

    let mut note = String::new();
    io::stdin().read_line(&mut note).unwrap();

    let timestamp = get_timestamp();
    let entry = format!("[{}] {}\n", timestamp, note.trim());

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("notes.txt")
        .unwrap();

    file.write_all(entry.as_bytes()).unwrap();

    println!("Note saved!");
}

fn show_notes() {
    let mut file = match File::open("notes.txt") {
        Ok(file) => file,
        Err(_) => {
            println!("No notes found.");
            return;
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("\n--- Your Notes ---");
    print!("{}", contents);
}

fn get_timestamp() -> String {
    use std::time::{UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    now.to_string()
}

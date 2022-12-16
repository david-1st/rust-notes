fn main() {
    let mut notes = Vec::new();

    print_menu();

    loop {
        let command = read_input();

        match command.trim() {
            "show" => show_notes(&notes),
            "add" => add_notes(&mut notes),
            _ => break,
        }
    }
}

fn print_menu() {
    println!("Notes app");
    println!("show - show all notes");
    println!("add - add new note");
}

fn read_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

fn add_notes(notes: &mut Vec<String>) {
    println!("-----");
    println!("Enter note");

    let input = read_input();

    let note = input.trim().to_string();

    notes.push(note);
    println!("-----")
}

fn show_notes(verc: &Vec<String>) {
    println!("-----");
    for item in verc {
        println!("{}", item);
    }
    println!("-----")
}

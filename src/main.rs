use std::io::{self, BufRead, Write};

use utils::make_file;
pub mod utils;

struct Dust {
    file_name: String,
    content: String,
}
//template: https://github.com/Axzurey/electron-react-typescript-webpack-template

//MODIFY: package.json, 
struct CreateElectronApp {
    name: String,
    author: String,
    desc: String
}

fn create_electron_app() {
    let mut line = String::new();
    let stdin = io::stdin();

    let table = ["name", "author", "desc"];
    let queries = ["Set an application name", "Set the author", "Set a description"];
    let mut results: [String; 3] = [String::from(""), String::from(""), String::from("")];
    let mut pointer = 0;

    loop {
        print!("{}\t", queries[pointer]);
        io::stdout().flush().expect("An unexpected error occured!");
        stdin.lock().read_line(&mut line).expect("Unable to read line");

        let clone = line.clone();

        results[pointer] = clone;

        pointer += 1;

        if pointer == table.len() {break}
    }
}

fn dust_file() {
    let out = Dust {
        file_name: std::env::args().nth(2).expect("Argument 2 is missing (File Name)"),
        content: std::env::args().nth(3).unwrap_or("".to_string())
    };

    make_file(&out.file_name, &out.content).expect("Error creating file");
}

fn construct_param_from_cli() {
    let cmd = std::env::args().nth(1).expect("Argument 1 is missing (Command)");

    if cmd == String::from("dust") {
        dust_file();
    }
    else if cmd == String::from("create-electron-app") {
        create_electron_app();
    }
    else {
        panic!("{}", format!("{cmd} is not a valid command!"))
    }
}

fn main() {
    let _cli = construct_param_from_cli();
}
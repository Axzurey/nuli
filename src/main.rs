use std::io::{self, BufRead, Write, Cursor};

use utils::{make_file, clone_repo};
pub mod utils;

struct Dust {
    file_name: String,
    content: String,
}
//template: https://github.com/Axzurey/electron-react-typescript-webpack-template

struct CreateElectronApp {
    name: String,
    author: String,
    desc: String
}

fn create_electron_app(additional: Additional) {
    let stdin = io::stdin();

    let table = ["name", "author", "desc"];
    let queries = ["Set an application name", "Set the author", "Set a description"];
    let mut results: [String; 3] = ["".to_string(), "".to_string(), "".to_string()];
    let mut pointer = 0;

    loop {
        print!("{}\t", queries[pointer]);
        io::stdout().flush().expect("An unexpected error occured!");
        stdin.lock().read_line(&mut results[pointer]).expect("Unable to read line");

        pointer += 1;
        if pointer == table.len() {break}
    }

    let _res = clone_repo(&String::from("https://github.com/Axzurey/electron-react-typescript-webpack-template"), &results[0]);

    let file = std::fs::read_to_string(file!()); //https://docs.rs/tar/latest/tar/
}

fn dust_file(additional: Additional) {
    let out = Dust {
        file_name: std::env::args().nth(2).expect("Argument 2 is missing (File Name)"),
        content: std::env::args().nth(3).unwrap_or("".to_string())
    };

    make_file(&out.file_name, &out.content).expect("Error creating file");
}

fn construct_param_from_cli(additional: Additional) {
    let cmd = std::env::args().nth(1).expect("Argument 1 is missing (Command)");

    if cmd == String::from("dust") {
        dust_file(additional);
    }
    else if cmd == String::from("create-electron-app") {
        create_electron_app(additional);
    }
    else {
        panic!("{}", format!("{cmd} is not a valid command!"))
    }
}

struct Additional<'a> {
    electron: Cursor<&'a [u8; 5396992]>
}

fn main() {
    let electrapp = Cursor::new(include_bytes!("create-electron-app.tar"));
    let _cli = construct_param_from_cli(Additional {
        electron: electrapp
    });
}
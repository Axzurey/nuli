use std::io::{self, BufRead, Write, Cursor};

use utils::{make_file, unwrap_buffer};
pub mod utils;

//template: https://github.com/Axzurey/electron-react-typescript-webpack-template

fn create_electron_app(additional: Additional) {
    let stdin = io::stdin();

    let table = ["name"];
    let queries = ["Application Name"];
    let mut results: [String; 1] = ["".to_string()];
    let mut pointer = 0;

    loop {
        print!("{}\t", queries[pointer]);

        io::stdout().flush().expect("An unexpected error occured!");

        stdin.lock().read_line(&mut results[pointer]).expect("Unable to read line");

        pointer += 1;

        if pointer == table.len() {break}
    }

    unwrap_buffer(additional.electron, &results[0]);

    // in the future maybe make a flag to pull from github let _res = clone_repo(&String::from("https://github.com/Axzurey/electron-react-typescript-webpack-template"), &results[0]);
}

fn dust_file(_additional: Additional) {
    let file_name = std::env::args().nth(2).expect("Argument 2 is missing (File Name)");
    let content = std::env::args().nth(3).unwrap_or("".to_string());

    make_file(&file_name, &content).expect("Error creating file");
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
    electron: Cursor<&'a [u8; 5301760]>
}

fn main() {
    let electrapp = Cursor::new(include_bytes!("include/create-electron-app.tar"));
    let _cli = construct_param_from_cli(Additional {
        electron: electrapp
    });
}
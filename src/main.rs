use std::{fs::File, io::Write};

struct Dust {
    file_name: String,
    content: String,
}

fn construct_param_from_cli() -> Option<Dust> {
    let cmd = std::env::args().nth(1).expect("Argument 1 is missing (Command)");

    match cmd == String::from("dust") { //something wrong here
            true => {
                let out = Dust {
                    file_name: std::env::args().nth(2).expect("Argument 2 is missing (File Name)"),
                    content: std::env::args().nth(3).unwrap_or("".to_string())
                };

                let mut file = File::create(&out.file_name).expect("there was an error creating the file");

                let clone = out.content.clone();

                let bytes = clone.into_bytes();
               
                file.write_all(&bytes).expect("Unable to write buffer to file.");

                return Some(out);
        },
        _ => panic!("{cmd} is not a valid command!")
    }
}

fn main() {
    let _cli = construct_param_from_cli();
}
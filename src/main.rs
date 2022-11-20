use std::fs::File;

struct Mkfl {
    cmd: String,
    file_name: String,
    content: Option<String>,
}

fn construct_param_from_cli() -> Option<Mkfl> {
    let cmd = std::env::args().nth(1).expect("Argument 1 is missing (Command)");

    match cmd == String::from("mkfl") { //something wrong here
            true => {
                let out = Mkfl {
                    cmd: String::from("mkfl"),
                    file_name: std::env::args().nth(2).expect("Argument 2 is missing (File Name)"),
                    content: std::env::args().nth(3)
                };

                let _file = File::create(&out.file_name).expect("there was an error creating the file");
                
                

                print!("{}", out.cmd);
                print!("{}", out.file_name);
                print!("{}", out.content.as_ref().expect("[No Contents]"));

                return Some(out);
        },
        _ => panic!("{cmd} is not a valid command!")
    }
}

fn main() {
    let _cli = construct_param_from_cli();
}
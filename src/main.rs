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

fn construct_param_from_cli() {
    let cmd = std::env::args().nth(1).expect("Argument 1 is missing (Command)");

    if cmd == String::from("dust") {
        let out = Dust {
            file_name: std::env::args().nth(2).expect("Argument 2 is missing (File Name)"),
            content: std::env::args().nth(3).unwrap_or("".to_string())
        };

        make_file(&out.file_name, &out.content).expect("Error creating file");
    }
    else if cmd == String::from("create-electron-app") {
        todo!()
    }
    else {
        panic!("{}", format!("{cmd} is not a valid command!"))
    }
}

fn main() {
    let _cli = construct_param_from_cli();
}
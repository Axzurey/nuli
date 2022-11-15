
struct Mkfl {
    cmd: String,
    file_name: String,
    content: Option<String>,
}

fn construct_param_from_cli() -> Option<Mkfl> {
    let cmd = std::env::args().nth(0).expect("Argument 0 is missing (Command)");

    if cmd == String::from("mkfl") { //something wrong here
        let out = Mkfl {
            cmd: String::from("mkfl"),
            file_name: std::env::args().nth(1).expect("Argument 1 is missing (File Name)"),
            content: std::env::args().nth(2)
        };
        print!("{}", out.file_name);
        print!("{}", out.cmd);
        print!("{}", out.content.as_ref().expect("NO"));
        return Some(out);
    }
    else {
        print!("This is bad!")
    }
    return None;
}

fn main() {

    let _cli = construct_param_from_cli();
}
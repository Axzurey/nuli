
struct Mkfl {
    cmd: String,
    file_name: String,
    content: Option<String>,
}

fn construct_param_from_cli() -> Option<Mkfl> {
    let cmd = std::env::args().nth(0).expect("Argument 0 is missing (Command)");

    if cmd == "mkfl" {
        let out = Mkfl {
            cmd: String::from("mkfl"),
            file_name: std::env::args().nth(1).expect("Argument 1 is missing (File Name)"),
            content: std::env::args().nth(2)
        };
        print!("{}", out.file_name);
        print!("{}", out.cmd);
        print!("{}", out.content.expect("NO"));
        return Some(out);
    }
    return None;
}

fn main() {

    let cli = construct_param_from_cli();
}
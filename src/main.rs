mod init;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args[1].as_str() {
        "init" => init::init_main(&args[2..]),
        _ => {
            print_help();
            println!("Command not support!");
        }
    }
}

fn print_help() {
    println!(
        "\
    This is the help menu"
    );
}

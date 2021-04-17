use std::{env, fs, io, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut prog = String::new();
    let mut input = String::new();

    if args.len() > 1 {
        if args[1] == "-h" || args[1] == "--help" {
            println!(
                "Usage: {} <filename>
If a filename is not provided code is read from standard input
-h, --help: Shows this message",
                args[0]
            );
        } else {
            prog = fs::read_to_string(&args[1])
                .expect("Error. I didn't quite get that.\nNo such file");
        }
    } else {
        io::stdin()
            .read_line(&mut prog)
            .expect("Error. I didn't quite get that");
    }

    let input = if prog.contains(",") {
        eprintln!("Enter the input characters");
        io::stdin()
            .read_line(&mut input)
            .expect("Error. I didn't quite get that");
        Some(&input.trim()[..])
    } else { None };

    match rust_bf::run(&prog[..], input) {
        Ok(output)  => print!("{}", output),
        Err(err)    => { eprintln!("{}", err); process::exit(1) }
    }
}

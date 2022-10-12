use std::{
    env,
    fs::File,
    io::{self, stdout, Read, Write},
};

use smol_bf::ProgramBuilder;

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);
    let program = {
        let arg = args.next().expect("no input provided");
        if arg == "-i" {
            let input = args.next().expect("no file provided");
            let mut input = File::open(input)?;
            let mut read = String::with_capacity(
                input.metadata().map(|meta| meta.len()).unwrap_or_default() as usize,
            );
            input.read_to_string(&mut read)?;
            read
        } else {
            arg
        }
    };

    let input = args.next();
    let program = ProgramBuilder::new(&program, input.as_deref());
    let res = program.run();

    stdout().lock().write_all(res.as_bytes())?;

    Ok(())
}

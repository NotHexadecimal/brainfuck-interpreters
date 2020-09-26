use std::io;
fn main() {
    let mut program = String::new();
    io::stdin().read_line(&mut program).expect("Can't read");
    let mut mem: Vec<u8> = vec![0];
    let mut loopi: Vec<usize> = Vec::new();
    let mut brackets: usize = 0;
    let mut i: usize = 0;
    let mut p: usize = 0;
    let bytes = program.as_bytes();
    while p < bytes.len() {
        let b = bytes[p];
        match b {
            b'>' => if i < mem.len() - 1 { i += 1 } else { mem.push(0); i += 1 },
            b'<' => if i > 0 { i -= 1 } else { i = mem.len() - 1 },
            b'+' => mem[i] = if mem[i] == 255 { 0 } else { mem[i] + 1 },
            b'-' => mem[i] = if mem[i] == 0 { 255 } else { mem[i] - 1 },
            b'.' => print!("{}", mem[i] as char),
            b',' => mem[i] = {
                let mut read = String::new();
                io::stdin().read_line(&mut read).expect("Can't read");
                read[..].as_bytes()[0]
            },
            b'[' => if mem[i] == 0 {
                while p < bytes.len() {
                    match bytes[p] {
                        b']' => { p += 1; if brackets == 0 { break } },
                        b'[' => { p += 1; brackets += 1 }
                        _    => p += 1
                    }
                }
            } else { loopi.push(p) }
            b']' => if mem[i] != 0 {
                if let Some(o) = loopi.pop() { p = o - 1
                } else { panic!("No matching bracket found") }
            } 
            _    => ()
        }
    p += 1;
    }
    println!("\nMemory: {:?}", mem);
}

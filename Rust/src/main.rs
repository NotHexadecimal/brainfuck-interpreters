use std::{
    collections::HashMap,
    env, fs,
    io::{self, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut prog = String::new();
    if args.len() > 1 {
        if args[1] == "-h" || args[1] == "--help" {
            println!(
                "Usage: {} <filename>
If filename is not provided code is read from standard input
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
    let bytes = prog.as_bytes();
    let loops = maploops(bytes);
    run(bytes, loops)
}

fn maploops(bytes: &[u8]) -> HashMap<usize, usize> {
    //Do a first pass on the program, adds every ['s position to a LIFO queue, pop from the vector and
    //add to a hashmap every time a ] is found.
    let mut map = HashMap::new();
    let mut open: Vec<usize> = Vec::new();
    let mut i: usize = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'[' => open.push(i),
            b']' => {
                let last = if let Some(last) = open.pop() {
                    last
                } else {
                    panic!(
                        "Error. I didn't quite get that.
Unmatched bracket at {}",
                        i
                    )
                };
                map.insert(last, i);
                map.insert(i, last);
            }
            _ => (),
        }
        i += 1
    }
    if open.len() != 0 {
        panic!(
            "Error. I didn't quite get that.
Unmatched bracket at {}",
            open.pop().unwrap()
        )
    }
    map
}

fn run(bytes: &[u8], map: HashMap<usize, usize>) {
    let mut mem: [u8; 30000] = [0; 30000];
    //30000 seems like the standard memory size, expand as needed.
    let mut i: usize = 0;
    let mut p: usize = 0;
    while p < bytes.len() {
        match bytes[p] {
            b'>' => i += 1,
            b'<' => i -= 1,
            b'+' => {
                if mem[i] == 255 {
                    mem[i] = 0
                } else {
                    mem[i] += 1
                }
            }
            b'-' => {
                if mem[i] == 0 {
                    mem[i] = 255
                } else {
                    mem[i] -= 1
                }
            }
            b'.' => {
                print!("{}", mem[i] as char);
                io::stdout().flush().unwrap()
            }
            //The flush make the write slower, but otherwise it won't show characters until a newline
            //is printed. Remove for better performance.
            b',' => {
                mem[i] = {
                    let mut buffer = String::new();
                    io::stdin()
                        .read_line(&mut buffer)
                        .expect("Error. I didn't quite get that");
                    buffer.as_bytes()[0]
                }
            }
            b'[' => {
                if mem[i] == 0 {
                    p = map[&p]
                }
            }
            b']' => {
                if mem[i] != 0 {
                    p = map[&p]
                }
            }
            _ => (),
        }
        p += 1
    }
    println!("")
}

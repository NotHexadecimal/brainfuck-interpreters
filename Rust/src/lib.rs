use std::{collections::HashMap,num::Wrapping};
fn maploops(bytes: &[u8]) -> Result<HashMap<usize, usize>, String> {
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
                    return Err(format!(
                        "Error. I didn't quite get that.\nUnmatched bracket at {}",
                        i
                    ));
                };
                map.insert(last, i);
                map.insert(i, last);
            }
            _ => (),
        }
        i += 1
    }
    if open.len() != 0 {
        return Err(format!(
            "Error. I didn't quite get that.\nUnmatched bracket at {}",
            open.pop().unwrap()
        ));
    }
    Ok(map)
}

fn exec(bytes: &[u8], map: HashMap<usize, usize>, input: Option<&str>) -> Result<String, String> {
    let mut mem: [Wrapping<u8>; 30000] = [Wrapping(0); 30000];
    //30000 seems like the standard memory size, expand as needed.
    let mut i: usize = 0; //position in memory
    let mut p: usize = 0; //position in program
    let mut b: usize = 0; //position in input buffer
    let mut output = String::new();
    let input = if let Some(a) = input {a} else {""};
    while p < bytes.len() {
        match bytes[p] {
            b'>' => {
                if i != 30000 {
                    i += 1
                } else {
                    return Err(String::from(
                        "Error, I didn't quite get that.\nOut of memory bounds",
                    ));
                }
            }
            b'<' => {
                if i != 0 {
                    i -= 1
                } else {
                    return Err(String::from(
                        "Error, I didn't quite get that.\nOut of memory bounds",
                    ));
                }
            }
            b'+' => {
                mem[i] += Wrapping(1)
            }
            b'-' => {
                mem[i] -= Wrapping(1)
            }
            b'.' => {
                output.push(mem[i].0 as char);
            }
            b',' => {
                mem[i] = {
                    b += 1;
                    if let Some(char) = input.as_bytes().get(b - 1) {
                        Wrapping(*char)
                    } else { return Err(String::from("Error, I didn't quite get that.\nInput too short."))}
                }
            }
            b'[' => {
                if mem[i].0 == 0 {
                    p = map[&p]
                }
            }
            b']' => {
                if mem[i].0 != 0 {
                    p = map[&p]
                }
            }
            _ => (),
        }
        p += 1
    }
    Ok(output)
}

pub fn run(program: &str, input: Option<&str>) -> Result<String, String> {
    let bytes = program.as_bytes();
    let loops = maploops(bytes)?;
    let output = exec(bytes, loops, input)?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_out() {
        assert_eq!(run(",.", Some("a")).unwrap(), String::from("a"));
    }

    #[test]
    fn loop_math() {
        assert_eq!(run("+++++[>++++++++++<-]>-.", None).unwrap(), String::from("1"));
    }

    #[test]
    #[should_panic]
    fn out_of_memory() {
        run("<", None).unwrap();
    }

    #[test]
    #[should_panic]
    fn out_of_input() {
        run(",", None).unwrap();
    }
}

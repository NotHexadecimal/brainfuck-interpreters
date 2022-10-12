use opt::*;
use std::{num::Wrapping, process::exit};

mod opt;

#[cfg(feature = "jit")]
mod jit;

#[cfg(test)]
mod tests;

/// Takes a brainfuck program and prepares it for execution
pub struct ProgramBuilder<'a> {
    input: Option<&'a str>,
    code: &'a str,
}

impl<'a> ProgramBuilder<'a> {
    /// Create a new program builder
    pub fn new(code: &'a str, input: Option<&'a str>) -> Self {
        Self { input, code }
    }

    /// Runs the program
    pub fn run(self) -> String {
        let mapped = self.code.chars().filter_map(char_to_ir);
        let grouped = group_insts(mapped);

        let loops = find_loops(&grouped);
        let grouped = optimize_loops(&grouped, &loops);

        #[cfg(feature = "jit")]
        return jit::compile_and_run(&grouped, self.input.unwrap_or_default());

        #[cfg(not(feature = "jit"))]
        {
            let mut grouped = grouped;
            let loops = find_loops(&grouped);

            for (start, end) in loops {
                grouped[start] = Instruction::LoopStart(end as u32);
                grouped[end] = Instruction::LoopEnd(start as u32);
            }

            interpret_insts(&grouped, self.input.unwrap_or_default())
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Instruction {
    Add(Wrapping<u8>),
    Move(isize),
    LoopStart(u32),
    LoopEnd(u32),
    Read,
    Write,
    Clear,
    SeekLeft,
    SeekRight
}

fn char_to_ir(c: char) -> Option<Instruction> {
    Some(match c {
        '+' => Instruction::Add(Wrapping(1)),
        '-' => Instruction::Add(Wrapping(255)),
        '<' => Instruction::Move(-1),
        '>' => Instruction::Move(1),
        '[' => Instruction::LoopStart(0),
        ']' => Instruction::LoopEnd(0),
        '.' => Instruction::Write,
        ',' => Instruction::Read,
        _ => return None,
    })
}

fn find_loops(insts: &[Instruction]) -> Vec<(usize, usize)> {
    let mut stack = vec![];
    let mut out = vec![];

    for (pos, inst) in insts.iter().enumerate() {
        match inst {
            Instruction::LoopStart(_) => stack.push(pos),
            Instruction::LoopEnd(_) => out.push((stack.pop().unwrap(), pos)),
            _ => (),
        };
    }

    if !stack.is_empty() {
        eprintln!("{count} unmatched brackets", count = stack.len());
        exit(1)
    }

    out
}

#[cfg(not(feature = "jit"))]
fn interpret_insts(code: &[Instruction], input: &str) -> String {
    let mut code_ptr = 0;
    let mut mem = [Wrapping(0); u16::MAX as usize];
    let mut ptr = Wrapping(0u16);
    let mut output = Vec::new();
    let mut input = input.as_bytes().iter();

    while code_ptr < code.len() {
        match code[code_ptr] {
            Instruction::Add(n) => mem[ptr.0 as usize] += n,
            Instruction::Move(n) => ptr += Wrapping(n as u16),
            Instruction::Write => output.push(mem[ptr.0 as usize].0),
            Instruction::Read => mem[ptr.0 as usize] = Wrapping(*input.next().unwrap_or(&0)),
            Instruction::LoopStart(jump) => {
                if mem[ptr.0 as usize].0 == 0 {
                    code_ptr = jump as usize - 1
                }
            }
            Instruction::LoopEnd(jump) => {
                if mem[ptr.0 as usize].0 != 0 {
                    code_ptr = jump as usize - 1
                }
            }
            Instruction::Clear => mem[ptr.0 as usize] = Wrapping(0),
            Instruction::SeekLeft => while mem[ptr.0 as usize].0 != 0 { ptr -= Wrapping(1) }
            Instruction::SeekRight => while mem[ptr.0 as usize].0 != 0 { ptr += Wrapping(1) }
        }

        code_ptr += 1;
    }

    String::from_utf8_lossy(&output).to_string()
}

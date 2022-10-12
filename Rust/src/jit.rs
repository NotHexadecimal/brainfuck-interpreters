use std::str::Bytes;

use lightning_sys::{Jit, JitPointer, JitState, Reg};

use super::*;

type JitMem = [u8; 65536];
type JitFn = unsafe extern "C" fn(*mut JitMem, *mut JitIO);

// Don't want to deal with storing registers on the stack, so i keep a pointer to this and call
// methods from wrapper functions
struct JitIO<'a> {
    input: Bytes<'a>,
    output: String,
}

impl<'a> JitIO<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input: input.bytes(),
            output: String::new(),
        }
    }

    fn read(&mut self) -> u8 {
        self.input.next().unwrap_or(0)
    }

    fn write(&mut self, ch: u8) {
        self.output.push(ch as char)
    }
}

// This was supposed to return a JitFn but for some reason it segfaulted when I called it.
// So now it's called here.
pub(crate) fn compile_and_run(code: &[Instruction], input: &str) -> String {
    let mut jit = Jit::new();
    let mut state = jit.new_state();

    function_prolog(&mut state);
    compile_instructions(&mut state, code);
    function_epilog(&mut state);

    let fn_ptr = unsafe { state.cast_emit::<JitFn>() };
    state.clear_state();

    let mut io = JitIO::new(input);
    let mut mem = [0u8; u16::MAX as usize];

    // Here be dragons
    unsafe { fn_ptr(mem.as_mut_ptr() as *mut JitMem, &mut io as *mut JitIO) };
    #[cfg(test)]
    println!("{:?}", &mem[..10]);

    io.output
}

fn function_prolog(state: &mut JitState) {
    // Register layout
    // V0: memory pointer
    // V1: memory position
    // V2: output string pointer
    state.prolog();
    // Store memory pointer in register V0;
    let mptr = state.arg();
    state.getarg(Reg::V(0), &mptr);
    // Clear memory offset
    state.xorr(Reg::V(1), Reg::V(1), Reg::V(1));
    // Store output pointer
    let optr = state.arg();
    state.getarg(Reg::V(2), &optr);
}

fn function_epilog(state: &mut JitState) {
    // return absolutely nothing
    state.ret();
    // done
    state.epilog();
}

fn compile_instructions(state: &mut JitState, code: &[Instruction]) {
    // easist way to call rust code, i think
    extern "C" fn jitio_write(target: &mut JitIO, val: u8) {
        target.write(val);
    }

    extern "C" fn jitio_read(target: &mut JitIO) -> u8 {
        target.read()
    }

    // store branch positions to patch later
    let mut jump_stack = Vec::new();

    for inst in code {
        match inst {
            Instruction::Move(n) => {
                // increment pointer
                // v1 += n
                state.addi(Reg::V(1), Reg::V(1), *n as i64);
                // prevent OOB
                // v1 &= mem.len()
                state.andi(Reg::V(1), Reg::V(1), u16::MAX as i64);
            }
            Instruction::Add(n) => {
                // r0 = *(mptr + offset)
                state.ldxr_uc(Reg::R(0), Reg::V(0), Reg::V(1));
                // r0 = r0 + n
                state.addi(Reg::R(0), Reg::R(0), n.0 as i64);
                // *(mptr + offset) = r0
                state.stxr_c(Reg::V(0), Reg::V(1), Reg::R(0));
            }
            Instruction::Clear => {
                // Set R0 to zero
                state.xorr(Reg::R(0), Reg::R(0), Reg::R(0));
                // *(mptr + offset) = 0
                state.stxr_c(Reg::V(0), Reg::V(1), Reg::R(0));
            }
            Instruction::Read => {
                state.prepare();
                // pointer to io wrapper
                state.pushargr(Reg::V(2));
                state.finishi(jitio_read as JitPointer);
                // fetch the return value
                state.retval_uc(Reg::R(0));
                // store at current memory position
                state.stxr_c(Reg::V(0), Reg::V(1), Reg::R(0));
            }
            Instruction::Write => {
                state.prepare();
                // pointer to io wrapper
                state.pushargr(Reg::V(2));
                // load mem pointer
                state.ldxr_uc(Reg::R(0), Reg::V(0), Reg::V(1));
                // character
                state.pushargr(Reg::R(0));
                state.finishi(jitio_write as JitPointer);
            }
            Instruction::LoopStart(_) => {
                // r0 = *(mptr + offset)
                let start = state.label();
                state.ldxr_uc(Reg::R(0), Reg::V(0), Reg::V(1));
                jump_stack.push((
                    // loop start
                    start,
                    // if r0 == 0 { jmp(end) }
                    // store node to patch in the end
                    state.beqi(Reg::R(0), 0),
                ));
            }
            Instruction::LoopEnd(_) => {
                let (start, end) = jump_stack.pop().unwrap();
                // everything hangs without this so i'll leave it
                state.ldxr_uc(Reg::R(0), Reg::V(0), Reg::V(1));

                // if r0 != 0
                let jump = state.jmpi();
                state.patch_at(&jump, &start);
                state.patch(&end);
            }
            Instruction::SeekRight => {
                // r0 = *(mptr + offset)
                state.ldxr_uc(Reg::R(0), Reg::V(0), Reg::V(1));

                let before = state.label();
                // if r0 == 0 skip ahead
                let branch = state.beqi(Reg::R(0), 0);
                // increment pointer
                state.addi(Reg::V(1), Reg::V(1), 1);
                // prevent OOB
                state.andi(Reg::V(1), Reg::V(1), u16::MAX as i64);
                // load
                state.ldxr_uc(Reg::R(0), Reg::V(0), Reg::V(1));

                let go_back = state.jmpi();
                state.patch_at(&go_back, &before);

                state.patch(&branch);
            }
            Instruction::SeekLeft => {
                state.ldxr_uc(Reg::R(0), Reg::V(0), Reg::V(1));

                let before = state.label();
                let branch = state.beqi(Reg::R(0), 0);
                // decrement pointer
                state.subi(Reg::V(1), Reg::V(1), 1);
                state.andi(Reg::V(1), Reg::V(1), u16::MAX as i64);
                state.ldxr_uc(Reg::R(0), Reg::V(0), Reg::V(1));

                let go_back = state.jmpi();
                state.patch_at(&go_back, &before);

                state.patch(&branch);
            }
        }
    }
}

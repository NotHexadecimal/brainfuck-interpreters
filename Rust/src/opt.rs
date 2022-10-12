use super::Instruction;

pub(crate) fn group_insts<I: Iterator<Item = Instruction>>(iter: I) -> Vec<Instruction> {
    let mut out = Vec::with_capacity(iter.size_hint().0);
    let mut prev = None;

    let mut matcher = |next| match (prev, next) {
        (Some(Instruction::Add(old)), Instruction::Add(new)) => {
            prev = Some(Instruction::Add(old + new))
        }
        (Some(Instruction::Move(old)), Instruction::Move(new)) => {
            prev = Some(Instruction::Move(old + new))
        }
        (Some(old), new) => {
            out.push(old);
            prev = Some(new)
        }
        (None, new) => prev = Some(new),
    };

    for inst in iter {
        matcher(inst)
    }

    if let Some(p) = prev { out.push(p) }

    out
}

fn analyze_loop(expr: &[Instruction]) -> Option<Instruction> {
    match expr {
        [Instruction::Add(_)] => Some(Instruction::Clear),
        [Instruction::Move(-1)] => Some(Instruction::SeekLeft),
        [Instruction::Move(1)] => Some(Instruction::SeekRight),
        _ => None
    }
}

pub(crate) fn optimize_loops(insts: &[Instruction], loops: &[(usize, usize)]) -> Vec<Instruction> {
    let mut output = Vec::with_capacity(insts.len());

    let mut optimized: Vec<_> = loops.iter()
        .map(|(start, end)| (*start, *end, &insts[start + 1..*end]))
        .filter_map(|(s, e, l)| analyze_loop(l).map(|l| (s, e, l)))
        .collect();
    optimized.sort_unstable_by_key(|(s, _, _)| *s);

    let mut index = 0;
    for (start, end, inst) in optimized {
        output.extend_from_slice(&insts[index..start]);
        output.push(inst);
        index = end + 1;
    }
    if index < insts.len() { output.extend_from_slice(&insts[index..]) }

    output
}

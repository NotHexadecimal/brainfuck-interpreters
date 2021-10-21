"""This is a supposedly simple python interpreter for brainfuck, \
but stuff got out of hand really quickly"""

import sys
from typing import Iterable, Iterator, Dict, Tuple


def brackets_iter(code: Iterable[str]) -> Iterator[Tuple[int, int]]:
    """Iterate on the start/end indexes of loops"""
    brackets_stack = []
    for index, instruction in enumerate(code):
        if instruction == '[':
            brackets_stack.append(index)
        elif instruction == ']':
            start = brackets_stack.pop()
            if start is not None:
                yield (start, index)
            else:
                raise Exception("Unmatched ']' at position {0}".format(index))
    if len(brackets_stack) != 0:
        positions = ", ".join([str(c) for c in brackets_stack])
        raise Exception("Unmatched '[' at position(s) {0}".format(positions))


def check_brackets(code: Iterable[str]):
    try:
        for _ in brackets_iter(code):
            pass
    except Exception as e:
        print(f"Error while parsing program: {e}")


def build_brackets_map(code: Iterable[str]) -> Dict[int, int]:
    """Collect bracket indexes into a jump table"""
    brackets = {}
    for start, end in brackets_iter(code):
        brackets[start] = end
        brackets[end] = start
    return brackets


def group_intructions(code: Iterator[str]) -> Iterator[Tuple[str, int]]:
    """Takes an iterator of istructions and groups together adjacent ones \
    when possible"""
    flip = {
        '+': '-',
        '-': '+',
        '<': '>',
        '>': '<',
    }

    count = 1
    prev = next(code)

    def to_yield():
        if prev in ('.', ',', '[', ']'):
            return prev, 0
        if count >= 0:
            return prev, count
        return flip[prev], abs(count)

    for cur in code:
        if (cur, prev) in (('+', '+'), ('-', '-'), ('<', '<'), ('>', '>')):
            count += 1
        elif (cur, prev) in (('+', '-'), ('-', '+'), ('<', '>'), ('>', '<')):
            count -= 1
        else:
            ret = to_yield()
            prev = cur
            count = 1
            yield ret
    else:
        yield to_yield()


def run(code: str, text_in=''):
    """Run the code"""

    check_brackets(code)
    filtered = [i for i in code if i in ('+', '-', '<', '>', '[', ']', ',', '.')]
    grouped = list(group_intructions(iter(filtered)))
    for start, end in brackets_iter([i for i, _ in grouped]):
        i, _ = grouped[start]
        grouped[start] = i, end - 1
        i, _ = grouped[end]
        grouped[end] = i, start - 1

    memory = [0] * 30000
    program_pointer = 0
    memory_pointer = 0
    input_pointer = 0

    icount = len(grouped)
    while program_pointer < icount:
        instruction, data = grouped[program_pointer]
        if instruction == '+':
            memory[memory_pointer] = (memory[memory_pointer] + data) % 256
        elif instruction == '-':
            memory[memory_pointer] = (memory[memory_pointer] - data) % 256
        elif instruction == '>':
            memory_pointer += data
        elif instruction == '<':
            memory_pointer -= data
        elif instruction == '[':
            if not memory[memory_pointer]:
                program_pointer = data
        elif instruction == ']':
            if memory[memory_pointer]:
                program_pointer = data
        elif instruction == '.':
            print(chr(memory[memory_pointer]), end='')
        elif instruction == ',':
            if input_pointer < len(text_in):
                memory[memory_pointer] = ord(text_in[input_pointer])
                input_pointer += 1
            else:
                memory[memory_pointer] = 0
        program_pointer += 1


if __name__ == '__main__':
    args = sys.argv
    if len(args) == 1:
        print('Specify a file containing the code to run. Exiting.')
    elif len(args) == 2:
        code = None
        with open(args[1]) as f:
            code = f.read()
        run(code)
    elif len(args) == 3:
        code = None
        text_in = None
        with open(args[1]) as f:
            code = f.read()
        with open(args[2]) as f:
            text_in = f.read()
        run(code, text_in)
    else:
        print('Too many arguments. Exiting.')

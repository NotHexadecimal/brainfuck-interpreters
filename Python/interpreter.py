import sys


def brackets_iter(code):
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
        positions = ", ".join(brackets_stack)
        raise Exception("Unmatched '[' at position(s) {0}".format(positions))


def build_brackets_map(code):
    brackets = {}
    for start, end in brackets_iter(code):
        brackets[start] = end
        brackets[end] = start
    return brackets


def run(code, text_in=''):
    brackets_map = build_brackets_map(code)
    memory = [0] * 30000
    program_pointer = 0
    memory_pointer = 0
    input_pointer = 0

    while True:
        instruction = code[program_pointer]
        if instruction == '+':
            memory[memory_pointer] = (memory[memory_pointer] + 1) % 256
        elif instruction == '-':
            memory[memory_pointer] = (memory[memory_pointer] - 1) % 256
        elif instruction == '>':
            memory_pointer += 1
        elif instruction == '<':
            memory_pointer -= 1
        elif instruction == '[':
            if not memory[memory_pointer]:
                program_pointer = brackets_map[program_pointer]
        elif instruction == ']':
            if memory[memory_pointer]:
                program_pointer = brackets_map[program_pointer]
        elif instruction == '.':
            print(chr(memory[memory_pointer]), end='')
        elif instruction == ',':
            if input_pointer < len(text_in):
                memory[memory_pointer] = ord(text_in[input_pointer])
                input_pointer += 1
            else:
                memory[memory_pointer] = 0
        program_pointer += 1
        if program_pointer == len(code):
            break


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

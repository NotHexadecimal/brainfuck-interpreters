import sys

def build_brackets_map(code):
  brackets_map = {}
  for index, instruction in enumerate(code):
    if instruction == '[':
      depth = 0
      for other_index, other_instruction in enumerate(code[index + 1:]):
        if other_instruction == '[':
          depth += 1
        elif other_instruction == ']':
          if depth:
            depth -= 1
          else:
            brackets_map[index] = other_index + index + 1
            brackets_map[other_index + index + 1] = index
            break
  return brackets_map

def run(code, text_in = ''):
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

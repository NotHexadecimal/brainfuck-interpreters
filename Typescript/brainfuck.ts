interface Array<T> {
    fill(value: T): Array<T>;
}

const buildParensMap = (program: string): Record<number, number> => {
  const parensMap: Record<number, number> = {};
  for (let i = 0; i < program.length; i++) {
    if (program[i] === '[') {
      let done = false;
      let parensCount = 1;
      for (let j = i + 1; j < program.length; j++) {
        if (program[j] === '[') {
          parensCount++;
        } else if (program[j] === ']') {
          if (!--parensCount) {
            parensMap[j] = i;
            parensMap[i] = j;
            done = true;
            break;
          }
        }
      }
      if (!done) {
        throw new Error('invalid codez');
      }
    }
  }
  return parensMap;
}

const modulo256 = (n: number): number => ((n % 256) + 256) % 256

const brainfuck = (program: string, input: string = ''): string => {
  let out = '';
  const parensMap = buildParensMap(program);
  let memoryPointer = 0;
  let programCounter = 0;
  let inputPointer = 0;
  const memory: number[] = new Array(4096).fill(0);
  while (true) {
    switch (program[programCounter]) {
      case '+':
        memory[memoryPointer] = modulo256(++memory[memoryPointer]);
        break;
      case '-':
        memory[memoryPointer] = modulo256(--memory[memoryPointer]);
        break;
      case '>':
        memoryPointer++;
        break;
      case '<':
        memoryPointer--;
        break;
      case '[':
        if (!memory[memoryPointer]) {
          programCounter = parensMap[programCounter];
        }
        break;
      case ']':
        if (memory[memoryPointer]) {
          programCounter = parensMap[programCounter];
        }
        break;
      case '.':
        out += String.fromCharCode(memory[memoryPointer]);
        break;
      case ',':
        if (inputPointer < input.length) {
          memory[memoryPointer] = input.charCodeAt(inputPointer);
          inputPointer++;
        } else {
          memory[memoryPointer] = 0;
        }
        break;
    }
    programCounter++;
    if (programCounter > program.length) {
      return out;
    }
  }
}

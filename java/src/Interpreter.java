// created by rampeo mattone

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

public class Interpreter {
    private final char[] memory = new char[30_000]; // 30kBytes of memory as per specifications
    private int memptr = 0; // memory pointer
    private int pc = 0; // program counter
    private final byte[] instructions; // instructions in brainfuck will be stored here

    // '>' operator (memory is circular)
    private void shiftRight() {
        memptr = memptr == 29_999 ? 0 : memptr + 1 ;
    }

    // '<' operator (memory is circular)
    private void shiftLeft() {
        memptr = memptr == 0 ? 29_999 : memptr - 1 ;
    }

    // '+' operator (memory cells are byte sized)
    private void addOne() {
        memory[memptr] = (char) ((memory[memptr] + 1) % 256);
    }

    // '-' operator (memory cells are byte sized)
    private void subOne() {
        memory[memptr] = (char) ((memory[memptr] - 1) % 256);
    }

    // '.' operator (output is most likely buffered)
    private void print() {
        System.out.print(memory[memptr]);
    }

    // ',' operator (output is unbuffered)
    private void read() throws IOException {
        memory[memptr] = (char) System.in.read();
    }

    // '[' operator
    private void jumpForward() {
        if (memory[memptr] == 0) {
            int to_close = 1;
            do {
                pc++;
                if (instructions[pc] == ']') to_close--;
                else if (instructions[pc] == '[') to_close++;
            } while (to_close > 0);
        }
    }

    // ']' operator
    private void jumpBackward() {
        if (memory[memptr] != 0) {
            int to_open = 1;
            do {
                pc--;
                if (instructions[pc] == ']') to_open++;
                else if (instructions[pc] == '[') to_open--;
            } while (to_open > 0);
        }
    }

    private void run() throws IOException {
        while (pc < instructions.length) {
            switch (instructions[pc]) {
                case '+': addOne(); break;
                case '-': subOne(); break;
                case '>': shiftRight(); break;
                case '<': shiftLeft(); break;
                case '.': print(); break;
                case ',': read(); break;
                case '[': jumpForward(); break;
                case ']': jumpBackward(); break;
            }
            pc++;
        }
    }

    Interpreter(String path) throws IOException {
        instructions = Files.readAllBytes(Path.of(path));
    }

    public static void main(String[] args) throws IOException {
        Interpreter interpreter = new Interpreter(args[0]);
        interpreter.run();
    }
}

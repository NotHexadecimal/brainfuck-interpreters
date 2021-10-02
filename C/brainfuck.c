//
// Created by Rampeo Mattone on 10/3/21.
//

#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *args[]) {
    /*
     * the code is read as follows:
     * + and -   # increase or decrease the value stored in the active memory cell
     * > and <   # move the active memory cell pointer forwards or backwards
     * , and .   # wait for input and save to active memory cell or output the active memory cell to stdout
     * [ and ]   # continue executing the code bounded by the brackets until the active memory cell reads 0
     *
     * the interpreter will allocate 30kBytes of memory on the stack
     * it will also read the instructions directly from a file, instead of from stdin
     * (this is so that I don't have to deal with array reallocation as much as possible)
     */
    // just check to make sure our user is not an idiot
    if (argc != 2) {
        printf("you must provide the path to some code you want to execute as an argument");
        return -1;
    }
    FILE *file = fopen(args[1], "r");   // let's assume the user is capable of typing a proper path
    fseek(file, 0, SEEK_END);
    long size = ftell(file);
    rewind(file);
    char *code = malloc(size);  // allocate enough memory to store the whole code in memory
    int _trash = fread(code, sizeof(char), size, file);  // read the instructions into memory
    fclose(file);
    char memory[30000] = {0};   // memory cells are on the stack
    int ptr = 0;    // this is the active cell pointer
    // main cycle of interpretation
    int instruction = 0;    // index of the instruction in 'code'
    do {
        switch (code[instruction]) {
            case '+':
                memory[ptr]++;
                break;
            case '-':
                memory[ptr]--;
                break;
            case '>':
                ptr++;
                break;
            case '<':
                ptr--;
                break;
            case ',':
                memory[ptr] = getchar();
                break;
            case '.':
                fputc(memory[ptr], stdout);
                fflush(stdout);
                break;
            case '[':
                if (memory[ptr] == 0) {
                    int to_close = 1;
                    do {
                        switch (code[++instruction]) {
                            case '[':
                                to_close++;
                                break;
                            case ']':
                                to_close--;
                                break;
                        }
                    } while (to_close != 0);
                }
                break;
            case ']':
                if (memory[ptr] != 0) {
                    int to_close = 1;
                    do {
                        switch (code[--instruction]) {
                            case '[':
                                to_close--;
                                break;
                            case ']':
                                to_close++;
                                break;
                        }
                    } while (to_close != 0);
                }
                break;
        }
        instruction++;
    } while (instruction < size);
    free(code);
    fputc('\n', stdout);
    return 0;
}

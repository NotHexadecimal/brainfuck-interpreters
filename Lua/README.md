# This is a simple Lua interpreter for brainfuck.

I have tested it using Lua5.4 (both work, **luajit is not supported**).

### Both interpreter.lua and transpiler.lua work like this:
You can call the script with a path to a brainfuck file as an optional parameter. If this argument is omitted the script will prompt you to input the brainfuckery through stdin.

### Only available if using the transpiler.lua script:
The script also adds the ability to save the translate brainfuck code as a lua chunk.

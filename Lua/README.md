# This is a simple Lua interpreter and transpiler for brainfuck.

I have tested it using Lua5.4 and LuaJit.

### transpiler.lua works like this:
You can call the script with a path to a brainfuck file as an optional parameter.  
If this argument is omitted the script will prompt you to input the brainfuckery through stdin.  
The script also adds the ability to save the translate brainfuck code as a lua chunk.

### interpreter.lua works like this:
You can call the script with a path to a brainfuck file.  
If you want the output to be unbuffered, than you must add 1 as a second optional parameter (omitting this will let the output be unbuffered)

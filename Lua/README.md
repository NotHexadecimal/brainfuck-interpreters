##### This is a simple Lua interpreter for brainfuck.

I have tested it using both Lua5.4 and Luajit (both work, **luajit is recommended**).

You can call the script with a path to a brainfuck file as an optional parameter.If this argument is omitted the script will prompt you to input the brainfuckery through stdin.

The script also adds the ability to save the translate brainfuck code as a lua chunk.

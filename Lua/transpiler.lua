---
--- Created by Rampeo Mattone (aka Nylon)
--- DateTime: 2020-09-18 23:22
--- Last updated: 2020-09-20 15:04
---

local interpreter = require("deps/Transpile") -- import the interpreter

local function read_brainfuckery() --reads the whole code and passes it to the caller as a string
    if arg[1] then
        local file = io.open(arg[1])
        if file then return file:read("*a")
        else
            io.write(string.format("File %s not found", arg[1]))
            os.exit(false)
        end
    else
        io.write("\nType the code you want to execute. input \"__stop__\" on a newline to finish:\n")
        local buffer = {}
        repeat
            local l = io.read()
            table.insert(buffer, l)
        until l == "__stop__"
        print("Stopped reading from the buffer!")
        return table.concat(buffer)
    end
end

local code = interpreter(read_brainfuckery())
local prog = load(code)
io.write("\nTranslation to a lua chunk has terminated. Would you like to run the code? (Y/n) -> ")
if string.lower(io.read()) ~= "n" then
    print("====================== S T A R T ======================")
    local status, error = pcall(prog)
    if not status then io.write(string.format("\nExecution failed with error: %s\n", error)) end
    print("======================== E N D ========================")
end
io.write("\nWould you like to save the translated code as a .lua file? (y/N) -> ")
if string.lower(io.read()) == "y" then
    io.write("\nType the name of the new file (???.lua) -> ")
    local filename = io.read()
    local file = io.open(filename .. ".lua", "w+")
    file:write(code)
    file:close()
    print(string.format("Finished writing code to %s.lua", filename))
end
print("Exiting...")

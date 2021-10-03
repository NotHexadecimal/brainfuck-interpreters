---
--- Created by Rampeo Mattone (aka Nylon)
--- DateTime: 2020-09-18 23:22
--- Last updated: 2020-09-20 15:04
---

local premade_fun = "\
local values, pointer = {}, 0\
setmetatable(values, {__index = function() return 0 end})\
local function v() return values[pointer] end\
local function p(val) pointer = pointer + val end\
local function add(val) values[pointer] = (values[pointer] + val) % 256 end\
local function out() io.write(string.char(v())); io.flush() end\
local function input() values[pointer] = string.byte(io.read(1)) end\
"

local function optimizer(buf)
    print("optimizing...")
    local opt = {}
    local i, max_i, c, old_i = 1, #buf + 1, 0 -- i -> itera nel buf, max_i serve per non andare oltre i limiti del buffer
    while i < max_i do
        ::continue::
        old_i = i
        while string.find(buf[i] or "", "p%([%-]?%d%)") do
            c = string.match(buf[i], "p%(([%-]?%d)%)") + c
            i = i + 1
        end
        if c ~= 0 then
            table.insert(opt, string.format("%s%s%s", "p(", c, ")"))
            c = 0
        end
        if i ~= old_i then goto continue end-- se è stato ottimizzato allora riparti con il checker
        while string.find(buf[i] or "", "add%([%-]?%d%)") do
            c = string.match(buf[i], "add%(([%-]?%d)%)") + c
            i = i + 1
        end
        if c ~= 0 then
            table.insert(opt, string.format("%s%s%s", "add(", c, ")"))
            c = 0
        end
        if i ~= old_i then goto continue end-- se è stato ottimizzato allora riparti con il checker
        -- se finisco oltre questo commento è perché la linea di codice non può essere condensata
        table.insert(opt, buf[i])
        i = i + 1
    end
    print("Finished optimizing!")
    return table.concat(opt, "\n")
end

local function interpreter (bf)
    print("Transpiling...")
    local code_buf = {}
    for op in bf:gmatch("[<>%.,%+%-%[%]]") do
        if op == ">" then
            table.insert(code_buf, "p(1)")
        elseif op == "<" then
            table.insert(code_buf, "p(-1)")
        elseif op == "+" then
            table.insert(code_buf, "add(1)")
        elseif op == "-" then
            table.insert(code_buf, "add(-1)")
        elseif op == "." then
            table.insert(code_buf, "out()")
        elseif op == "," then
            table.insert(code_buf, "input()")
        elseif op == "[" then
            table.insert(code_buf, "while v() ~= 0 do")
        elseif op == "]" then
            table.insert(code_buf, "end")
        end
    end
    print("Finished transpiling!")
    return premade_fun .. optimizer(code_buf)
end

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

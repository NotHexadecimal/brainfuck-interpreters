---
--- Created by Rampeo Mattone.
--- DateTime: 2020-09-18 23:22
---

local interpreter -- = require("deps/interpreter") -- old require, deprecated.
do
    local premade_fun = "\
local values, pointer = {}, 0\
setmetatable(values, {__index = function() return 0 end})\
local v = function() return values[pointer] end\
local p = function(val) pointer = pointer + val end\
local add = function(val) values[pointer] = (values[pointer] + val) % 256 end\
local out = function() io.write(string.char(v())) end\
local input = function() return string.byte(io.read(1)) end\
\n"

    local function optimizer(buf)
        local opt = {}
        local i, max_i, c, old_i = 1, #buf + 1, 0 -- i -> itera nel buf, max_i serve per non andare oltre i limiti del buffer
        while i < max_i do
            ::continue::
            old_i = i
            while string.find(buf[i], "p%([%-]?%d%)") do
                c = string.match(buf[i], "p%(([%-]?%d)%)") + c
                i = i + 1
            end
            if c ~= 0 then
                table.insert(opt, string.format("%s%s%s", "p(", c, ")"))
                c = 0
            end
            if i ~= old_i then goto continue end-- se è stato ottimizzato allora riparti con il checker
            while string.find(buf[i], "add%([%-]?%d%)") do
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
        return table.concat(opt, "\n")
    end

    interpreter = function(bf)
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
        return premade_fun .. optimizer(code_buf)
    end
end -- this block of code defines the interpreter and exposes it to the rest of the script

local function read_brainfuckery() --reads the whole code and passes it to the caller as a string
    if arg[1] then return io.open(arg[1]):read("a")
    else
        io.write("\nType the code you want to execute. input a \"__stop__\" to finish:\n")
        local buffer = {}
        repeat
            local l = io.read("l")
            table.insert(buffer, l)
        until l == "__stop__"
        return table.concat(buffer)
    end
end

local code = interpreter(read_brainfuckery())
local prog = load(code)
io.write("\nCompilation has terminated. Would you like to run the code? (Y/n) -> ")
io.flush()
if string.lower(io.read("l")) ~= "n" then prog() end
io.write("\nWould you like to save the code to a binary file? (y/N) -> ")
io.flush()
if string.lower(io.read("l")) == "y" then
    io.write("\nType the name of the new file -> ")
    io.flush()
    local file = io.open(io.read("l"), "w+")
    file:write(string.dump(prog))
    file:close()
    print("Done")
end
print("Exiting")
---
--- Generated by EmmyLua(https://github.com/EmmyLua)
--- Created by rm.
--- DateTime: 27/09/20 11:40
---

return function() --reads the whole code and passes it to the caller as a string
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
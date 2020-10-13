---
--- Generated by EmmyLua(https://github.com/EmmyLua)
--- Created by rm.
--- DateTime: 27/09/20 11:37
---
local values, pointer = {}, 0
setmetatable(values, {__index = function() return 0 end})
local function v() return values[pointer] end
local function p(val) pointer = pointer + val end
local function add(val) values[pointer] = (values[pointer] + val) % 256 end
local function out() io.write(string.char(v())); io.flush() end
local function input() values[pointer] = string.byte(io.read(1)) end

return {v, p, add, out, input}
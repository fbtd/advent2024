local iter = io.stdin:lines()

local instruction_pointer = 0
local reg_a = math.tointeger(iter():sub(13))
local reg_b = math.tointeger(iter():sub(13))
local reg_c = math.tointeger(iter():sub(13))
local _ = iter()
local program = iter():sub(10) .. ","

print(reg_a, reg_b, reg_c, program)

local function print_out(n)
    io.write(math.tointeger(n % 8) .. ",")
end

-- instructions return a, b, c, ip, output
local function adv(a, b, c, ip, operand)
    return a // 2 ^ operand, b, c, ip + 1
end

local function bxl(a, b, c, ip, operand)
    return a, b ~ operand, c, ip + 1
end

local function bst(a, b, c, ip, operand)
    return a, operand % 8, c, ip + 1
end

local function jnz(a, b, c, ip, operand)
    if a == 0 then
        return a, b, c, ip + 1
    end
    return a, b, c, operand
end

local function bxc(a, b, c, ip, operand)
    return a, b ~ c, c, ip + 1
end

local function out(a, b, c, ip, operand, out_fn)
    -- out_fn(operand)
    return a, b, c, ip + 1, math.tointeger(operand % 8) .. ","
end

local function bdv(a, b, c, ip, operand)
    return a, a // 2 ^ operand, c, ip + 1
end

local function cdv(a, b, c, ip, operand)
    return a, b, a // 2 ^ operand, ip + 1
end


-- operand parsers
local function literal(a, b, c, operand)
    return math.tointeger(operand)
end

local function combo(a, b, c, operand)
    if operand <= 3 then
        return operand
    elseif operand == 4 then
        return a
    elseif operand == 5 then
        return b
    elseif operand == 6 then
        return c
    end
    error("unknown operand " .. operand)
end

local opcode_to_function = { adv, bxl, bst, jnz, bxc, out, bdv, cdv }
local opcode_to_operand_parser = { combo, literal, combo, literal, literal, combo, combo, combo }

local function execute(p, ip, a, b, c, abort_if_no_match_p)
    local total_output = ""
    while ip * 4 < #p do
        local output
        local base = (ip * 4)
        if base > #p then return nil end
        local opcode = math.tointeger(p:sub(base + 1, base + 1))
        local operand = math.tointeger(p:sub(base + 3, base + 3))
        local operand_parsed = opcode_to_operand_parser[opcode + 1](a, b, c, operand)
        -- print("ip: " .. ip .. "   opcode: " .. opcode .. "  operand: " .. operand_parsed)
        a, b, c, ip, output = opcode_to_function[opcode + 1](a, b, c, ip, operand_parsed)

        if output then
            total_output = total_output .. output
        end

        if abort_if_no_match_p and total_output ~= p:sub(1, #total_output) then
            -- print("no match: " .. total_output)
            break
        end

        if total_output == p then
            print("match!")
            return true
        end
    end

    -- print(total_output)
    return false
end

local output = {}

execute(program, 0, reg_a, reg_b, reg_c, false)

reg_a = 0
while not execute(program, 0, reg_a, reg_b, reg_c, true) do
    reg_a = reg_a + 1
    if reg_a % 1000000 == 0 then print("trying a=" .. reg_a) end
end
print(reg_a)

local bit = require("bit")
local math = require("math")

local iter = io.stdin:lines()

local MAX_PROG_LEN = 10000

local instruction_pointer = 0
local reg_a = math.floor(iter():sub(13))
local reg_b = math.floor(iter():sub(13))
local reg_c = math.floor(iter():sub(13))
local _ = iter()
local program = iter():sub(10) .. ","

print(reg_a, reg_b, reg_c, program)

local function print_set(set)
    for k, v in pairs(set) do
        io.write(k .. " ")
    end
    print()
end


local function print_out(n)
    io.write(math.floor(n % 8) .. ",")
end

-- instructions return a, b, c, ip, output
local function adv(a, b, c, ip, operand)
    return math.floor(a / 2 ^ operand), b, c, ip + 1
end

local function bxl(a, b, c, ip, operand)
    return a, bit.bxor(b, operand), c, ip + 1
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
    return a, bit.bxor(b, c), c, ip + 1
end

local function out(a, b, c, ip, operand, out_fn)
    -- out_fn(operand)
    return a, b, c, ip + 1, math.floor(operand % 8) .. ","
end

local function bdv(a, b, c, ip, operand)
    return a, math.floor(a / 2 ^ operand), c, ip + 1
end

local function cdv(a, b, c, ip, operand)
    return a, b, math.floor(a / 2 ^ operand), ip + 1
end


-- operand parsers
local function literal(a, b, c, operand)
    return math.floor(operand)
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

-- verbose 1 = print partial matches, 2 = print out, 3 = print steps
local function execute(p, ip, a, b, c, abort_if_no_match_p, verbose, min_match)
    verbose = verbose or 0
    local total_output = ""
    local original_a = a
    local i = 0
    while ip * 4 < #p do
        i = i + 1
        if i > MAX_PROG_LEN then
            print("max prog len reached for a=" .. a)
            return false
        end
        local output
        local base = (ip * 4)
        if base > #p then return nil end
        local opcode = math.floor(p:sub(base + 1, base + 1))
        local operand = math.floor(p:sub(base + 3, base + 3))

        if verbose >= 3 then
            print("about to execute: a=" .. a .. " b=" .. b .. " c=" .. c .. " ip=" .. ip .. " opcode=" .. opcode)
        end

        local operand_parsed = opcode_to_operand_parser[opcode + 1](a, b, c, operand)
        -- print("ip: " .. ip .. "   opcode: " .. opcode .. "  operand: " .. operand_parsed)
        a, b, c, ip, output = opcode_to_function[opcode + 1](a, b, c, ip, operand_parsed)

        if output then
            total_output = total_output .. output
        end

        if total_output == p then
            print(string.format("%i", original_a))
            -- print(string.format("ZZ match for original a=%i, t=%s", original_a, total_output))
            return true
        end

        if min_match and total_output == p:sub(1, min_match) then
            if verbose >= 1 then
                print("partial match: ", total_output)
            end
            return true
        end

        if abort_if_no_match_p and total_output ~= p:sub(1, #total_output) then
            -- print("no match: " .. total_output)
            break
        end
    end

    if verbose >= 2 then
        print("total_output: "..total_output)
    end

    return false
end

local output = {}

execute(program, 0, reg_a, reg_b, reg_c, false, 2)

-- os.exit(0)

local candidates = {}
candidates[0]    = true

for len = 2, #program, 2 do
    local new_candidates = {}
    for candidate, _ in pairs(candidates) do
        -- add 2 octs and see what sticks
        for l = 0, 15 do
            for r = 0, 15 do
                local new_candidate = candidate
                    + l * 2 ^ (3 * (len / 2))
                    + r * 2 ^ (3 * (len / 2 - 1))
                local new_candidate_truncated = candidate
                    + r * 2 ^ (3 * (len / 2 - 1))
                if new_candidate > 0 then
                    local result = execute(program, 0, new_candidate, reg_b, reg_c, false, 0, len)
                    if result then
                        if len == #program then
                            -- print("found match with reg_a = " .. new_candidate)
                            -- print("truncated = " .. new_candidate_truncated)
                        else
                            new_candidates[new_candidate_truncated] = true
                        end
                    end
                else
                    -- print(l, r, new_candidate, r * 2 ^ (3 * (len / 2 - 1)))
                end
            end
        end
    end
    -- print_set(new_candidates)
    candidates = new_candidates
end




-- local po        = false
-- local ab        = true
--
-- if arg[3] then
--     reg_a = math.floor(arg[3])
--     min_reg_a = reg_a - 1
--     po = true
--     ab = false
-- end

-- while reg_a > min_reg_a do
--     execute(program, 0, reg_a, reg_b, reg_c, ab, false, po)
--     reg_a = reg_a - n_workers
-- end
-- print(reg_a)

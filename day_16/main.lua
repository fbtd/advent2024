Maze = require("lualib.maze")

local input = io.read("a")
local result_step_1 = math.maxinteger

local m = Maze.from_string(input)

local Visited = {}
function Visited:new(o)
    o = o or {}
    self.__index = self
    setmetatable(o, self)
    return o
end

function Visited:get_active_index()
    for index, val in pairs(self) do
        if type(index) == "number" then
            for heading, score_active in pairs(val) do
                if score_active.active then
                    return index, heading, score_active.score
                end
            end
        end
    end
    return nil
end

function Visited:update(index, heading, score)
    if not self[index] then
        self[index] = {
            E = { score = math.maxinteger, active = false },
            S = { score = math.maxinteger, active = false },
            W = { score = math.maxinteger, active = false },
            N = { score = math.maxinteger, active = false }
        }
    end

    local current_score = self[index][heading].score
    if score < current_score then
        self[index][heading].score = score
        self[index][heading].active = true
    end
end

function Visited:run(maze)
    while true do
        local i, heading, score = self:get_active_index()
        if not i then break end
        -- if i == m:xy_to_index(1, maze.height - 2) then
        --     print("candidate:", heading, score)
        -- end

        -- forward
        local i_to_check = maze:index_add(i, heading)
        local slot = maze:get(i_to_check)
        if slot == "." or slot == "E" or slot == "S" then
            self:update(i_to_check, heading, score + 1)
        end

        -- turn
        for _, new_heading in pairs { "E", "S", "W", "N" } do
            -- no 180!
            if not (heading == "E" and new_heading == "W"
                    or heading == "W" and new_heading == "E"
                    or heading == "S" and new_heading == "N"
                    or heading == "N" and new_heading == "S") then
                self:update(i, new_heading, score + 1000)
            end
        end

        -- mark as visited
        self[i][heading].active = false
    end
end

local from_s = {}
from_s[m:xy_to_index(1, m.height - 2)] = {
    E = { score = 0000, active = true },
    S = { score = 1000, active = true },
    W = { score = 1000, active = true },
    N = { score = 1000, active = true }
}
local visited_from_s = Visited:new(from_s)
visited_from_s:run(m)


-- step 1
for _, h in pairs { "E", "S", "W", "N" } do
    result_step_1 = math.min(result_step_1, visited_from_s[m:xy_to_index(m.width - 2, 1)][h].score)
end


-- step 2
local from_e = {}
from_e[m:xy_to_index(m.width - 2, 1)] = {
    E = { score = 0000, active = true },
    S = { score = 0000, active = true },
    W = { score = 0000, active = true },
    N = { score = 0000, active = true }
}
local visited_from_e = Visited:new(from_e)
visited_from_e:run(m)

local score_e_to_s = visited_from_e[m:xy_to_index(1, m.height - 2)]["E"].score
print("score e -> s", score_e_to_s)

local result_step_2 = 0
for index, val_s in pairs(visited_from_s) do
    if type(index) == "number" then
        local val_e = visited_from_e[index]

        -- distance to E + distance to S [ + turn] == perfect score?
        if val_s["S"].score + val_e["N"].score == result_step_1
            or val_s["N"].score + val_e["S"].score == result_step_1
            or val_s["E"].score + val_e["W"].score == result_step_1
            or val_s["W"].score + val_e["E"].score == result_step_1

            or val_s["N"].score + val_e["E"].score + 1000 == result_step_1
            or val_s["N"].score + val_e["W"].score + 1000 == result_step_1

            or val_s["S"].score + val_e["E"].score + 1000 == result_step_1
            or val_s["S"].score + val_e["W"].score + 1000 == result_step_1

            or val_s["E"].score + val_e["N"].score + 1000 == result_step_1
            or val_s["E"].score + val_e["S"].score + 1000 == result_step_1

            or val_s["W"].score + val_e["N"].score + 1000 == result_step_1
            or val_s["W"].score + val_e["S"].score + 1000 == result_step_1
        then
            -- print(m:index_to_xy(index))
            result_step_2 = result_step_2 + 1
        end
    end
end

print("result step 1:", result_step_1)
print("result step 2:", result_step_2)

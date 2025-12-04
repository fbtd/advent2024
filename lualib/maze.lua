local Maze = {}


function Maze:_new(o)
    o = o or {}
    self.__index = self
    setmetatable(o, self)
    return o
end

function Maze.from_string(s)
    local t = {}
    t.width = string.find(s, "\n", 1, true) - 1
    local as_string = string.gsub(s, "\n", "")
    t.height = #as_string // t.width
    t.sequence = table.pack(as_string:byte(1, #as_string))
    t.size = #as_string
    return Maze:_new(t)
end

-- x and y starts at 0
-- index starts at 1.......
function Maze:xy_to_index(x, y)
    return x + self.width * y + 1
end

function Maze:index_to_xy(index)
    local x = (index - 1) % self.width
    local y = (index - 1) // self.height
    return x, y
end

function Maze:index_add(index, heading, n)
    n = n or 1
    local x, y = self:index_to_xy(index)
    if heading == "N" then
        return self:xy_to_index(x, y - n)
    elseif heading == "E" then
        return self:xy_to_index(x + n, y)
    elseif heading == "S" then
        return self:xy_to_index(x, y + n)
    elseif heading == "W" then
        return self:xy_to_index(x - n, y)
    end
    error("unknown heading ".. heading)
end

function Maze:xy_add(x, y, add_to_x, add_to_y)
    local destination_x = math.max(math.min(x + add_to_x, self.width - 1), 0)
    local destination_y = math.max(math.min(y + add_to_y, self.height - 1), 0)
    return destination_x, destination_y
end

function Maze:get(x_or_index, y)
    if not y then
        return string.char(self.sequence[x_or_index])
    end
    return string.char(self.sequence[self:xy_to_index(x_or_index, y)])
end

function Maze:set(x, y, value)
    self.sequence[self:xy_to_index(x, y)] = value:byte()
end

return Maze

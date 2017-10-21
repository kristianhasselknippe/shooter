local vec2 = require "vec2"
local helpers = require "helpers"

local entity = {}

local function new(name, pos)
   return setmetatable({
		 name = name,
		 position = pos,
		 rotation = 0,
		 components = {}
	}, entity)
end

function entity:add_component(comp)
   table.insert(self.components, comp)
end

function create_entity(name)
   entities[name] = new(name, vec2.new(0,0))
end

function get_entity(name)
   return entities[name]
end


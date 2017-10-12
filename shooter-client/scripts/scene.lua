local vector = require "vector"
local helpers = require "helpers"

function create_entity(name)
   entities[name] = {
	  name = name,
	  position = vector.new(0,0)
   }
end

function get_entity(name)
   return entities[name]
end
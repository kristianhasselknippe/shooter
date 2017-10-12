local vector = require "vector"
print("Loaded vector");
function create_entity(name)
   local id = entities_id_counter
   entities[id] = {
	  name = name,
	  position = vector.new(0,0)
   }
   entities_id_counter = entities_id_counter + 1
   print("Lua returning id: " .. id)
   return id
end

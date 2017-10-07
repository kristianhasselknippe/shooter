function create_entity(name)
   local id = entities_id_counter
   entities[id] = {
	  name = name,
	  position = {
		 x = 0,
		 y = 0,
	  }
   }
   entities_id_counter = entities_id_counter + 1
   print("Lua returning id: " .. id)
   return id
end

function build_scene()
   print("building scene")
end
build_scene()

print("we are loading scene");
print("we are loading scene");
print("we are loading scene");

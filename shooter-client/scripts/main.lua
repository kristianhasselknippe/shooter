entities_id_counter = 1
entities = {}

function main()
   print("Hello world from lua")
end

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
   return id
end

input  = {
   left_down = false,
   up_down = false,
   right_down = false,
   down_down = false,
}

function update_input(left,up,right,down)
   input.left_down = left
   input.up_down = up
   input.right_down = right
   input.down_down = down
end

speed = 20.5
function update_entities(dt)
   for _,e in ipairs(entities) do
	  if e.name == "player" then

		 local vec = { x = 0, y = 0 }
		 if input.left_down then vec.x = vec.x - 1 end
		 if input.up_down then vec.y = vec.y + 1 end
		 if input.right_down then vec.x = vec.x + 1 end
		 if input.down_down then vec.y = vec.y - 1 end
		 e.position.x = e.position.x + vec.x * dt * speed
		 e.position.y = e.position.y + vec.y * dt * speed
	  end
   end
end

function get_entity(id)
   local ret = entities[id]
   return entities[id]
end

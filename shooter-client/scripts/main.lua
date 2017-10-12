local helpers = require "helpers"

function main()
   print("Hello world from lua")
end

function update_input(left,up,right,down)
   input.left_down = left
   input.up_down = up
   input.right_down = right
   input.down_down = down
end

speed = 10.5
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
	  if e.name == "camera" then
		 local player_pos = get_entity("player").position
		 --local dir = player_pos - e.position
		 
	  end
   end
end


   
function get_entity(id)
   if type(id) == "number" then
	  local ret = entities[id]
	  return entities[id]
   elseif type(id) == "string" then
	  for k,e in pairs(entities) do
		 if e.name == id then
			return e
		 end
	  end
   end
end

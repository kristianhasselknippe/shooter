local helpers = require "helpers"
local vec = require "vector"

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
   for name,e in pairs(entities) do
	  if e.name == "player" then
		 local vec = vec.new(0,0)
		 if input.left_down then vec.x = vec.x - 1 end
		 if input.up_down then vec.y = vec.y + 1 end
		 if input.right_down then vec.x = vec.x + 1 end
		 if input.down_down then vec.y = vec.y - 1 end

		 e.position = e.position + vec * dt * speed;
	  end
	  if e.name == "camera" then
		 local player_pos = get_entity("player").position
		 local direction = player_pos - e.position
		 print("Dir " .. helpers.tprint(direction))
		 local dir = direction.normalizeInplace()
		 print("Diretion " .. helpers.tprint(dir))
		 --e.position = e.position + direction * dt;
	  end
   end
end

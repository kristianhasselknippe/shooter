local helpers = require "helpers"
local vec2 = require "vec2"

function main()
   print("Hello world from lua")
end

function update_input(left,up,right,down)
   input.left_down = left
   input.up_down = up
   input.right_down = right
   input.down_down = down
end

speed = 3.5
function update_entities(dt)
   for name,e in pairs(entities) do
	  if e.name == "player" then
		 local vec = vec2.new(0,0)
		 if input.left_down then
			vec.x = vec.x - speed
			e.rotation = e.rotation + 10
		 end
		 if input.right_down then
			vec.x = vec.x + speed
			e.rotation = e.rotation - 0.1
		 end
		 
		 if input.up_down then
			vec.y = vec.y + speed
			--Camera.set_size(Camera.instance, 0.1 * speed, 0.1 * speed);
		 end
		 if input.down_down then
			vec.y = vec.y - speed
			--Camera.set_size(Camera.instance, 0.1 * -speed, 0.1 * -speed);
		 end

		 e.position = e.position + vec * dt * speed;
	  end
	  if e.name == "camera" then
		 local player_pos = get_entity("player").position
		 local direction = player_pos - e.position
		 local dir = vec2.normalize(direction)
		 e.position = player_pos --e.position + direction * dt * speed / 2;
	  end
   end
end

function test()
   print("Ref: " .. tostring(GameStateRef))
   local e = GameState.get_entity(GameStateRef, "player")
   print("Entity " .. e);
end

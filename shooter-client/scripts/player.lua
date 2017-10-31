local helpers = require("helpers");
local vec2 = require("vec2");

local player = {}

local speed = 10.3

function update(gs, dt, entity)
   print(helpers.tprint(GameState));
   print(helpers.tprint(Entity));
   print("Player script running");
   local e = GameState.get_entity(gs, entity)
   local input = Input.get_input(InputRef);

   local p = Entity.get_pos(e);
   local vec = vec2.new(p.x, p.y);

   if input.left_down then
	  vec.x = vec.x - speed * dt
   end
   if input.right_down then
	  vec.x = vec.x + speed * dt
   end
   
   if input.up_down then
	  vec.y = vec.y + speed * dt
   end
   if input.down_down then
	  vec.y = vec.y - speed * dt
   end

   Entity.set_pos(e, vec.x, vec.y);
end

player["update"] = update

return player

local helpers = require("helpers");
local vec2 = require("vec2");

local player = {}

local speed = 0.3

function update(gs, dt, entity)
   local e = GameState.get_entity(gs, entity)

   local p = Entity.get_pos(e);
   helpers.tprint(p);

   local vec = vec2.new(p.x, p.y);

   print("Vec: " .. tostring(vec.x) .. ", " .. tostring(vec.y));

   if input.left_down then
	  vec.x = vec.x - speed
   end
   if input.right_down then
	  vec.x = vec.x + speed
   end
   
   if input.up_down then
	  vec.y = vec.y + speed
   end
   if input.down_down then
	  vec.y = vec.y - speed
   end

   Entity.set_pos(e, vec.x, vec.y);
end

player["update"] = update

return player

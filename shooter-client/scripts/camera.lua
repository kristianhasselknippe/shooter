local helpers = require("helpers");
local vec2 = require("vec2");

local camera = {
   player = nil
}

local camera_speed = 3.3

function camera.update(gs, dt, entity)
   --print(helpers.tprint(GameState));
   --print(helpers.tprint(Entity));
   --print("Camera script running");
   local player_entity = GameState.get_entity(gs, camera.player)
   --print("GameState worked");
   
   local pp = Entity.get_pos(player_entity);

   local player_pos = vec2.new(pp.x,pp.y);

   local e = GameState.get_entity(gs, entity);
   local cp = Entity.get_pos(e);
   local camera_pos = vec2.new(cp.x, cp.y)

   local dir = player_pos - camera_pos
   camera_pos = camera_pos + dir * camera_speed * dt;

   Entity.set_pos(e, camera_pos.x, camera_pos.y);
end

return camera

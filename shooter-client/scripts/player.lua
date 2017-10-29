local player = {}

function update(gs, dt, entity)
   print("Player was updated; dt" .. tostring(dt))
   print("entity ref" .. tostring(entity))

   print("Looking for entity in gs: " .. tostring(gs))

   local e = GameState.get_entity(gs, entity)
   print("Done getting entity: " .. tostring(e));
end

player["update"] = update
player["foobar"] = "hello there"

return player

local player = {}

function update(dt, entity)
   print("Player was updated; dt" .. tostring(dt))
   print("entity ref" .. tostring(entity))
end

player["update"] = update
player["foobar"] = "hello there"

return player

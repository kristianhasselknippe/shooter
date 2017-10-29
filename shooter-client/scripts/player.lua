local player = {}

function update(dt, entity)
   print("Player was updated" .. tostring(dt))
end

player["update"] = update
player["foobar"] = "hello there"

return player

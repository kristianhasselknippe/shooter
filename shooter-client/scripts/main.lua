scene = {}
game_object_id_counter = 0
game_objects = {}

function main()
   print("Hello world from lua")
end


function create_game_object(name)
   local id = game_object_id_counter
   print("Name " .. name .. " id: " .. id)
   game_object_id_counter = game_object_id_counter + 1
   game_objects[id] = {
	  name = name
   }
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

   print("Input updated " .. tostring(left) .. " " .. tostring(up) .. " " .. tostring(right) .. " " .. tostring(down))
end

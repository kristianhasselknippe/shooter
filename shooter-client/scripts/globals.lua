entities_id_counter = 1
entities = {}

input  = {
   left_down = false,
   up_down = false,
   right_down = false,
   down_down = false,
}

function get_some(foo, bar)
   print("Called get some" .. foo .. " ::: " .. bar);
   return 123123;
end

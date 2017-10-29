local helpers = require('helpers')

__entity_scripts = {};

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
end

function debug_scripts()
   print("Debugging entity scripts");
   print(tostring(__entity_scripts));
   helpers.tprint(__entity_scripts);
end

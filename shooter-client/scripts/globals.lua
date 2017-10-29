local helpers = require('helpers')

__entity_scripts = {};


function debug_scripts()
   print("Debugging entity scripts");
   print(tostring(__entity_scripts));
   helpers.tprint(__entity_scripts);
end

entities_id_counter = 1
entities = {}

input  = {
   bar = "roflmao",
   t = {},
   foo = {
	  flais = {
		 foo = 123,
		 bar = 123123,
		 hei = "hallo"
	  },
	  blais = {
		 foo = 123,
		 bar = 123123,
		 hei = "hallo"
	  }
   },
   bar1 = "roflmao1"
}

function tprint (tbl, indent)
   if not indent then indent = 0 end
   local toprint = string.rep(" ", indent) .. "{\r\n"
   indent = indent + 2 
   for k, v in pairs(tbl) do
	  toprint = toprint .. string.rep(" ", indent)
	  if (type(k) == "number") then
		 toprint = toprint .. "[" .. k .. "] = "
	  elseif (type(k) == "string") then
		 toprint = toprint  .. k ..  "= "   
	  end
	  if (type(v) == "number") then
		 toprint = toprint .. v .. ",\r\n"
	  elseif (type(v) == "string") then
		 toprint = toprint .. "\"" .. v .. "\",\r\n"
	  elseif (type(v) == "table") then
		 toprint = toprint .. "table\r\n" --tprint(v, indent + 2) .. ",\r\n"
	  else
		 toprint = toprint .. "\"" .. tostring(v) .. "\",\r\n"
	  end
   end
   toprint = toprint .. string.rep(" ", indent-2) .. "}"
   return toprint
end




print("printing input") 
print(tprint(input));

print(tprint(package.loaded))

function get_some(foo, bar)
   print("Called get some" .. foo .. " ::: " .. bar);
   return 123123;
end

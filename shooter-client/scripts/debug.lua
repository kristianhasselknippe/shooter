function main_error_handler(err_msg)
   return err_msg .. "\n" .. debug.traceback()
end

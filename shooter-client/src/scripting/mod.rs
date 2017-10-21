pub mod script;
pub mod lua;
pub mod userdata;

use self::script::*;
use self::lua::*;

use super::na::Vector3;
use std::path::Path;
use of::OrderedFloat;

use super::entities::*;
use super::camera::Camera;

pub struct ScriptEngine {
    pub lua: Lua,
    script_watcher: ScriptWatcher, //TODO(Rename ScriptWatcher)
}

impl ScriptEngine {
    pub fn new() -> ScriptEngine {
        let mut lua = Lua::new();
        lua.open_libs();

        println!("Loading userdata libraries");
        lua.new_userdata(&Camera::get_userdata());
        println!("Done loading userdata libraries");

        let mut sw = ScriptWatcher::new(&Path::new("scripts"));
        sw.new_script_from_file(&Path::new("scripts/debug.lua")).load(&mut lua);

        sw.new_script_from_file(&Path::new("scripts/math/vec3.lua")).load(&mut lua);
        sw.new_script_from_file(&Path::new("scripts/math/vec2.lua")).load(&mut lua);
        sw.new_script_from_file(&Path::new("scripts/math/constants.lua")).load(&mut lua);
        sw.new_script_from_file(&Path::new("scripts/math/quat.lua")).load(&mut lua);
        sw.new_script_from_file(&Path::new("scripts/math/utils.lua")).load(&mut lua);
        sw.new_script_from_file(&Path::new("scripts/math/mat4.lua")).load(&mut lua);

        sw.new_script_from_file(&Path::new("scripts/math/color.lua")).load(&mut lua);

        sw.new_script_from_file(&Path::new("scripts/math/intersect.lua")).load(&mut lua);

        sw.new_script_from_file(&Path::new("scripts/math/mesh.lua")).load(&mut lua);
        sw.new_script_from_file(&Path::new("scripts/math/octree.lua")).load(&mut lua);

        sw.new_script_from_file(&Path::new("scripts/math/simplex.lua")).load(&mut lua);


        
        sw.new_script_from_file(&Path::new("scripts/helpers.lua")).load(&mut lua);
        sw.new_script_from_file(&Path::new("scripts/globals.lua")).load(&mut lua);
        sw.new_script_from_file(&Path::new("scripts/scene.lua")).load(&mut lua);
        sw.new_script_from_file(&Path::new("scripts/main.lua")).load(&mut lua);



        println!("Done loading modules");
        lua.print_stack_dump();
        println!("Done loading scripts");

        ScriptEngine {
            lua: lua,
            script_watcher: sw,
        } 
    }

    pub fn pre_update(&mut self) {
        self.script_watcher.tick(&mut self.lua);
    }

    pub fn call(&mut self, name: &str, args: &[LuaType]) -> Result<LuaType, ()> {
        let lua_args: Vec<LuaType> = args.iter().map(|a| LuaType::from(a.clone())).collect();
        let ret = self.lua.call_global(name, &lua_args).and_then(|r| {
            Ok(LuaType::from(r))
        });
        ret
    }

    pub fn update_input(&mut self, left_down: bool,up_down: bool,right_down: bool,down_down: bool) {
        self.call("update_input", &[
            LuaType::Bool(left_down),
            LuaType::Bool(up_down),
            LuaType::Bool(right_down),
            LuaType::Bool(down_down)
        ]).unwrap();
    }

    pub fn get_entities(&mut self) -> Vec<Entity> {
        let entities = self.lua.get_global("entities").unwrap();

        let mut ret = Vec::new();
        if let Some(entities) = entities.unwrap_array() {
            for entity in entities {
                ret.push(Entity::from_lua_type(&entity));
            }
        }
        ret
    }

    pub fn update_entities(&mut self, dt: f64) {
        self.lua.call_global("update_entities", &[LuaType::Number(OrderedFloat(dt))]).unwrap();
    }

    pub fn get_entity(&self, name: &str) -> Option<Entity> {
        let e = self.lua.call_global("get_entity", &[LuaType::String(name.to_string())]).unwrap();
        match e {
            LuaType::Table(_) => Some(Entity::from_lua_type(&e)),
            _ => {
                print!("Got null while tryingt og et entity {}", name);
                None
            },
        }
    }

    pub fn add_entity(&mut self, name: &str) {
        println!("Adding entity");
        let r = self.lua.call_global("create_entity", &[LuaType::String(name.to_string())]);
    }
}

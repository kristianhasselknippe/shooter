pub mod script;
#[macro_use] pub mod lua;
pub mod userdata;

use self::script::*;
use self::lua::*;
use super::na::Vector3;
use std::path::Path;
use of::OrderedFloat;

use super::entities::*;
use super::camera::Camera;
use super::game_state::GameState;

#[macro_export]
macro_rules! userdata {
    ($name:expr, $( $x:expr => $y:expr ),* ) => {
        {
            let library = {
                let mut library = Vec::new();
                $(
                    library.push(luaL_Reg::new($x, $y));
                )*
                library.push(luaL_Reg::null());
                library
            };
            UserData {
                name: $name.to_string(),
                methods: library,
            }
        }
    };
}

#[macro_export]
macro_rules! luafunction {
    ($name:ident, $lua:ident, $body:expr) => {
        extern "C" fn $name($lua: *mut lua_State) -> c_int {
            $body
        }
    }
}

#[derive(Debug)]
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
        lua.new_userdata(&GameState::get_userdata());
        println!("Done loading userdata libraries");

        let mut sw = ScriptWatcher::new(&Path::new("scripts"));
        sw.new_script_from_path(&Path::new("scripts/debug.lua"), &lua);
        sw.new_script_from_path(&Path::new("scripts/math/vec3.lua"), &lua);
        sw.new_script_from_path(&Path::new("scripts/math/vec2.lua"), &lua);
        sw.new_script_from_path(&Path::new("scripts/math/constants.lua"), &lua);
        sw.new_script_from_path(&Path::new("scripts/math/quat.lua"), &lua);
        sw.new_script_from_path(&Path::new("scripts/math/utils.lua"), &lua);
        sw.new_script_from_path(&Path::new("scripts/math/mat4.lua"), &lua);
        sw.new_script_from_path(&Path::new("scripts/math/color.lua"), &lua);
        sw.new_script_from_path(&Path::new("scripts/math/intersect.lua"), &lua);
        sw.new_script_from_path(&Path::new("scripts/math/mesh.lua"), &lua);
        sw.new_script_from_path(&Path::new("scripts/math/octree.lua"), &lua);
        sw.new_script_from_path(&Path::new("scripts/math/simplex.lua"), &lua);
        sw.new_script_from_path(&Path::new("scripts/helpers.lua"), &lua);
        sw.new_script_from_path(&Path::new("scripts/globals.lua"), &lua);
        sw.new_script_from_path(&Path::new("scripts/scene.lua"), &lua);
        sw.new_script_from_path(&Path::new("scripts/main.lua"), &lua);

        println!("Done loading modules");
        lua.print_stack_dump();
        println!("Done loading scripts");

        ScriptEngine {
            lua: lua,
            script_watcher: sw,
        } 
    }

    pub fn new_script_from_path(&mut self, p: &Path) -> Script {
        self.script_watcher.new_script_from_path(p, &self.lua)
    }

    pub fn new_behavior_script_from_path(&mut self, name: &str, p: &Path) -> BehaviorScript {
        let script = self.script_watcher.new_behavior_script_from_path(p, &self.lua);
        BehaviorScript::new(name, script)
    }

    pub fn pre_update(&mut self) {
        self.script_watcher.tick(&mut self.lua);
    }

    pub fn call(&mut self, name: &str, args: &[LuaType]) -> Result<LuaType, ()> {
        self.lua.call_global(name, args)
    }

    pub fn update_input(&mut self, left_down: bool,up_down: bool,right_down: bool,down_down: bool) {
        self.call("update_input", &[
            LuaType::Bool(left_down),
            LuaType::Bool(up_down),
            LuaType::Bool(right_down),
            LuaType::Bool(down_down)
        ]).unwrap();
    }

    pub fn update(&mut self, e: &EntityRef, script: &BehaviorScript, dt: f64) {
        let script_id = script.script.get_string_id();
        self.call(&format!("__entity_scripts.{}.update", script_id), &[
            LuaType::Number(OrderedFloat(dt)),
            LuaType::Number(OrderedFloat(e.0 as f64))
        ]);
    }
}

#[derive(Debug,Clone)]
pub struct BehaviorScript {
    name: String,
    pub script: Script,
}

impl BehaviorScript {
    pub fn new(name: &str, script: Script) -> BehaviorScript {
        BehaviorScript {
            name: name.to_string(),
            script: script,
        }
    }
}

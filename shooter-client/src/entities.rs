extern crate copy_arena;

use std;
use self::copy_arena::Arena;
use na::{Vector2,Vector3,Matrix4,Unit};
use std::ffi::CString;
use super::scripting::*;
use super::scripting::script::*;
use super::scripting::lua::*;
use super::scripting::lua::lua52_sys::*;
use libc::{c_int,c_void};
use super::game_state::GameState;
use std::collections::HashMap;
use of::OrderedFloat;

#[derive(Hash,Eq,PartialEq,Debug,Clone)]
pub struct EntityRef(pub u32);

impl EntityRef {
    pub fn get_string_id(&self) -> String {
        format!("__entity{}", self.0)
    }
}

#[derive(Debug)]
pub struct Entity {
    pub name: String,
    pub pos: Vector3<f32>,
    pub rot: f32,
}

impl Entity {
    pub fn new(name: &str, pos: Vector2<f32>) -> Entity {
        Entity {
            pos: Vector3::new(pos.x, pos.y, 0.0),
            rot: 0.0,
            name: name.to_string(),
        }
    }
}

luafunction!(get_pos, L, {
    unsafe {
        let entity = c_void_to_ref!(Entity, lua_touserdata(L, 1));

        push_new_table(L)
            .with_value("x", &LuaType::Number(OrderedFloat(entity.pos.x as f64)))
            .with_value("y", &LuaType::Number(OrderedFloat(entity.pos.y as f64)));
        1
    }
});

luafunction!(set_pos, L, {
    unsafe {

        let entity = c_void_to_ref!(Entity, lua_touserdata(L, 1));
        let x = lua_tonumberx(L, 2, std::ptr::null_mut());
        let y = lua_tonumberx(L, 3, std::ptr::null_mut());
        (*entity).pos = Vector3::new(x as f32,y as f32, 0.0);
        0
    }
});

impl NativeLibraryProvider for Entity {
   fn get_native_library() -> NativeLibrary {
        nativelualib!(
            "Entity",
            "get_pos" => get_pos,
            "set_pos" => set_pos
        )
    }
}


#[derive(Debug)]
pub struct EntityComponentStore {
    pub entities: HashMap<EntityRef,Entity>,
    pub scripts: HashMap<EntityRef, Vec<BehaviorScript>>,
    components_arena: Arena,

    entity_id_counter: u32,
}

impl EntityComponentStore {
    pub fn new() -> EntityComponentStore {
        EntityComponentStore {
            entities: HashMap::new(),
            components_arena: Arena::new(),
            scripts: HashMap::new(),
            entity_id_counter: 0,
        }
    }

    pub fn add_entity(&mut self, e: Entity) -> EntityRef {
        let ret = EntityRef(self.entity_id_counter);
        self.entity_id_counter += 1;
        self.entities.insert(ret.clone(), e);
        ret
    }

    pub fn add_script(&mut self, e: &EntityRef, script: &BehaviorScript) {
        if !self.scripts.contains_key(e) {
            self.scripts.insert(e.clone(), Vec::new());
        }
        let mut entity_scripts = self.scripts.get_mut(e).unwrap();
        entity_scripts.push(script.clone());
    }

    pub fn get_entity(&self, e: &EntityRef) -> Option<&Entity> {
        self.entities.get(e)
    }
}

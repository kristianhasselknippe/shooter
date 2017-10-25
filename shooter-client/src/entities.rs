extern crate copy_arena;

use self::copy_arena::Arena;
use na::{Vector2,Vector3,Matrix4,Unit};
use super::scripting::*;
use super::scripting::lua::LuaType;
use std::collections::HashMap;

#[derive(Hash,Eq,PartialEq,Debug,Clone)]
pub struct EntityRef(u32);

#[derive(Debug)]
pub struct Entity {
    pub name: String,
    pub pos: Vector3<f32>,
    pub rot: f32,
}

impl Entity {
    pub fn from_lua_type(t: &LuaType) -> Entity {
        let pos = t.get("position").unwrap();
        let x = pos.get("x").unwrap().unwrap_number();
        let y = pos.get("y").unwrap().unwrap_number();
        let rot = t.get("rotation").unwrap().unwrap_number();
        Entity {
            name: t.get("name").unwrap().unwrap_string().to_string(),
            pos: Vector3::new(x as f32,y as f32,0.0),
            rot: rot as f32,
        }
    }
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

#[derive(Debug)]
pub struct EntityComponentStore {
    pub entities: HashMap<EntityRef,Entity>,
    components_arena: Arena,

    entity_id_counter: u32,
}

impl EntityComponentStore {
    pub fn new() -> EntityComponentStore {
        EntityComponentStore {
            entities: HashMap::new(),
            components_arena: Arena::new(),
            entity_id_counter: 0,
        }
    }

    pub fn add_entity(&mut self, e: Entity) -> EntityRef {
        let ret = EntityRef(self.entity_id_counter);
        self.entity_id_counter += 1;
        self.entities.insert(ret, e);
        ret
    }

    pub fn get_entity(&self, e: &EntityRef) -> Option<&Entity> {
        self.entities.get(e)
    }
}

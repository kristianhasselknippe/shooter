use na::{Vector2,Point3,Isometry3,Vector3,Matrix4};
use alga::linear::Transformation;
use texture::{Texture,TextureUnit};
use mesh::{Mesh};
use std::path::{Path};
use drawing::*;
use std::cell::{RefCell,Ref,RefMut};
use std::rc::Rc;

use shader::ShaderProgram;
use std::collections::HashMap;
use super::input::Input;
use super::camera::Camera;
use super::scripting::*;


pub struct Sprite {
    pub pos: Vector3<f32>,
    size: Vector3<f32>,

    texture: Texture,

    program: ShaderProgram, //this should be optimized
    mesh: Mesh,
}

impl Sprite {
    pub fn from_png(path: &Path, w: f32, h: f32) -> Sprite {
        let t = Texture::from_png(path);
        let program = ShaderProgram::create_program("sprite");

        Sprite {
            pos: Vector3::new(0.0,0.0,0.0),
            size: Vector3::new(w,h,1.0),

            texture: t,

            program: program,
            mesh: Mesh::create_quad(),
        }
    }

    pub fn draw(&self, camera_matrix: &Matrix4<f32>) {
        self.program.use_program();
        self.texture.bind(TextureUnit::Unit0);

        let translation = Matrix4::new_translation(&Vector3::new(self.pos.x, self.pos.y, self.pos.z));
        let scaling = Matrix4::new_nonuniform_scaling(&self.size);

        let model = translation * scaling;
        let mvp = camera_matrix * model;

        self.program.set_mat4("mvp", mvp);

        self.mesh.draw_now();
    }
}

#[derive(Debug)]
pub struct Entity {
    pub name: String,
    pub pos: Vector3<f32>,
}

#[derive(Hash,Clone,Copy,Eq,PartialEq, Debug)]
pub struct EntityRef(u32);

impl Entity {
    pub fn new(name: &str, pos: Vector2<f32>) -> Entity {
        Entity {
            pos: Vector3::new(pos.x, pos.y, 0.0),
            name: name.to_string(),
        }
    }
}

pub struct GameState {
    script_engine: ScriptEngine,
}

impl GameState {
    pub fn new() -> GameState {
        let mut script_engine = ScriptEngine::new();
        GameState {
            script_engine: script_engine,
        }
    }

    pub fn new_entity(&mut self, name: &str) -> EntityRef {
        let id = self.script_engine.add_entity(name);
        let ret = EntityRef(id as u32);
        ret
    }

    pub fn get_entity(&mut self, entity_ref: &EntityRef) -> Entity {
        self.script_engine.get_entity(entity_ref.0)
    }

    pub fn get_entities(&mut self) -> Vec<Entity> {
        self.script_engine.get_entities()
    }

    pub fn pre_update(&mut self) {
        self.script_engine.pre_update();
    }

    pub fn update_entities(&mut self, dt: f64) {
        self.script_engine.update_entities(dt);
    }

    pub fn update_input(&mut self, input: &Input) {
        self.script_engine.call("update_input", &[
            ScriptValue::Bool(input.left_down),
            ScriptValue::Bool(input.up_down),
            ScriptValue::Bool(input.right_down),
            ScriptValue::Bool(input.down_down),
        ]);
    }

}

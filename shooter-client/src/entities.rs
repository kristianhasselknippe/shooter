use na::{Vector2,Vector3,Matrix4};
use texture::{Texture,TextureUnit};
use mesh::{Mesh};
use std::path::{Path};

use shader::ShaderProgram;
use super::input::Input;
use super::scripting::*;
use super::scripting::lua::LuaType;

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
        let script_engine = ScriptEngine::new();

        GameState {
            script_engine: script_engine,
        }
    }

    pub fn new_entity(&mut self, name: &str) -> EntityRef {
        let id = self.script_engine.add_entity(name);
        let ret = EntityRef(id as u32);
        ret
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
            LuaType::Bool(input.left_down),
            LuaType::Bool(input.up_down),
            LuaType::Bool(input.right_down),
            LuaType::Bool(input.down_down),
        ]).unwrap();
    }

}

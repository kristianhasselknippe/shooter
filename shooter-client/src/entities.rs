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

use super::scripting::hlua::{LuaFunction,LuaTable};

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
    pub pos: Vector3<f32>,
}

#[derive(Hash,Clone,Copy,Eq,PartialEq, Debug)]
pub struct EntityRef(u32);

impl Entity {
    pub fn new(pos: Vector2<f32>) -> Entity {
        Entity {
            pos: Vector3::new(pos.x, pos.y, 0.0),
        }
    }
}

pub struct GameState<'a> {
    script_engine: ScriptEngine<'a>,
}

impl<'a> GameState<'a> {
    pub fn new() -> GameState<'a> {
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
        //let o = self.script_engine.call_function("get_entity", &[ScriptValue::Number(entity_ref.0 as f64)]);
        let mut fun: LuaFunction<_> = self.script_engine.lua.get("get_entity").unwrap();
        let mut result: LuaTable<_> = fun.call_with_args((entity_ref.0)).unwrap();

        let mut x = 0.0;
        let mut y = 0.0;


        for (k, v) in result.iter::<String, f64>().filter_map(|e| e) {
            println!("{} => {}", k, v);
        }
        /*for &(ref k, ref v) in o.extract_field("position").extract_object_vec() {
            if k.extract_string() == "x" { x = v.extract_number() }
            if k.extract_string() == "y" { y = v.extract_number() }
        }*/

        println!("X: {}, Y: {}", x, y);

        return Entity {
            pos: Vector3::new(x as f32,y as f32, 0.0)
        }
    }

    pub fn update_entities(&mut self, dt: f64) {
        self.script_engine.update_entities(dt);
    }

    pub fn update_input(&mut self, input: &Input) {
        self.script_engine.call_function("update_input", &[
            ScriptValue::Bool(input.left_down),
            ScriptValue::Bool(input.up_down),
            ScriptValue::Bool(input.right_down),
            ScriptValue::Bool(input.down_down),
        ]);
    }

}

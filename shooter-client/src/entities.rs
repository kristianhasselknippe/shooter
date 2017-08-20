use na::{Vector2,Point3,Isometry3,Vector3,Matrix4};
use alga::linear::Transformation;
use texture::{Texture,TextureUnit};
use mesh::{Mesh};
use std::path::{Path};
use drawing::*;
use std::cell::RefCell;
use std::rc::Rc;
use shader::ShaderProgram;
use std::collections::HashMap;
use super::input::Input;

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

        let translation = Matrix4::new_translation(&self.pos);
        let scaling = Matrix4::new_nonuniform_scaling(&self.size);

        let model = translation * scaling;
        let mvp = model * camera_matrix;

        self.program.set_mat4("mvp", mvp);

        self.mesh.draw_now();
    }
}

pub struct Entity {
    pub pos: Vector2<f32>,
}

#[derive(Hash,Clone,Copy,Eq,PartialEq)]
pub struct EntityRef(u32);

impl Entity {
    pub fn new(pos: Vector2<f32>) -> Entity {
        Entity {
            pos: pos,
        }
    }
}

pub trait Component {
    fn update(&self, e: &mut Entity, dt: f32);
}

pub struct GameState {
    entities: HashMap<EntityRef,Entity>,
    components: HashMap<EntityRef, Vec<Box<Component>>>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            entities: HashMap::new(),
            components: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) -> EntityRef {
        let id = self.entities.len() as u32;
        let ret = EntityRef(id);
        self.entities.insert(ret, entity);
        ret
    }

    pub fn get_entity(&self, entity_ref: &EntityRef) -> &Entity {
        self.entities.get(entity_ref).unwrap()
    }

    pub fn add_component(&mut self, comp: Box<Component>, entity_ref: &EntityRef) {
        if !self.components.contains_key(entity_ref) {
            self.components.insert(*entity_ref, Vec::new());
        }
        let comps = self.components.get_mut(entity_ref).unwrap();
        comps.push(comp);
    }

    pub fn update(&mut self, dt: f32) {
        for (er, mut e) in &mut self.entities {
            let components = self.components.get_mut(&er).unwrap();
            for c in components {
                c.update(&mut e, dt);
            }
        }
    }
}

pub struct PlayerController {
    input: Rc<RefCell<Input>>,
}

impl PlayerController {
    pub fn new(input: &Rc<RefCell<Input>>) -> PlayerController {
        PlayerController {
            input: input.clone(),
        }
    }
}

impl Component for PlayerController {
    fn update(&self, entity: &mut Entity, dt: f32) {
        let input = self.input.borrow();
        if input.left_down {
            entity.pos.x -= 0.1 * dt;
        }
        if input.up_down {
            entity.pos.y += 0.1 * dt;
        }
        if input.right_down {
            entity.pos.x += 0.1 * dt;
        }
        if input.down_down {
            entity.pos.y -= 0.1 * dt;
        }

    }
}

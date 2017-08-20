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

        let translation = Matrix4::new_translation(&Vector3::new(-&self.pos.x, -self.pos.y, self.pos.z));
        let scaling = Matrix4::new_nonuniform_scaling(&self.size);

        let model = translation * scaling;
        let mvp = model * camera_matrix;

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


#[derive(Hash,Clone,Copy,Eq,PartialEq, Debug)]
pub struct ComponentRef(u32);

pub trait Component {
    fn update(&self, e: &mut Entity, dt: f32, game_state: &GameState);
}

struct Ownership {
    owning_entity: EntityRef,
    component: ComponentRef,
}

pub struct GameState {
    entities: HashMap<EntityRef,Rc<RefCell<Entity>>>,
    components: HashMap<ComponentRef, Rc<RefCell<Component>>>,

    relationships: Vec<Ownership>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            entities: HashMap::new(),
            components: HashMap::new(),

            relationships: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: &Rc<RefCell<Entity>>) -> EntityRef {
        let id = self.entities.len() as u32;
        let ret = EntityRef(id);
        self.entities.insert(ret, entity.clone());
        ret
    }

    pub fn get_entity(&self, entity_ref: &EntityRef) -> Ref<Entity> {
        self.entities.get(entity_ref).unwrap().borrow()
    }

    pub fn get_entity_mut(&self, entity_ref: &EntityRef) -> RefMut<Entity> {
        self.entities.get(entity_ref).unwrap().borrow_mut()
    }

    pub fn add_component<T: Component + 'static>(&mut self, comp: &Rc<RefCell<T>>, entity_ref: &EntityRef) -> ComponentRef {
        let id = self.components.len() as u32;
        let ret = ComponentRef(id);
        self.components.insert(ret, comp.clone());

        self.relationships.push(Ownership {
            owning_entity: entity_ref.clone(),
            component: ret.clone(),
        });

        ret
    }

    pub fn get_component(&self, comp_ref: &ComponentRef) -> Ref<Component> {
        self.components.get(comp_ref).unwrap().borrow()
    }

    pub fn update(&self, dt: f32) {
        for rel in &self.relationships {
            let c = self.get_component(&rel.component);
            let mut e = self.get_entity_mut(&rel.owning_entity);

            c.update(&mut e, dt, self);
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
    fn update(&self, entity: &mut Entity, dt: f32, game_state: &GameState) {
        let input_vector = self.input.borrow().normalized_input_vector();

        entity.pos.x += input_vector.x * dt;
        entity.pos.y += input_vector.y * dt;
    }
}

pub struct PlayerCamera {
    player: EntityRef,
    camera: Camera,
}

impl PlayerCamera {
    pub fn new(player: &EntityRef, camera: Camera) -> PlayerCamera {
        PlayerCamera {
            player: player.clone(),
            camera: camera,
        }
    }

    pub fn camera_matrix(&self, game_state: &GameState) -> Matrix4<f32> {
        self.camera.camera_matrix()
    }
}

impl Component for PlayerCamera {
    fn update(&self, entity: &mut Entity, dt: f32, game_state: &GameState) {
        let player_entity = game_state.get_entity(&self.player);
        let player_vec = player_entity.pos - entity.pos;

        entity.pos += dt * player_vec;


        println!("PlayerPos: {:?}, CamPos: {:?}", player_entity.pos, entity.pos);
    }
}

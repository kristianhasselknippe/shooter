use na::{Point3,Isometry3,Vector3,Matrix4};
use alga::linear::Transformation;
use texture::{Texture,TextureUnit};
use mesh::{Mesh};
use std::path::{Path};
use drawing::*;



use shader::ShaderProgram;

use std::rc::Rc;

pub struct Sprite {
    pos: Vector3<f32>,
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
}


impl Drawable for Sprite {
    fn draw(&self) {
        self.program.use_program();
        self.texture.bind(TextureUnit::Unit0);

        let translation = Matrix4::new_translation(&self.pos);
        let scaling = Matrix4::new_nonuniform_scaling(&self.size);

        self.mesh.draw_now();
    }
}

pub struct Scene {
    sprites: Vec<Rc<Sprite>>,
}


impl Scene {
    pub fn new() -> Scene {
        Scene {
            sprites: Vec::new()
        }
    }

    pub fn add_sprite(&mut self, sprite: &Rc<Sprite>) {
        self.sprites.push(sprite.clone());
    }
}

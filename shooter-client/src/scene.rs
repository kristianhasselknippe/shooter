use super::na::{Point2};
use super::texture::{Texture};
use super::mesh::{Mesh};
use std::path::{Path};


use std::rc::Rc;

pub struct Sprite {
    pos: Point2<f64>,
    texture: Texture,
    mesh: Mesh,
}

impl Sprite {
    pub fn from_png(path: &Path) -> Sprite {
        let t = Texture::from_png(path);
        Sprite {
            pos: Point2::new(0.0,0.0),
            texture: t,
            mesh: Mesh::create_quad(),
        }
    }

    pub fn bind(&self) {
        self.texture.bind();
        self.mesh.bind();
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

    pub fn bind(&self) {
        for s in &self.sprites {
            s.bind();
        }
    }

    pub fn add_sprite(&mut self, sprite: &Rc<Sprite>) {
        self.sprites.push(sprite.clone());
    }
}

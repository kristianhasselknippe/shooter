use na::{Vector3,Matrix4,Unit};
use texture::{Texture,TextureUnit};
use mesh::mesh::{GlMesh};
use std::path::{Path};
use drawing::*;

pub struct Sprite {
    pub pos: Vector3<f32>,
    pub rot: f32,
    size: Vector3<f32>,

    texture: Texture,

    mesh: GlMesh,
}

impl Sprite {
    pub fn from_png(path: &Path, w: f32, h: f32) -> Sprite {
        let t = Texture::from_png(path);

        Sprite {
            pos: Vector3::new(0.0,0.0,0.0),
            size: Vector3::new(w,h,1.0),

            texture: t,
            rot: 0.0,

            mesh: GlMesh::create_quad(),
        }
    }
}

impl Drawable for Sprite {
    fn draw(&self, dc: &DrawContext) {
        let shader_ref = dc.use_shader_program("sprite");
        
        self.texture.bind(TextureUnit::Unit0);

        let translation = Matrix4::new_translation(&Vector3::new(self.pos.x, self.pos.y, self.pos.z));
        let rotation = Matrix4::from_axis_angle(&Unit::new_normalize(Vector3::new(0.0,0.0,1.0)), self.rot);
        let scaling = Matrix4::new_nonuniform_scaling(&self.size);

        let model = translation * rotation * scaling;
        let mvp = dc.camera_matrix() * model;

        shader_ref.set_mat4("mvp", mvp);
        shader_ref.set_float("rotation", self.rot);

        self.mesh.draw(dc);
    }
}

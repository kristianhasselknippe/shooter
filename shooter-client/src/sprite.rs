use na::{Vector2,Vector3,Matrix4,Unit};
use texture::{Texture,TextureUnit};
use mesh::{Mesh};
use std::path::{Path};
use shader::ShaderProgram;

pub struct Sprite {
    pub pos: Vector3<f32>,
    pub rot: f32,
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
            rot: 0.0,

            program: program,
            mesh: Mesh::create_quad(),
        }
    }

    pub fn draw(&self, camera_matrix: &Matrix4<f32>) {
        self.program.use_program();
        self.texture.bind(TextureUnit::Unit0);

        let translation = Matrix4::new_translation(&Vector3::new(self.pos.x, self.pos.y, self.pos.z));
        let rotation = Matrix4::from_axis_angle(&Unit::new_normalize(Vector3::new(0.0,0.0,1.0)), self.rot);
        let scaling = Matrix4::new_nonuniform_scaling(&self.size);

        let model = translation * rotation * scaling;
        let mvp = camera_matrix * model;

        self.program.set_mat4("mvp", mvp);
        self.program.set_float("rotation", self.rot);

        self.mesh.draw_now();
    }
}
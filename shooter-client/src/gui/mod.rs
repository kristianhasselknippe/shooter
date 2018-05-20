use camera::OrthoCamera;
use gl;
use gl::types::*;
use imgui::*;
use input::Input;
use na::{Vector2,Vector3,Vector4};
use shader::ShaderProgram;
use drawing::{Color3,Color4};
use std::os::raw::c_char;
use std::ptr::{null_mut,null};
use utils::gl::{*, texture::*};
use glutin::*;

macro_rules! cstr {
    ($i:ident) => {
        format!("{}{}", $i, "\0").as_ptr() as *const c_char
    };
    ($s:expr) => {
        concat!($s, "\0") as *const str as *const [c_char] as *const c_char
    };
}

pub struct Gui {
    context: *mut ImGuiContext,
    io: *mut ImGuiIO,

    shader: ShaderProgram,
}

impl Gui {
    pub fn init_gui(w: f32, h: f32) -> Gui {
        let shader = ShaderProgram::create_program("gui");

        unsafe {
            let ctx = igCreateContext(None, None);
            let io = igGetIO();
            (*io).display_size.x = w;
            (*io).display_size.y = h;

            ImFontAtlas_AddFontFromFileTTF(
                (*io).fonts,
                cstr!("./assets/fonts/Lato-Regular.ttf"),
                15.0,
                null_mut(),
                null_mut(),
            );

            (*io).key_map[ImGuiKey::Backspace as usize] = VirtualKeyCode::Back as _;
            (*io).key_map[ImGuiKey::Escape as usize] = VirtualKeyCode::Escape as _;
            (*io).key_map[ImGuiKey::Enter as usize] = VirtualKeyCode::Return as _;

            let mut pixels: *mut u8 = null_mut();
            let mut width: i32 = 0;
            let mut height: i32 = 0;
            let mut bytes_per_pixel: i32 = 0;

            ImFontAtlas_GetTexDataAsRGBA32(
                (*io).fonts,
                &mut pixels as *mut *mut u8,
                &mut width as *mut i32,
                &mut height as *mut i32,
                &mut bytes_per_pixel as *mut i32,
            );

            let mut font_texture = Texture::new();
            font_texture.bind_to_texture_unit(0);
            font_texture.upload(pixels, width as u32, height as u32, bytes_per_pixel);

            ImFontAtlas_SetTexID((*io).fonts, font_texture.handle as _);

            Gui {
                context: ctx,
                io: io,
                shader: shader,
            }
        }
    }

    pub fn new_frame(&mut self) {
        unsafe {
            igNewFrame();
        }
    }

    pub fn render(&mut self, w: f32, h: f32) {
        unsafe {
            igRender();
            self.my_render_function(igGetDrawData(), w, h);
        }
    }

    pub fn update_input(&mut self, input: &Input, dt: f32) {
        unsafe {
            (*self.io).delta_time = dt;
            (*self.io).mouse_pos = ImVec2::new(input.mouse_pos.x, input.mouse_pos.y);
            (*self.io).mouse_down[0] = input.mouse_left;
            (*self.io).mouse_down[1] = input.mouse_right;
            (*self.io).keys_down[VirtualKeyCode::Back as usize] = input.backspace;
            (*self.io).keys_down[VirtualKeyCode::Escape as usize] = input.escape;
            (*self.io).keys_down[VirtualKeyCode::Return as usize] = input.backspace;
        }
    }

    pub fn add_input_character(&mut self, codepoint: char) {
        unsafe {
            ImGuiIO_AddInputCharacter(codepoint as _);
        }
    }

    pub fn draw_test(&mut self) {
        unsafe {
            igText(cstr!("Hello, world"));
            igText(cstr!("Hello, world 2"));
            igText(cstr!("Hello, world 3"));
            if igButton(cstr!("Save"), ImVec2::new(150.0, 80.0)) {}
            //igInputText(cstr("string"), buf, IM_ARRAYSIZE(buf));
            //igSliderFloat("float", &f, 0.0f, 1.0f);
        }
    }

    pub fn begin(&mut self, name: &str, mut open: bool) -> bool {
        unsafe { igBegin(cstr!(name), &mut open as *mut bool, ImGuiWindowFlags::empty()) }
    }

    pub fn end(&mut self) {
        unsafe { igEnd(); }
    }

    pub fn slider_float(&mut self, label: &str, value: &mut f32, min: f32, max: f32) -> bool {
        unsafe {
            igSliderFloat(cstr!(label), value as *mut _, min, max, null(), 1.0)
        }
    }

    pub fn drag_float(&mut self, label: &str, value: &mut f32, speed: f32, min: f32, max: f32) -> bool {
        unsafe { igDragFloat(cstr!(label), value as *mut _, speed, min, max, null(), 1.0) }
    }

    pub fn drag_float2(&mut self, label: &str, value: &mut Vector2<f32>, speed: f32, min: f32, max: f32) -> bool {
        unsafe { igDragFloat2(cstr!(label), value.as_mut_slice().as_mut_ptr() as *mut _, speed, min, max, null(), 1.0) }
    }

    pub fn drag_float3(&mut self, label: &str, value: &mut Vector3<f32>, speed: f32, min: f32, max: f32) -> bool {
        unsafe { igDragFloat3(cstr!(label), value.as_mut_slice().as_mut_ptr() as *mut _, speed, min, max, null(), 1.0) }
    }

    pub fn drag_float4(&mut self, label: &str, value: &mut Vector4<f32>, speed: f32, min: f32, max: f32) -> bool {
        unsafe { igDragFloat4(cstr!(label), value.as_mut_slice().as_mut_ptr() as *mut _, speed, min, max, null(), 1.0) }
    }

    pub fn color_edit3(&mut self, label: &str, value: &mut Color3) -> bool {
        unsafe { igColorEdit3(cstr!(label), value.as_mut_slice().as_mut_ptr() as *mut _, ImGuiColorEditFlags::empty()) }
    }

    pub fn color_edit4(&mut self, label: &str, value: &mut Color4) -> bool {
        unsafe { igColorEdit4(cstr!(label), value.as_mut_slice().as_mut_ptr() as *mut _, ImGuiColorEditFlags::empty()) }
    }

    pub fn same_line(&mut self, x: f32, spacing: f32) {
        unsafe { igSameLine(x as _, spacing as _); }
    }

    pub fn text(&mut self, text: &str) {
        unsafe { igText(cstr!(text)) };
    }

    pub fn button(&mut self, text: &str, width: f32, height: f32) -> bool {
        unsafe { igButton(cstr!(text), ImVec2::new(width, height)) }
    }

    pub fn input_text(&mut self, label: &str, buf: &mut String) -> bool {
        unsafe {
            igInputText(
                cstr!(label),
                buf.as_ptr() as *mut c_char,
                buf.len(),
                ImGuiInputTextFlags::empty(),
                None,
                null_mut(),
            )
        }
    }

    fn my_render_function(&mut self, draw_data: *mut ImDrawData, w: f32, h: f32) {
        unsafe {
            for &cmd_list in (*draw_data).cmd_lists() {
                let mut idx_buffer = (*cmd_list).idx_buffer.data; // index buffer generated by ImGui
                for cmd_i in 0..(*cmd_list).cmd_buffer.size {
                    let pcmd = &(*cmd_list).cmd_buffer.as_slice()[cmd_i as usize];
                    if let Some(user_callback) = (*pcmd).user_callback {
                        user_callback(cmd_list, pcmd);
                    } else {
                        let tid = pcmd.texture_id;

                        //TODO: MyEngineScissor((int)pcmd->ClipRect.x, (int)pcmd->ClipRect.y, (int)(pcmd->ClipRect.z - pcmd->ClipRect.x), (int)(pcmd->ClipRect.w - pcmd->ClipRect.y));
                        self.draw_indexed_triangles(
                            if ::std::mem::size_of::<ImDrawIdx>() == 2 {
                                gl::UNSIGNED_SHORT
                            } else {
                                gl::UNSIGNED_INT
                            },
                            &(*cmd_list).idx_buffer,
                            &(*cmd_list).vtx_buffer,
                            tid as _,
                            w,
                            h,
                        );
                    }
                    idx_buffer.add((*pcmd).elem_count as usize);
                }
            }
        }
    }

    fn draw_indexed_triangles(
        &mut self,
        element_type: GLenum,
        elements: &ImVector<ImDrawIdx>,
        vertices: &ImVector<ImDrawVert>,
        texture_handle: u32,
        w: f32,
        h: f32,
    ) {
        let mut vao = gen_vertex_array();
        vao.bind();

        let mut vbo = Buffer::gen_vbo();
        vbo.bind();
        let mut ebo = Buffer::gen_ebo();
        ebo.bind();

        let camera = OrthoCamera::new(w, h);

        disable(Capability::CullFace);
        disable(Capability::DepthTest);
        enable(Capability::Blend);
        //enable(Capability::ScissorTest);

        enable_vertex_attribs(&[
            VertexAttribute::new(0, gl::FLOAT, 2, false),
            VertexAttribute::new(1, gl::FLOAT, 2, false),
            VertexAttribute::new(2, gl::UNSIGNED_BYTE, 4, true),
        ]);

        bind_texture_unit(0, texture_handle);

        let verts_size = vertices.size * ::std::mem::size_of::<ImDrawVert>() as i32;
        let elems_size = elements.size * ::std::mem::size_of::<ImDrawIdx>() as i32;
        vbo.upload_data(vertices.data as _, verts_size as isize);
        ebo.upload_data(elements.data as _, elems_size as isize);

        self.shader.use_program();
        self.shader.set_mat4("proj", &camera.projection);
        self.shader.set_int("tex", 0);

        draw_triangles(elems_size, element_type);

        vbo.delete();
        ebo.delete();
        vao.delete();
    }

    pub fn set_display_size(&mut self, size: (f32, f32)) {
        unsafe {
            let newSize = ImVec2::new(size.0, size.1);
            (*self.io).display_size.x = newSize.x;
            (*self.io).display_size.y = newSize.y;
            igSetNextWindowSize(newSize, ImGuiCond::Always);
        }
    }
}

impl Drop for Gui {
    fn drop(&mut self) {
        unsafe {
            igDestroyContext(self.context);
        }
    }
}

use gl::types::*;
use gl;
use utils::gl::{
    texture::*,
    *
};
use drawing::*;
use input::Input;
use imgui::*;
use shader::ShaderProgram;
use camera::Camera;
use std::ptr::{null, null_mut};
use std::os::raw::c_char;
use na::Point3;

macro_rules! cstr {
    ($s:expr) => (
        concat!($s, "\0") as *const str as *const [c_char] as *const c_char
    );
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
            // Application init
            let ctx = igCreateContext(None, None);
            let io = igGetIO();
            (*io).display_size.x = w;
            (*io).display_size.y = h;

            let mut pixels: *mut u8 = null_mut();
            let mut width: i32 = 0;
            let mut height: i32 = 0;
            let mut bytes_per_pixel: i32 = 0;

            ImFontAtlas_GetTexDataAsRGBA32((*io).fonts, &mut pixels as *mut *mut u8, width as *mut i32, &mut height as *mut i32, &mut bytes_per_pixel as *mut i32);
            println!("Pixels adr: {}, width: {}, height: {}, bbp: {}", pixels as i32, width, height, bytes_per_pixel);

            let my_texture = Texture::new();
            //upload

            // TODO: Store your texture pointer/identifier (whatever your engine uses) in 'io.Fonts->TexID'. This will be passed back to your via the renderer.

            ImFontAtlas_SetTexID((*io).fonts, 2 as _); //TODO: This should be the ID of the texture
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

    pub fn render(&mut self) {
        unsafe {
            // Render & swap video buffers
            igRender();
            /*MyImGuiRenderFunction(ImGui::GetDrawData());
            SwapBuffers();*/
            self.my_render_function(igGetDrawData());
        }
    }

    pub fn update_input(&mut self, input: &Input) {
        unsafe {
            /*ImGuiIO& io = ImGui::GetIO();
            io.DeltaTime = 1.0f/60.0f;
            io.MousePos = mouse_pos;
            io.MouseDown[0] = mouse_button_0;
            io.MouseDown[1] = mouse_button_1;*/
        }
    }

    pub fn draw_test(&mut self) {
        unsafe {
            igText(cstr!("Hello, world"));
            if igButton(cstr!("Save"), ImVec2::new(20.0,10.0))
            {

            }
            //igInputText(cstr("string"), buf, IM_ARRAYSIZE(buf));
            //igSliderFloat("float", &f, 0.0f, 1.0f);
        }
    }

    fn draw_indexed_triangles(&mut self, element_type: GLenum, elements: &ImVector<ImDrawIdx>, vertices: &ImVector<ImDrawVert>) {
        let mut vao = gen_vertex_array();
        vao.bind();

        let mut vbo = Buffer::gen_vbo();
        vbo.bind();
        let mut ebo = Buffer::gen_ebo();
        ebo.bind();

        let camera = Camera::new_orthographic(800.0, 600.0, Point3::new(0.0,0.0,0.0));

        disable(Capability::CullFace);
        disable(Capability::DepthTest);
        enable(Capability::Blend);
        enable(Capability::ScissorTest);

        println!("Enabling vertex attribs");
        enable_vertex_attribs(&[
            VertexAttribute::new(0, gl::FLOAT, 2),
            VertexAttribute::new(1, gl::FLOAT, 2),
            VertexAttribute::new(2, gl::UNSIGNED_INT, 1),
        ]);

        let verts_size = vertices.size * ::std::mem::size_of::<ImDrawVert>() as i32;
        let elems_size = elements.size * ::std::mem::size_of::<ImDrawIdx>() as i32;
        vbo.upload_data(vertices.data as _, verts_size as isize);
        ebo.upload_data(elements.data as _, elems_size as isize);

        /*unsafe {
            for i in 0..vertices.size {
                let v = vertices.data.offset(i as isize);
                println!("Vert: {},{}", (*v).pos.x, (*v).pos.y);
            }
        }*/

        self.shader.use_program();
        self.shader.set_mat4("proj", &camera.projection);

        println!("Drawing triangles: {}", elems_size);
        draw_triangles(elems_size, element_type);
        println!("done drawing triangles");

        vbo.delete();
        ebo.delete();
        vao.delete();
    }

    fn my_render_function(&mut self, draw_data: *mut ImDrawData)
    {
        unsafe {

            // TODO: Setup render state: alpha-blending enabled, no face culling, no depth testing, scissor enabled
            // TODO: Setup viewport, orthographic projection matrix
            // TODO: Setup shader: vertex { float2 pos, float2 uv, u32 color }, fragment shader sample color from 1 texture, multiply by vertex color.

            let cmd_lists_count = (*draw_data).cmd_lists_count;
            println!("DrawData: {}", cmd_lists_count);
            for &cmd_list in (*draw_data).cmd_lists()
            {
                let vtx_buffer = (*cmd_list).vtx_buffer.data;  // vertex buffer generated by ImGui
                let mut idx_buffer = (*cmd_list).idx_buffer.data;   // index buffer generated by ImGui
                for cmd_i in 0..(*cmd_list).cmd_buffer.size
                {
                    let pcmd = &(*cmd_list).cmd_buffer.as_slice()[cmd_i as usize];
                    if let Some(user_callback) = (*pcmd).user_callback
                    {
                        user_callback(cmd_list, pcmd);
                    }
                    else
                    {
                        // The texture for the draw call is specified by pcmd->TextureId.
                        // The vast majority of draw calls with use the imgui texture atlas, which value you have set yourself during initialization.
                        //TODO: bind_texture((*pcmd).texture_id);
                        bind_texture_unit(0,0);
                        // We are using scissoring to clip some objects. All low-level graphics API supports it.
                        // If your engine doesn't support scissoring yet, you may ignore this at first. You will get some small glitches
                        // (some elements visible outside their bounds) but you can fix that once everywhere else works!
                        //TODO: MyEngineScissor((int)pcmd->ClipRect.x, (int)pcmd->ClipRect.y, (int)(pcmd->ClipRect.z - pcmd->ClipRect.x), (int)(pcmd->ClipRect.w - pcmd->ClipRect.y));
                        // Render 'pcmd->ElemCount/3' indexed triangles.
                        // By default the indices ImDrawIdx are 16-bits, you can change them to 32-bits if your engine doesn't support 16-bits indices.
                        self.draw_indexed_triangles(
                            if ::std::mem::size_of::<ImDrawIdx>() == 2 { gl::UNSIGNED_SHORT } else { gl::UNSIGNED_INT },
                            &(*cmd_list).idx_buffer,
                            &(*cmd_list).vtx_buffer,
                        );
                    }
                    idx_buffer.add((*pcmd).elem_count as usize);
                }
            }
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


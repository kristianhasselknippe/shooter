use camera::*;
use drawing::*;
use fps_counter::*;
use glm::*;
use gui::imgui::*;
use input::*;
use mesh::model::*;
use scene::*;
use shader::*;
use time::*;

pub struct EngineContext {
    pub camera: Camera,
    pub program: ShaderProgram,
    pub game_objects: Vec<GameObject>,
    pub time: Time,
    pub fps_counter: FpsCounter,
    pub input: Input,
    pub gui: Gui,
}

pub fn init_defaults(window_size: (i32, i32), dpi_factor: f32) -> EngineContext {
    let mut camera =
        Camera::new_perspective(16.0 / 9.0, 3.14 / 4.0, 1.0, 1000.0, vec3(0.0, 0.0, 8.0));

    let mut program = ShaderProgram::create_program("default");

    let bow = GameObject::new(
        "Bow",
        Model::load_from_wavefront_file("Bow/Bow.obj").unwrap(),
        vec3(0.0, 0.0, 0.0),
    );
    let bow2 = GameObject::new(
        "Bow2",
        Model::load_from_wavefront_file("Bow2/Bow.obj").unwrap(),
        vec3(40.0, 0.0, 0.0),
    );

    let mut game_objects = vec![bow, bow2];

    let mut time = Time::new(60);

    let mut fps_counter = FpsCounter::new();

    let mut input = Input::new();

    let mut gui = Gui::new(
        window_size.0 as f32 * dpi_factor,
        window_size.1 as f32 * dpi_factor,
    );

    EngineContext {
        camera: camera,
        program: program,
        game_objects: game_objects,
        time: time,
        fps_counter: fps_counter,
        input: input,
        gui: gui,
    }
}

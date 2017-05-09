extern crate glfw;
use glfw::{Action, Context, Key};

extern crate gl;
use gl::types::*;
use std::mem;

use std::net::UdpSocket;

fn main() {
    /*let socket = UdpSocket::bind("127.0.0.1:12346").unwrap();

    socket.connect("127.0.0.1:12345");

    let to_send = b"foobar yo";
    socket.send(to_send);*/

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = glfw.create_window(600,800, "Shooter", glfw::WindowMode::Windowed).expect("failed to create window");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| window.get_proc_address(s) as *const GLvoid);

    let vertices: [GLfloat;9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0,  0.5, 0.0
    ];

    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        //glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);
        gl::BufferData(gl::ARRAY_BUFFER, (mem::size_of::<GLfloat>() * vertices.len()) as isize, mem::transmute(&vertices), gl::STATIC_DRAW);
    };


    while !window.should_close() {
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                },
                _ => (),
            }
        }
    }


}

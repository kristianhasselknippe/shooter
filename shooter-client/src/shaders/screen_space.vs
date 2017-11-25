#version 330 core

uniform vec2 screen_size;

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 tex_coord;

out vec2 TexCoord;

void main()
{
    TexCoord = tex_coord;
    gl_Position = vec4(position.x / screen_size.x, position.y / screen_size.y, position.z, 1.0);
}

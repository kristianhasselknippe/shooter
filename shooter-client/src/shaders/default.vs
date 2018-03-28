#version 330 core

layout (location = 0) in vec3 position;
//layout (location = 1) in vec2 tex_coord;

uniform mat4 mvp;

out vec2 TexCoord;

void main()
{
    TexCoord = vec2(position.x, position.y);
    gl_Position = /*mvp */ vec4(position.x * 0.6, position.y * 0.6, 0.0, 0.0);
}

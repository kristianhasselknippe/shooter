#version 330 core

uniform vec2 screen_size;
uniform vec2 shape_size;

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 tex_coord;

out vec2 TexCoord;

void main()
{
    TexCoord = tex_coord;
    float x = position.x / screen_size.x;
    float y = position.y / screen_size.y;

    x -= 1.0;
    y += 1.0;
    y -=  (shape_size.y / screen_size.y);
    
    gl_Position = vec4(x, y, position.z, 1.0);
}

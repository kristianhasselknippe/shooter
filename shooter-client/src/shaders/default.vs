#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;

uniform mat4 mvp;

out vec2 TexCoord;
out vec3 norm;

void main()
{
    TexCoord = vec2(position.x, position.y);
    norm = (mvp * vec4(normal, 1.0)).xyz;
    gl_Position = mvp * vec4(position.xyz, 1.0);
}

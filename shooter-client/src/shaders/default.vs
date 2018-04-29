#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;

uniform mat4 mvp;
uniform mat4 view;
uniform mat4 m; //Model

out vec2 TexCoord;
out vec3 n;
out vec3 p;

void main()
{
    TexCoord = vec2(position.x, position.y);
    n = normalize((view * m * vec4(normal, 1.0)).xyz);
    gl_Position = mvp * vec4(position, 1.0);
	p = (view * m * vec4(position,1.0)).xyz;
}

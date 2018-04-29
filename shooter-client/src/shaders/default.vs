#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform mat4 mvp;
uniform mat3 inverseTranspose;

out vec2 TexCoord;
out vec3 n;
out vec3 p;

void main()
{
	TexCoord = vec2(position.x, position.y);
	n = normalize(inverseTranspose * normal);
	//n = normalize((mvp * vec4(normal, 1.0)).xyz);
	p = (mvp * vec4(position,0.0)).xyz;

	gl_Position = mvp * vec4(position, 1.0);
}

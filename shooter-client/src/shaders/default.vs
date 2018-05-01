#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform mat3 mv_inv;

out vec3 n;
out vec3 p;

void main()
{
	mat4 viewModel = view * model;

	n = mv_inv * normal;
	p = (view * model * vec4(position, 1.0)).xyz;

	gl_Position = projection * view * model * vec4(position, 1.0);
}

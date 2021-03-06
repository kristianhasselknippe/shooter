#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec3 tex_coords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform mat3 mv_inv;

out vec3 n;
out vec3 p;
out vec2 texCoords;

void main()
{
	n = mv_inv * normal;
	p = (view * model * vec4(position, 1.0)).xyz;
	texCoords = tex_coords.xy;

	gl_Position = projection * view * model * vec4(position, 1.0);
}

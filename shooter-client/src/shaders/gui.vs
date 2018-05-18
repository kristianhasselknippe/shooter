#version 330 core

layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 uv;
layout (location = 2) in vec4 color;

uniform mat4 proj;

varying vec2 tex_coords;
varying vec4 v_color;

void main()
{
	v_color = color;
	tex_coords = uv;
	gl_Position = proj * vec4(pos.xy, 0.0, 1.0);
}

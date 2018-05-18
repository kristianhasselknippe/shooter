#version 330 core

layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 uv;
layout (location = 2) in uint color;

uniform mat4 proj;

out vec2 tex_coords;
out uint v_color;

void main()
{
	v_color = color;
	tex_coords = uv;
	gl_Position = proj * vec4(pos.xy, 0.0, 1.0);
}

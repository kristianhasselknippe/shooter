#version 330 core

in vec2 tex_coords;
in vec4 v_color;

uniform sampler2D tex;

out vec4 color;

void main() {
	color = v_color
		* texture(tex, tex_coords);
}

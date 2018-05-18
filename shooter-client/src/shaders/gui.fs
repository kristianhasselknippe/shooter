#version 330 core

varying vec2 tex_coords;
varying vec4 v_color;

uniform sampler2D tex;

out vec4 color;

void main() {
	color = v_color
		* texture(tex, tex_coords);
}

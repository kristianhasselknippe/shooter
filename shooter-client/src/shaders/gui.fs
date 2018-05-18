#version 330 core

in vec2 tex_coords;
in uint v_color;

uniform sampler2D tex;

out vec4 color;

void main() {
	color = texture(tex, tex_coords);
}

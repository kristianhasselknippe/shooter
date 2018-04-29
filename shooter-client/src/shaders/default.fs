#version 330 core

//uniform sampler2D tex0;

in vec2 TexCoord;
in vec3 n;
in vec3 p;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform mat4 mvp;

uniform mat4 viewPosition;


const vec3 diffuseColor = vec3(0.3,2.0,0.3); //Diffuse

const vec3 l = vec3(10,10,10);

out vec4 color;

void main() {
	vec3 lightPos = (mvp * vec4(l, 1.0)).xyz;
	vec3 lightVector = normalize(lightPos - p);
	float diffuse = clamp(dot(n, lightVector), 0.0, 1.0);
	vec3 Lo = vec3(diffuse);

	color = vec4(Lo, 1.0);
}

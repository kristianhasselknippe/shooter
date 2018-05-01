#version 330 core

in vec3 n;
in vec3 p;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

const vec3 diffuseColor = vec3(0.3,2.0,0.3); //Diffuse
const vec3 lightPosWorld = vec3(10,10,-10);//In world space..?

out vec4 color;

void main() {
	mat4 viewModel = view * model;
	vec3 normal = normalize(n);

	vec3 lightPosView = (view * vec4(lightPosWorld,1.0)).xyz;
	vec3 lightDir = normalize(lightPosView - p);

	float diffuse = clamp(dot(normal, lightDir), 0.0, 1.0);
	vec3 Lo = vec3(diffuse);

	color = vec4(Lo, 1.0);
}

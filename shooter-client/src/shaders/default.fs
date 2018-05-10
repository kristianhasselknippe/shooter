#version 330 core

in vec3 n;
in vec3 p;
in vec2 texCoords;

layout(binding = 0) uniform sampler2D diffuseMap;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

const float ambientAmount = 0.1;

const float lightIntensity = 4.0;
const vec3 ambientColor = vec3(1.0,1.0,1.0);
const vec3 diffuseColor = vec3(1.0,1.0,1.0); //Diffuse
const vec3 specularColor = vec3(1.0,0.5,0.5); //Specular
const vec3 lightPosWorld = vec3(5,10,15);//In world space..?

const float smoothness = 10.0;

out vec4 color;

void main() {
	vec3 viewDir = normalize(-p); //since we are in view space
	vec3 normal = normalize(n);

	vec3 lightPosView = (view * vec4(lightPosWorld,1.0)).xyz;
	vec3 lightDir = normalize(lightPosView - p);

	//specular
	vec3 halfVec = normalize(lightDir + viewDir);
	float h = clamp(dot(normal, halfVec), 0.0, 1.0);
	float specAmount = pow(h, smoothness);

	//diffuse
	float lightDist = length(lightPosView - p);
	float attenuation = 1.0 / (1.0 + 0.1*lightDist + 0.01*pow(lightDist,2));

	float diffuseAmount = clamp(dot(lightDir, normal), 0.0, 1.0);

	//diffuseColor
	//have to do 1-texCoords.y since obj and glsl disagree on the origin
	vec3 diffuse = texture(diffuseMap, vec2(texCoords.x, 1.0-texCoords.y)).xyz * diffuseAmount;
	vec3 specular = ((smoothness + 8.0) / (8.0 * 3.14)) * specAmount * specularColor;
	vec3 ambient = ambientAmount * ambientColor;
	vec3 Lo = ambient + (diffuse + specular) * attenuation * lightIntensity;

	color = vec4(Lo, 1.0); 
	//color = texture(diffuseMap, vec2(texCoords.x, 1.0 - texCoords.y));
	//color = vec4(texCoords,0.0,1.0);
}

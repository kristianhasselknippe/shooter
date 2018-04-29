#version 330 core

//uniform sampler2D tex0;

in vec2 TexCoord;
in vec3 n;
in vec3 p;

uniform mat4 m;
uniform mat4 view; //view
uniform mat4 vp; //view projection
uniform vec3 pv; //View position
const vec3 Kd = vec3(0.3,2.0,0.3); //Diffuse
const vec3 Ks = vec3(1.0,0.6,0.6);//Specular
const int lightCount = 1;

const int MAXLIGHTS = 1;
const vec3 l = vec3(10,10,10); //Light in world space
//const vec3 EL[MAXLIGHTS] = vec3[](vec3(0,1,1));

out vec4 color;

void main() {
	vec3 viewPos = (vp * vec4(pv, 1.0)).xyz;
	vec3 v = normalize(viewPos - p); //View ray
	vec3 Lo = vec3(0.0f, 0.0f, 0.0f); //Outgoing radiance

	vec3 lightVector = normalize((view * vec4(p - l,1.0)).xyz);
	//vec3 h = normalize(v + light); //Half between view ray and light ray
	//float cosTh = saturate(dot(n,h));
	float cosTi = clamp(dot(n, lightVector), 0.0, 1.0); //Diffuse part
	Lo = vec3(cosTi);// * Kd;
	//Lo += (Kd + Ks * pow(cosTh, m)) * elight * cosTi;

	color = vec4(Lo, 1.0);
}

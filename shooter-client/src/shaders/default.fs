#version 330 core

//uniform sampler2D tex0;

in vec2 TexCoord;
in vec3 norm;

out vec4 color;

void main() {
    float angle = dot(norm, vec3(1.0,-1.0,0.0));
    float ambient = (angle + 1.0) / 2.0;
    //color = vec4(vec3(1.0,0.0,0.0) * ambient,1.0);
    color = vec4(norm.x,0.0,0.0,1.0);
}

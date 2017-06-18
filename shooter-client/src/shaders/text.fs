#version 330 core

uniform sampler2D tex0;

in vec2 TexCoord;
out vec4 color;

void main()
{
    float x = texture(tex0,TexCoord).x;
    color = vec4(1.0-x,1.0-x,1.0-x, 1.0);
}

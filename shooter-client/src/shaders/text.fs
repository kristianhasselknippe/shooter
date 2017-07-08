#version 330 core

uniform sampler2D font;

in vec2 TexCoord;
out vec4 color;

void main()
{
    float x = texture(font,TexCoord).r;
    color = vec4(x,x,x, 1.0);
}

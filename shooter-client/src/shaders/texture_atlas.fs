#version 330 core

uniform sampler2D tex0;

uniform vec3 spriteColor;

in vec2 TexCoord;

out vec4 color;

void main()
{
    color = texture(tex0, TexCoord);
    //color = vec4(spriteColor, 1.0);
}

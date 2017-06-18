#version 330 core

uniform sampler2D tex0;

uniform vec3 spriteColor;

in vec2 TexCoord;

out vec4 color;

void main()
{
    //color = texture(tex0, TexCoord); //vec4(TexCoord.x, TexCoord.y, 0.0,1.0);
    color = vec4(spriteColor, 1.0);
}

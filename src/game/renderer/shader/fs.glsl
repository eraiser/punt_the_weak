#version 440

in vec2 UV;

out vec3 color;

uniform sampler2D TextureSampler;

void main()
{
	color = texture( myTextureSampler, UV ).rgb;
}
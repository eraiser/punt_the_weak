#version 440

in vec2 UV;

out vec3 color;

uniform sampler2D myTextureSampler;

void main()
{

    // Output color = red
    //color = fragmentColor;
	color = texture( myTextureSampler, UV ).bgr;

}
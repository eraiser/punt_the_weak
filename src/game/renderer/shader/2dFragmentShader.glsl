#version 440

// Interpolated values from the vertex shaders
in vec2 UV;

// Ouput data
out vec4 color;

// Values that stay constant for the whole mesh.
uniform sampler2D myTextureSampler;
uniform float level;

void main(){
	color = texture( myTextureSampler, UV ).rgba;
}
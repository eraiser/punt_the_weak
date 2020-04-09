#version 440

// Input vertex data, different for all executions of this shader.
layout(location = 0) in vec2 vertex_uniformed;
layout(location = 1) in vec2 vertexUV;

// Output data ; will be interpolated for each fragment.
out vec2 UV;

uniform float level;
uniform vec2 dimensions;
uniform vec2 offset_screenspace;
uniform mat4 Orthogonal_matrix;

void main(){
	vec2 vertexPosition_screenspace = vec2(vertex_uniformed.x * dimensions.x, vertex_uniformed.y * dimensions.y);// [0..800][0..600] -> [-400..400][-300..300]
	vertexPosition_screenspace+=offset_screenspace;

	gl_Position =  Orthogonal_matrix * vec4(vertexPosition_screenspace,0,1);
	gl_Position.z = level;
	UV = vertexUV;
}


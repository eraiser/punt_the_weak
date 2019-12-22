#version 440

// Input vertex data, different for all executions of this shader.
layout(location = 0) in vec2 vertex_uniformed;
layout(location = 1) in vec2 vertexUV;

// Output data ; will be interpolated for each fragment.
out vec2 UV;

uniform float scale2d;
uniform vec2 offset_screenspace;
uniform vec2 window_dimensions;
uniform mat4 O;

void main(){
	vec2 vertexPosition_screenspace = vertex_uniformed * scale2d;// [0..800][0..600] -> [-400..400][-300..300]
	vertexPosition_screenspace+=offset_screenspace;

	gl_Position =  O * vec4(vertexPosition_screenspace,0,1);
	UV = vertexUV;
}


#version 440

const int MAX_LIGHTS = 4;
// Input vertex data, different for all executions of this shader.
layout(location = 0) in vec3 vertexPosition_modelspace;
layout(location = 1) in vec3 vertexNormal_modelspace;
layout(location = 2) in vec2 vertexUV;


// Output data ; will be interpolated for each fragment.
out vec2 UV;
out vec3 Position_worldspace;
out vec3 Normal_cameraspace;
out vec3 EyeDirection_cameraspace;
out vec3 LightDirection_cameraspace[MAX_LIGHTS];

// Values that stay constant for the whole mesh.
uniform mat4 MVP;
uniform mat4 view_matrix;
uniform mat4 model_matrix;
uniform vec3 LightPositions_worldspace[4];

void main(){

	// Output position of the vertex, in clip space : MVP * position
	gl_Position =  MVP * vec4(vertexPosition_modelspace,1);
	
	// Position of the vertex, in worldspace : M * position
	Position_worldspace = (model_matrix * vec4(vertexPosition_modelspace,1)).xyz;
	
	// Vector that goes from the vertex to the camera, in camera space.
	// In camera space, the camera is at the origin (0,0,0).
	vec3 vertexPosition_cameraspace = ( view_matrix * model_matrix * vec4(vertexPosition_modelspace,1)).xyz;
	EyeDirection_cameraspace = vec3(0,0,0) - vertexPosition_cameraspace;

	// Vector that goes from the vertex to the light, in camera space. M is ommited because it's identity.
	for(int i=0 ; i<MAX_LIGHTS ; i++) {
		vec3 LightPosition_cameraspace = ( view_matrix * vec4(LightPositions_worldspace[i],1)).xyz;
		LightDirection_cameraspace[i] = LightPosition_cameraspace + EyeDirection_cameraspace;
	}
	
	// Normal of the the vertex, in camera space
	Normal_cameraspace = ( view_matrix * model_matrix * vec4(vertexNormal_modelspace,0)).xyz; // Only correct if ModelMatrix does not scale the model_transform ! Use its inverse transpose if not.
	
	// UV of the vertex. No special space for this one.
	UV = vertexUV;
}


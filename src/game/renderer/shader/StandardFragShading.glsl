#version 440

const int MAX_LIGHTS = 4;
// Interpolated values from the vertex shaders
in vec2 UV;
in vec3 Position_worldspace;
in vec3 Normal_cameraspace;
in vec3 EyeDirection_cameraspace;
in vec3 LightDirection_cameraspace[MAX_LIGHTS];

// Ouput data
out vec4 color;

// Values that stay constant for the whole mesh.
uniform sampler2D myTextureSampler;
uniform vec3 LightColors[MAX_LIGHTS];
uniform float LightPowers[MAX_LIGHTS];
uniform vec3 LightPositions_worldspace[4];

void main(){

	// Light emission properties
	// You probably want to put them as uniforms
	//vec3 LightColor = vec3(1,1,1);
	//float LightPower = 50.0f;
	
	// Material properties

	vec3 MaterialDiffuseColor = texture( myTextureSampler, UV ).rgb;
	vec3 MaterialAmbientColor = vec3(0.1,0.1,0.1) * MaterialDiffuseColor;
	vec3 MaterialSpecularColor = vec3(0.3,0.3,0.3);

	vec3 TotalDiffuseColor = vec3(0.0);
	vec3 TotalSpecularColor = vec3(0.0);

	// Normal of the computed fragment, in camera space
	vec3 n = normalize( Normal_cameraspace );
	// Eye vector (towards the camera)
	vec3 E = normalize(EyeDirection_cameraspace);

	for(int i=0 ; i<MAX_LIGHTS ; i++) {
		// Distance to the light
		float distance = length( LightPositions_worldspace[i] - Position_worldspace );

		// Direction of the light (from the fragment to the light)
		vec3 l = normalize( LightDirection_cameraspace[i] );
		// Cosine of the angle between the normal and the light direction,
		// clamped above 0
		//  - light is at the vertical of the triangle -> 1
		//  - light is perpendicular to the triangle -> 0
		//  - light is behind the triangle -> 0
		float cosTheta = clamp( dot( n,l ), 0,1 );

		// Direction in which the triangle reflects the light
		vec3 R = reflect(-l,n);
		// Cosine of the angle between the Eye vector and the Reflect vector,
		// clamped to 0
		//  - Looking into the reflection -> 1
		//  - Looking elsewhere -> < 1
		float cosAlpha = clamp( dot( E,R ), 0,1 );

		// Diffuse : "color" of the object
		vec3 calc1 = (MaterialDiffuseColor * LightColors[i] * LightPowers[i] * cosTheta / (distance*distance));

		TotalDiffuseColor = TotalDiffuseColor + calc1;

		// Specular : reflective highlight, like a mirror
		vec3 calc2 = (MaterialSpecularColor * LightColors[i] * LightPowers[i] * pow(cosAlpha,5) / (distance*distance));

		TotalSpecularColor = TotalSpecularColor + calc2;


	}
	//TotalDiffuseColor = max(TotalDiffuseColor, 0.2);
	// Ambient : simulates indirect lighting
	color = vec4(MaterialAmbientColor + TotalDiffuseColor + TotalSpecularColor , 1.0);
}
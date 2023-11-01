//Vertex Properties
layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec2 a_uv;
layout(location = 3) in vec4  a_add_uv;
layout(location = 4) in vec4  a_bone_indices;
layout(location = 5) in vec4  a_bone_weights;
layout(location = 6) in float a_weight_formula;

out vec2 o_uv;

uniform mat4 Bones[200];
uniform mat4 u_mvp;

void main()
{
	vec4 newVertex;
	vec4 newNormal;
	
	int index1, index2, index3, index4, weightFormula;

	index1 = int(a_bone_indices.x);
	index2 = int(a_bone_indices.y);
	index3 = int(a_bone_indices.z);
	index4 = int(a_bone_indices.w);

	weightFormula = int(a_weight_formula);
	
    vec4 position = vec4(a_position, 1.0);

	if(weightFormula==0) //BDEF1
	{
		newVertex  = (Bones[index1] * position);
		newNormal  = (Bones[index1] * vec4(a_normal, 0.0)) * a_bone_weights.x;
	}
	else if(weightFormula==1) //BDEF2
	{
		newVertex  = (Bones[index1] * position)          * a_bone_weights.x;
		newVertex += (Bones[index2] * position)          * (1.0 - a_bone_weights.x);
		
		newNormal  = (Bones[index1] * vec4(a_normal, 0.0)) * a_bone_weights.x;
		newNormal += (Bones[index2] * vec4(a_normal, 0.0)) * (1.0 - a_bone_weights.x);
	}
	else if(weightFormula==2) //BDEF4
	{
		newVertex  = (Bones[index1] * position)          * a_bone_weights.x;
		newNormal  = (Bones[index1] * vec4(a_normal, 0.0)) * a_bone_weights.x;
		
		newVertex += (Bones[index2] * position)          * a_bone_weights.y;
		newNormal += (Bones[index2] * vec4(a_normal, 0.0)) * a_bone_weights.y;
		
		newVertex += (Bones[index3] * position)          * a_bone_weights.z;
		newNormal += (Bones[index3] * vec4(a_normal, 0.0)) * a_bone_weights.z;
		
		newVertex += (Bones[index4] * position)          * a_bone_weights.w;
		newNormal += (Bones[index4] * vec4(a_normal, 0.0)) * a_bone_weights.w;
	}
	else // TODO: SDEF (weightFormula==3), QDEF (weightFormula==4)
	{
		newVertex = position;
		newNormal = vec4(a_normal, 0.0);
	}
	
	newVertex.w = 1.0;

	// gl_Position = MVP * newVertex;
	// gl_Position = pers * view * newVertex;
	// UV 		    = vUV;
	// normal      = normalize(newNormal.xyz);
    gl_Position = u_mvp * vec4(a_position, 1.0);
	o_uv = a_uv;
}

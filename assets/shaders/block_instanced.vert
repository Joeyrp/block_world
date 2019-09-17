 #version 330   

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 tex_coord;
layout (location = 2) in vec3 normal;
layout (location = 3) in vec3 offset;
layout (location = 4) in uint texture;

out VS_OUTPUT 
{
    vec2 TexCoord;
    vec3 v_normal;
    flat uint texture;
} OUT;

uniform mat4 perspective;  
uniform mat4 view;
uniform mat4 model;

void main() 
{
    mat4 translation = mat4(model);
    translation[3][0] = offset.x;
    translation[3][1] = offset.y;
    translation[3][2] = offset.z;
    OUT.TexCoord = tex_coord;
    OUT.v_normal = transpose(inverse(mat3(translation))) * normal; 
    OUT.texture = texture;
    gl_Position = perspective * view * translation * vec4(position, 1.0);
}
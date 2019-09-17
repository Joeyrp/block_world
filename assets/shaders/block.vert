 #version 330   

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 tex_coord;
layout (location = 2) in vec3 normal;

out VS_OUTPUT 
{
    vec2 TexCoord;
    vec3 v_normal;
} OUT;

uniform mat4 perspective;  
uniform mat4 view;
uniform mat4 model;

void main() 
{
    OUT.TexCoord = tex_coord;
    OUT.v_normal = transpose(inverse(mat3(model))) * normal; 
    gl_Position = perspective * view * model * vec4(position, 1.0);
}
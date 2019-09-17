#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 color;


out VS_OUTPUT 
{
    vec3 Color;
} OUT;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    OUT.Color = color;
    gl_Position = projection * view * model * vec4(pos, 1.0);
}
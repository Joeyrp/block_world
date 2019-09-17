#version 330 core

in VS_OUTPUT
{
    vec3 Color;
} IN;

out vec4 FragColor;

void main()
{
    FragColor = vec4(IN.Color, 1.0);
}
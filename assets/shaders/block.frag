 #version 330

in VS_OUTPUT
{
    vec2 TexCoord;
    vec3 v_normal;
    flat uint texture;
} IN;

out vec4 color;
uniform vec3 u_light;
uniform sampler2D tex1;
uniform sampler2D tex2;
uniform sampler2D tex3;

void main() 
{
    float brightness = dot(normalize(IN.v_normal), normalize(u_light));
    //vec4 regular_color = vec4(1.0, 0.0, 0.0, 1.0);
    
    vec4 regular_color = texture(tex1, IN.TexCoord);

    if (IN.texture == uint(2))
        regular_color = texture(tex2, IN.TexCoord);
    
    if (IN.texture == uint(3))
        regular_color = texture(tex3, IN.TexCoord);

    vec4 dark_color = vec4(regular_color.x * 0.5f, regular_color.y * 0.5f, regular_color.z * 0.5f, 1.0);
    color = vec4(mix(dark_color, regular_color, brightness));
}
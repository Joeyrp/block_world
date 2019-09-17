#version 330 core
struct Material
{
    sampler2D diffuse;
    sampler2D specular;
    sampler2D normal;       // TODO: implement normal map
	//sampler2D emission;
    float shininess;
};

struct DirLight {
    vec3 direction;
  
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};  
uniform DirLight dirLight;

struct PointLight {    
    vec3 position;
    
    float constant;
    float linear;
    float quadratic;  

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};  
#define NR_POINT_LIGHTS 4  
uniform PointLight pointLights[NR_POINT_LIGHTS];

struct SpotLight {
    vec3  position;
    vec3  direction;
    float cutOff;
    float outerCutOff;

    float constant;
    float linear;
    float quadratic;
    
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
}; 
uniform SpotLight spotLight;

uniform Material material;
uniform vec3 viewPos;

in vec3 FragPos; 
in vec2 TextureCoords;
in vec3 Normal; 

out vec4 FragColor;

// FUNCTIONS
vec3 CalcDirLight(DirLight light, vec3 normal, vec3 viewDir);  
vec3 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir);
vec3 CalcSpotLight(SpotLight light, vec3 norm, vec3 fragPos, vec3 viewDir); 

void main()
{
    // properties
    vec3 norm = normalize(Normal);
    vec3 viewDir = normalize(viewPos - FragPos);

    // phase 1: Directional lighting
    vec3 result = CalcDirLight(dirLight, norm, viewDir);

    // phase 2: Point lights
    //for(int i = 0; i < NR_POINT_LIGHTS; i++)
	//	result += CalcPointLight(pointLights[i], norm, FragPos, viewDir);    

    // phase 3: Spot light
   // result += CalcSpotLight(spotLight, norm, FragPos, viewDir);    
    
    FragColor = vec4(result, 1.0);

   //FragColor = vec4(vec3(texture(material.diffuse, TextureCoords)), 1.0);
}

vec3 CalcDirLight(DirLight light, vec3 normal, vec3 viewDir)
{
    vec3 lightDir = normalize(-light.direction);
    // diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);
    // specular shading
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    // combine results
    vec3 ambient  = light.ambient  * vec3(texture(material.diffuse, TextureCoords));
    vec3 diffuse  = light.diffuse  * diff * vec3(texture(material.diffuse, TextureCoords));
    vec3 specular = light.specular * spec * vec3(texture(material.specular, TextureCoords));
    return (ambient + diffuse + specular);
} 

vec3 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir)
{
    vec3 lightDir = normalize(light.position - fragPos);
    // diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);
    // specular shading
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    // attenuation
    float distance    = length(light.position - fragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + 
  			     light.quadratic * (distance * distance));    
    // combine results
    vec3 ambient  = light.ambient  * vec3(texture(material.diffuse, TextureCoords));
    vec3 diffuse  = light.diffuse  * diff * vec3(texture(material.diffuse, TextureCoords));
    vec3 specular = light.specular * spec * vec3(texture(material.specular, TextureCoords));
    ambient  *= attenuation;
    diffuse  *= attenuation;
    specular *= attenuation;
    return (ambient + diffuse + specular);
} 

vec3 CalcSpotLight(SpotLight light, vec3 norm, vec3 fragPos, vec3 viewDir)
{
	// ambient
    vec3 ambient = light.ambient * texture(material.diffuse, TextureCoords).rgb;
    
    // diffuse 
    vec3 lightDir = normalize(light.position - fragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = light.diffuse * diff * texture(material.diffuse, TextureCoords).rgb;  
    
    // specular
    vec3 reflectDir = reflect(-lightDir, norm);  
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = light.specular * spec * texture(material.specular, TextureCoords).rgb;  

 // spotlight (soft edges)
    float theta = dot(lightDir, normalize(-light.direction)); 
    float epsilon = (light.cutOff - light.outerCutOff);
    float intensity = clamp((theta - light.outerCutOff) / epsilon, 0.0, 1.0);
    diffuse  *= intensity;
    specular *= intensity;
    
    // attenuation
    float distance    = length(light.position - FragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));    
    ambient  *= attenuation; 
    diffuse   *= attenuation;
    specular *= attenuation; 

	return (ambient + diffuse + specular);

	
	// Spot Light Hard Edge Version
//	float theta = dot(lightDir, normalize(-light.direction));
//    
//	if(theta > light.cutOff) 
//	{       
//	  float distance    = length(light.position - FragPos);
//    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));    
//    ambient  *= attenuation; 
//    diffuse   *= attenuation;
//    specular *= attenuation; 
//
//	return (ambient + diffuse + specular);
//	}
//	else  // else, use ambient light so scene isn't completely dark outside the spotlight.
//	  return vec3(light.ambient * vec3(texture(material.diffuse, TextureCoords)));
}
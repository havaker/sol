#version 420

in vec2 texture_coords;
in vec3 frag_normal;
in vec3 frag_pos;

out vec4 color;

uniform sampler2D tex;

uniform vec3 light_pos;
uniform vec3 light_color;
uniform float ambient_strength;

void main() {
    vec3 norm = normalize(frag_normal);
    vec3 light_direction = normalize(light_pos - frag_pos);
    float diff = max(dot(norm, light_direction), 0.0);
    vec3 diffuse = diff * light_color;

    vec3 ambient = ambient_strength * light_color;

    vec3 texture_color = texture(tex, texture_coords).xyz;

    color = vec4((ambient + diffuse) * texture_color, 0);
}

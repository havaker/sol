#version 140

in vec3 position;
in vec3 normal;
in vec3 texture;

out vec2 texture_coords;
out vec3 frag_normal;
out vec3 frag_pos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 perspective;

void main() {
    gl_Position = perspective * (view * (model * vec4(position, 1.0f)));

    texture_coords = texture.xy;

    frag_normal = vec3(mat3(transpose(inverse(model))) * normal);
    //frag_normal = normal;
    frag_pos = vec3(model * vec4(position, 1.0));

}

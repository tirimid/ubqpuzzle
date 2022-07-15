#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;
layout (location = 2) in vec3 normal;

uniform mat4 model_mat;
uniform mat4 view_mat;
uniform mat4 proj_mat;

out vs_output {
    vec3 color;
} vs_out;

void main() {
    gl_Position = proj_mat * view_mat * model_mat * vec4(position, 1.0);
    vs_out.color = color;
}

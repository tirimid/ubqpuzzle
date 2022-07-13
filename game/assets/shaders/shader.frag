#version 330 core

in vs_output {
    vec3 color;
} fg_in;

out vec4 color;

void main() {
    color = vec4(fg_in.color, 1.0f);
}

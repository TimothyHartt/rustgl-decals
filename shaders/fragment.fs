#version 400 core

out vec4 final_color;

uniform vec3 color;

void main() {
    
    final_color = vec4(color.x, color.y, color.z, 1.0);
}
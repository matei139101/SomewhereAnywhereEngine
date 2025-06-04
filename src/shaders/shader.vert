#version 450

layout(set = 0, binding = 0) uniform Uniforms {
    mat4 mvp;
} ubo;

// Input from vertex buffer
layout(location = 0) in vec3 position;
layout(location = 1) in vec3 color;

// Output to fragment shader
layout(location = 0) out vec3 fragColor;

void main() {
    gl_Position = ubo.mvp * vec4(position, 1.0);
    fragColor = color;
}
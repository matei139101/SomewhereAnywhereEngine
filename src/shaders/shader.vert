#version 450

// Replace the uniform block with push constants
layout(push_constant) uniform PushConstants {
    mat4 mvp;
} pc;

// Input from vertex buffer
layout(location = 0) in vec3 position;
layout(location = 1) in vec3 color;

// Output to fragment shader
layout(location = 0) out vec3 fragColor;

void main() {
    // Use pc.mvp instead of ubo.mvp
    gl_Position = pc.mvp * vec4(position, 1.0);
    fragColor = color;
}
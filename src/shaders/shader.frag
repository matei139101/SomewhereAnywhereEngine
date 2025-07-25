#version 450

// Input
layout(location = 0) in vec3 fragColor;
layout(location = 1) in vec2 texCoord;

// Output
layout(binding = 1) uniform sampler2D tex_sampler;

void main() {
    vec4 tex_color = texture(tex_sampler, texCoord);
    outColor = tex_color * vec4(frag_color, 1.0);
    outColor = vec4(fragColor, 1.0);
}
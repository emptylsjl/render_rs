#version 460

//#extension GL_ARB_separate_shader_objects : enable
//#extension GL_ARB_shading_language_420pack : enable
//
//layout (location = 0) in vec4 pos;
//layout (location = 1) in vec4 color;
//
//
//layout (location = 0) out vec4 o_color;
//void main() {
//    o_color = color;
//    gl_Position = pos;
//}

layout (location = 0) in vec4 position;

layout (binding = 0) uniform cameraTransformation {
    mat4 cam;
} camera;
//layout (binding = 0) uniform mat4 camera;

layout (location = 1) in vec4 color;

layout(location = 2) in vec2 TexCoord;

layout(location = 0) out vec4 fragColor;

layout(location = 1) out vec2 fragTexCoord;

void main() {
    gl_Position = camera.cam * position;
    fragColor = color;
    fragTexCoord = TexCoord;
}
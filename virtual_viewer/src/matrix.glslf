#version 150 core

uniform sampler2D t_Texture;
in vec2 v_Uv;
in vec4 v_Color;
out vec4 Target0;

layout (std140) uniform Globals {
    mat4 u_MVP;
};

layout (std140) uniform MatrixPixelShader {
    float u_Width;
    float u_Height;
};

const float pi = 3.1415926535897932384626433832795;

void main() {
    float inner_x = fract(v_Uv.x * u_Width);
    float inner_y = fract(v_Uv.y * u_Height);

    float sin_blob = sin(inner_x * pi) * sin(inner_y * pi);
    float blob_mask = smoothstep(0.75, 0.8, sin_blob);

    vec3 tex = texture(t_Texture, v_Uv).rgb;
    vec3 pix = tex * blob_mask;

    Target0 = vec4(pix, 1.0);
}

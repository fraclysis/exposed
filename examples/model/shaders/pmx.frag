uniform sampler2D u_texture;

in vec2 o_uv;
out vec4 FragColor;

void main() {
    FragColor = texture(u_texture, o_uv);
}
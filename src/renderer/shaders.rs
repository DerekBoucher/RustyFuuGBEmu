pub const VERTEX: &str = r#"
#version 330 core

layout (location = 0) in vec2 position;
layout (location = 1) in vec3 color;
out vec4 outputColor;

void main() 
{
    outputColor = vec4(color, 1.0);
    gl_Position = vec4(position.x, position.y, 0.0, 1.0);
}
"#;

pub const FRAGMENT: &str = r#"
#version 330 core

in vec4 outputColor;

out vec4 FragColor;

void main()
{
    FragColor = outputColor;
}
"#;

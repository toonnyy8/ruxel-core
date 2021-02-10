let msg = cmd;
clear_down(down);

if render_pipeline == "default" {
    up = default_render(canvas,color,cursor,up);
} else if render_pipeline == "custom" {
    up = custom_render(canvas,color,cursor,up,"  ","bg");
}

print("\n");
print(msg);
print("\x1B[1F");
print(":");

let up = up + 1;
let down = msg_line_num(msg);
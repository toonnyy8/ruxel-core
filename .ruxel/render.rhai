let msg = cmd;
clear_down(down);
let up = default_render(canvas,color,cursor,up);
// print("\x1B[2K");
print("\n");
print(msg);
print("\x1B[1F");
print(":");

let up = up + 1;
let down = msg_line_num(msg);
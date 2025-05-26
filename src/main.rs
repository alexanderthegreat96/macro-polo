mod lua_bindings;

fn main() {
    let img_path: &str = "/home/alexander/Downloads/database-workflow.drawio.png";
    let (x, y, width, height) = (0, 0, 100, 100);

    let img_compare = lua_bindings::image_tools::image_match(img_path, x, y, width, height);
    println!("Template match found: {:?}", img_compare);

    // let lua_text = r#"
    //     move_mouse(500, 300)
    //     left_click()
    //     type_text("mcro-polo intialized!")
    //     key_press("A")
    //     sleep(1000)
    //     key_press("B")
    //     sleep(1000)
    //     key_press("C")
    //     "#;

    // let execute: mlua::Result<()> = lua_bindings::lua_executor::compile_and_exec(lua_text);

    // if execute.is_err() {
    //     println!("something happened: {:?}", execute.err())
    // }
}

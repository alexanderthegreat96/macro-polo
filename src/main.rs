mod lua_bindings;

fn main() {
    let lua_text = r#"
        move_mouse(500, 300)
        left_click()
        type_text("mcro-polo intialized!")
        key_press("A")
        sleep(1000)
        key_press("B")
        sleep(1000)
        key_press("C")
        "#;

    let execute: mlua::Result<()> = lua_bindings::lua_executor::compile_and_exec(lua_text);

    if execute.is_err() {
        println!("something happened: {:?}", execute.err())
    }
}

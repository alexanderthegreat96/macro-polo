use enigo::*;
use mlua::Lua;

fn main() -> mlua::Result<()> {
    let lua = Lua::new();
    let globals = lua.globals();

    let move_mouse = lua.create_function(|_, (x, y): (i32, i32)| {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();

        enigo.move_mouse(x, y, Coordinate::Abs).unwrap();
        Ok(())
    })?;

    let left_click = lua.create_function(|_, ()| {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();

        enigo.button(Button::Left, Direction::Click).unwrap();

        Ok(())
    })?;

    let right_click = lua.create_function(|_, ()| {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();

        enigo.button(Button::Right, Direction::Click).unwrap();
        Ok(())
    })?;

    globals.set("move_mouse", move_mouse)?;
    globals.set("left_click", left_click)?;
    globals.set("right_click", right_click)?;

    lua.load(
        r#"
        move_mouse(500, 300)
        left_click()
    "#,
    )
    .exec()?;

    Ok(())
}

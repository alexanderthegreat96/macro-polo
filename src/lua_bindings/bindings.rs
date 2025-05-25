use std::cell::RefCell;
use std::rc::Rc;

use enigo::{Button, Coordinate, Direction, Enigo, Keyboard, Mouse};
use mlua::{Error, FromLuaMulti, Lua, MultiValue, Result, Value};

// holly fuck, this is going to give you brain cancer
// but long story short, we're binding enigo's ability to
// execute mouse & keyboard actions
// to mlua so that we can use lua to hook into these

fn register_fn<F>(lua: &Lua, name: &str, func: F) -> Result<()>
where
    F: 'static + Fn(&Lua, MultiValue) -> Result<Value>,
{
    let globals = lua.globals();
    let wrapper = lua.create_function(move |lua, args| func(lua, args))?;
    globals.set(name, wrapper)?;
    Ok(())
}

pub fn register_all(lua: &Lua, enigo: Rc<RefCell<Enigo>>) -> Result<()> {
    let mut failures: Vec<(&str, Error)> = Vec::new();

    // move_mouse(x, y)
    if let Err(e) = register_fn(lua, "move_mouse", {
        let enigo = Rc::clone(&enigo);
        move |lua, args: MultiValue| {
            let (x, y): (i32, i32) = FromLuaMulti::from_lua_multi(args, lua)?;
            enigo
                .borrow_mut()
                .move_mouse(x, y, Coordinate::Abs)
                .map_err(Error::external)?;
            Ok(Value::Nil)
        }
    }) {
        failures.push(("move_mouse", e));
    }

    // left_click()
    if let Err(e) = register_fn(lua, "left_click", {
        let enigo = Rc::clone(&enigo);
        move |_, _| {
            enigo
                .borrow_mut()
                .button(Button::Left, Direction::Click)
                .map_err(Error::external)?;
            Ok(Value::Nil)
        }
    }) {
        failures.push(("left_click", e));
    }

    // type_text("Hello")
    if let Err(e) = register_fn(lua, "type_text", {
        let enigo = Rc::clone(&enigo);
        move |lua, args: MultiValue| {
            let (text,): (String,) = FromLuaMulti::from_lua_multi(args, lua)?;
            enigo.borrow_mut().text(&text).map_err(Error::external)?;
            Ok(Value::Nil)
        }
    }) {
        failures.push(("type_text", e));
    }

    if !failures.is_empty() {
        eprintln!("⚠ Failed to bind the following Lua functions:");
        for (name, err) in failures {
            eprintln!("- {}: {}", name, err);
        }
    }

    Ok(())
}

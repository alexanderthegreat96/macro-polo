use std::rc::Rc;
use std::{cell::RefCell, time::Duration};

use enigo::{Axis, Button, Coordinate, Direction, Enigo, Key, Keyboard, Mouse};
use mlua::{Error, FromLuaMulti, Lua, MultiValue, Result, Value};

use super::key_mapping::key_from_str;

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

    // right_click()
    if let Err(e) = register_fn(lua, "right_click", {
        let enigo = Rc::clone(&enigo);
        move |_, _| {
            enigo
                .borrow_mut()
                .button(Button::Right, Direction::Click)
                .map_err(Error::external)?;
            Ok(Value::Nil)
        }
    }) {
        failures.push(("right_click", e));
    }

    // left_press()
    if let Err(e) = register_fn(lua, "left_press", {
        let enigo = Rc::clone(&enigo);
        move |_, _| {
            enigo
                .borrow_mut()
                .button(Button::Left, Direction::Press)
                .map_err(Error::external)?;
            Ok(Value::Nil)
        }
    }) {
        failures.push(("left_press", e));
    }

    // right_press()
    if let Err(e) = register_fn(lua, "right_press", {
        let enigo = Rc::clone(&enigo);
        move |_, _| {
            enigo
                .borrow_mut()
                .button(Button::Right, Direction::Press)
                .map_err(Error::external)?;
            Ok(Value::Nil)
        }
    }) {
        failures.push(("right_press", e));
    }

    // left_release()
    if let Err(e) = register_fn(lua, "left_release", {
        let enigo = Rc::clone(&enigo);
        move |_, _| {
            enigo
                .borrow_mut()
                .button(Button::Left, Direction::Release)
                .map_err(Error::external)?;
            Ok(Value::Nil)
        }
    }) {
        failures.push(("left_release", e));
    }

    // right_release()
    if let Err(e) = register_fn(lua, "right_release", {
        let enigo = Rc::clone(&enigo);
        move |_, _| {
            enigo
                .borrow_mut()
                .button(Button::Right, Direction::Release)
                .map_err(Error::external)?;
            Ok(Value::Nil)
        }
    }) {
        failures.push(("right_release", e));
    }

    // scroll_mouse(x, y)
    if let Err(e) = register_fn(lua, "scroll_mouse", {
        let enigo = Rc::clone(&enigo);
        move |lua, args: MultiValue| {
            let (length, axis_str): (i32, String) = FromLuaMulti::from_lua_multi(args, lua)?;

            let axis = match axis_str.as_str() {
                "x" | "horizontal" => Axis::Horizontal,
                "y" | "vertical" => Axis::Vertical,
                _ => return Err(Error::external("Invalid axis: use 'x' or 'y'")),
            };

            enigo
                .borrow_mut()
                .scroll(length, axis)
                .map_err(Error::external)?;

            Ok(Value::Nil)
        }
    }) {
        failures.push(("scroll_mouse", e));
    }

    // drag_mouse(x1, y1, x2, y2)
    if let Err(e) = register_fn(lua, "drag_mouse", {
        let enigo = Rc::clone(&enigo);
        move |lua, args: MultiValue| {
            let (x1, y1, x2, y2): (i32, i32, i32, i32) = FromLuaMulti::from_lua_multi(args, lua)?;

            let mut enigo = enigo.borrow_mut();
            enigo
                .move_mouse(x1, y1, Coordinate::Abs)
                .map_err(Error::external)?;
            enigo
                .button(Button::Left, Direction::Press)
                .map_err(Error::external)?;
            enigo
                .move_mouse(x2, y2, Coordinate::Abs)
                .map_err(Error::external)?;
            enigo
                .button(Button::Left, Direction::Release)
                .map_err(Error::external)?;
            Ok(Value::Nil)
        }
    }) {
        failures.push(("drag_mouse", e));
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

    // key_down("key")
    if let Err(e) = register_fn(lua, "key_down", {
        let enigo = Rc::clone(&enigo);
        move |lua, args: MultiValue| {
            let (input_key,): (String,) = FromLuaMulti::from_lua_multi(args, lua)?;

            match key_from_str(&input_key) {
                Some(key) => {
                    enigo
                        .borrow_mut()
                        .key(key, Direction::Press)
                        .map_err(Error::external)?;
                    Ok(Value::Nil)
                }
                None => Err(Error::external(format!("Unknown key: {}", input_key))),
            }
        }
    }) {
        failures.push(("key_down", e));
    }

    // key_up("key")
    if let Err(e) = register_fn(lua, "key_up", {
        let enigo = Rc::clone(&enigo);
        move |lua, args: MultiValue| {
            let (input_key,): (String,) = FromLuaMulti::from_lua_multi(args, lua)?;

            match key_from_str(&input_key) {
                Some(key) => {
                    enigo
                        .borrow_mut()
                        .key(key, Direction::Release)
                        .map_err(Error::external)?;
                    Ok(Value::Nil)
                }
                None => Err(Error::external(format!("Unknown key: {}", input_key))),
            }
        }
    }) {
        failures.push(("key_up", e));
    }

    // key_press(key)
    if let Err(e) = register_fn(lua, "key_press", {
        let enigo = Rc::clone(&enigo);
        move |lua, args: MultiValue| {
            let (input_key,): (String,) = FromLuaMulti::from_lua_multi(args, lua)?;

            match key_from_str(&input_key) {
                Some(key) => {
                    enigo
                        .borrow_mut()
                        .key(key, Direction::Click)
                        .map_err(Error::external)?;
                    Ok(Value::Nil)
                }
                None => Err(Error::external(format!("Unknown key: {}", input_key))),
            }
        }
    }) {
        failures.push(("key_press", e));
    }

    // sleep(milliseconds)
    if let Err(e) = register_fn(lua, "sleep", {
        move |lua, args: MultiValue| {
            let (millis,): (u64,) = FromLuaMulti::from_lua_multi(args, lua)?;
            std::thread::sleep(Duration::from_millis(millis));
            Ok(Value::Nil)
        }
    }) {
        failures.push(("sleep", e));
    }

    if !failures.is_empty() {
        eprintln!("⚠ Failed to bind the following Lua functions:");
        for (name, err) in failures {
            eprintln!("- {}: {}", name, err);
        }
    }

    Ok(())
}

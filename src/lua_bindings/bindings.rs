use std::rc::Rc;
use std::{cell::RefCell, time::Duration};

use enigo::{Axis, Button, Coordinate, Direction, Enigo, Keyboard, Mouse};
use mlua::{Error, FromLuaMulti, Function, Lua, MultiValue, Result, Value};
use thiserror::Error;

use super::key_mapping::key_from_str;

// custom error to wrap lua type stuff
#[derive(Debug, Error)]
pub enum ScriptError {
    #[error("Lua error: {0}")]
    Lua(#[from] mlua::Error),

    #[error("Enigo Input Error: {0}")]
    EnigoInput(#[from] enigo::InputError),

    #[error("Enigo Con Error: {0}")]
    EnigoConnection(#[from] enigo::NewConError),
}

// Helper function to register a Lua function
fn register_fn<F>(lua: &Lua, name: &str, func: F) -> Result<()>
where
    F: 'static + Fn(&Lua, MultiValue) -> Result<Value>,
{
    let globals = lua.globals();
    let wrapper = lua.create_function(move |lua, args| func(lua, args))?;
    globals.set(name, wrapper)?;
    Ok(())
}

// Macro for mouse button actions
macro_rules! bind_mouse_button {
    ($lua:expr, $enigo:expr, $name:expr, $button:expr, $direction:expr) => {
        register_fn($lua, $name, {
            let enigo = Rc::clone(&$enigo);
            move |_, _| {
                enigo
                    .borrow_mut()
                    .button($button, $direction)
                    .map_err(Error::external)?;
                Ok(Value::Nil)
            }
        })
    };
}

// Macro for key actions
macro_rules! bind_key_action {
    ($lua:expr, $enigo:expr, $name:expr, $direction:expr) => {
        register_fn($lua, $name, {
            let enigo = Rc::clone(&$enigo);
            move |lua, args: MultiValue| {
                let (input_key,): (String,) = FromLuaMulti::from_lua_multi(args, lua)?;
                match key_from_str(&input_key) {
                    Some(key) => {
                        enigo
                            .borrow_mut()
                            .key(key, $direction)
                            .map_err(Error::external)?;
                        Ok(Value::Nil)
                    }
                    None => Err(Error::external(format!("Unknown key: {}", input_key))),
                }
            }
        })
    };
}

pub fn register_all(lua: &Lua, enigo: Rc<RefCell<Enigo>>) -> Result<()> {
    let mut failures: Vec<(&str, Error)> = Vec::new();

    // **Mouse Button Actions**
    if let Err(e) = bind_mouse_button!(lua, enigo, "left_click", Button::Left, Direction::Click) {
        failures.push(("left_click", e));
    }
    if let Err(e) = bind_mouse_button!(lua, enigo, "right_click", Button::Right, Direction::Click) {
        failures.push(("right_click", e));
    }
    if let Err(e) = bind_mouse_button!(lua, enigo, "left_press", Button::Left, Direction::Press) {
        failures.push(("left_press", e));
    }
    if let Err(e) = bind_mouse_button!(lua, enigo, "right_press", Button::Right, Direction::Press) {
        failures.push(("right_press", e));
    }
    if let Err(e) = bind_mouse_button!(lua, enigo, "left_release", Button::Left, Direction::Release)
    {
        failures.push(("left_release", e));
    }
    if let Err(e) = bind_mouse_button!(
        lua,
        enigo,
        "right_release",
        Button::Right,
        Direction::Release
    ) {
        failures.push(("right_release", e));
    }

    // **Key Actions**
    if let Err(e) = bind_key_action!(lua, enigo, "key_down", Direction::Press) {
        failures.push(("key_down", e));
    }
    if let Err(e) = bind_key_action!(lua, enigo, "key_up", Direction::Release) {
        failures.push(("key_up", e));
    }
    if let Err(e) = bind_key_action!(lua, enigo, "key_press", Direction::Click) {
        failures.push(("key_press", e));
    }

    // **Other Functions**

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

    // scroll_mouse(length, axis)
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

    // type_text(text)
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

    // wait_for(condition)
    if let Err(e) = register_fn(lua, "wait_for", {
        move |lua, args: MultiValue| {
            let (func,): (Function,) = FromLuaMulti::from_lua_multi(args, lua)?;
            loop {
                let should_continue: bool = func.call(())?;
                if should_continue {
                    break;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            Ok(Value::Nil)
        }
    }) {
        failures.push(("wait_for", e));
    }

    // Report any failures
    if !failures.is_empty() {
        eprintln!("⚠ Failed to bind the following Lua functions:");
        for (name, err) in failures {
            eprintln!("- {}: {}", name, err);
        }
    }

    Ok(())
}

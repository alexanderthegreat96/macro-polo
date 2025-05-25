use enigo::{Enigo, Settings};
use mlua::{Lua, StdLib};
use std::cell::RefCell;
use std::rc::Rc;

use crate::lua_bindings::bindings::register_all;

// will take the string from the user
// compile, hook the functions
// and execute the macro basically

pub fn compile_and_exec(lua_text: &str) -> mlua::Result<()> {
    let lua = Lua::new_with(StdLib::ALL_SAFE, Default::default())?;
    let enigo = Rc::new(RefCell::new(Enigo::new(&Settings::default()).unwrap()));

    register_all(&lua, Rc::clone(&enigo))?;

    lua.load(lua_text).exec()?;
    Ok(())
}

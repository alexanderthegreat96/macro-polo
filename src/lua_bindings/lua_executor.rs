use enigo::{Enigo, Settings};
use mlua::{Lua, StdLib};
use std::cell::RefCell;
use std::rc::Rc;

use crate::lua_bindings::bindings::register_all;
use crate::lua_bindings::bindings::ScriptError;

// will take the string from the user
// compile, hook the functions
// and execute the macro basically

pub fn compile_and_exec(lua_text: &str) -> Result<(), ScriptError> {
    let lua = Lua::new_with(StdLib::ALL_SAFE, Default::default())?;

    let enigo = Enigo::new(&Settings::default()).map_err(ScriptError::EnigoConnection)?;
    let enigo = Rc::new(RefCell::new(enigo));

    register_all(&lua, Rc::clone(&enigo))?;
    lua.load(lua_text).exec().map_err(ScriptError::Lua)
}

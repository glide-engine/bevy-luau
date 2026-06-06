use lasso::{Rodeo, Spur};
use mluau::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct EngineStringPool {
    pub rodeo: Rodeo,
    pub bridge: HashMap<Spur, LuaString>,
}

impl EngineStringPool {
    /// # Panics
    #[must_use]
    pub fn get_lua_str(&self, spur: Spur) -> &LuaString {
        self.bridge.get(&spur).expect("unregistered spur")
    }

    /// # Panics
    pub fn intern(&mut self, lua: &Lua, s: &str) -> Spur {
        let spur = self.rodeo.get_or_intern(s);
        self.bridge
            .entry(spur)
            .or_insert_with(|| lua.create_string(s).unwrap());
        spur
    }
}

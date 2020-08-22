use rlua;

fn main() -> rlua::Result<()> {
    // You can create a new Lua state with `Lua::new()`.  This loads the default Lua std library
    // *without* the debug library.  You can get more control over this with the other
    // `Lua::xxx_new_xxx` functions.
    let lua = rlua::Lua::new();

    // In order to interact with Lua values at all, you must do so inside a callback given to the
    // `Lua::context` method.  This provides some extra safety and allows the rlua API to avoid some
    // extra runtime checks.
    lua.context(|lua_ctx:rlua::Context| {
        // You can get and set global variables.  Notice that the globals table here is a permanent
        // reference to _G, and it is mutated behind the scenes as Lua code is loaded.  This API is
        // based heavily around sharing and internal mutation (just like Lua itself).

        let globals = lua_ctx.globals();

        globals.set("string_var", "hello")?;
        globals.set("int_var", 42)?;

        Ok(())
    })?;

    lua.context(|lua_ctx:rlua::Context| {
        // The Lua state lives inside the top-level `Lua` value, and all state changes persist
        // between `Lua::context` calls.  This is another table reference in another context call,
        // but it refers to the same table _G.

        let globals = lua_ctx.globals();

        assert_eq!(globals.get::<_, String>("string_var")?, "hello");
        assert_eq!(globals.get::<_, i64>("int_var")?, 42);

        Ok(())
    })
}

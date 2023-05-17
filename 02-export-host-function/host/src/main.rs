fn main() -> anyhow::Result<()> {
    // Modules can be compiled through either the text or binary format
    let engine = wasmtime::Engine::default();
    let module = wasmtime::Module::from_file(
        &engine,
        "../guest/target/wasm32-unknown-unknown/debug/guest.wasm",
    )?;

    // All wasm objects operate within the context of a "store". Each
    // `Store` has a type parameter to store host-specific data, which in
    // this case we're using `4` for.
    let mut store = wasmtime::Store::new(&engine, ());

    let from_host = wasmtime::Func::wrap(&mut store, |mut caller: wasmtime::Caller<'_, ()>| {
        println!("called from wasm");
        let mut o = wasmtime::Val::I32(0);
        let mut v = vec![o];
        caller.get_export("foo").expect("foo not found").into_func().expect("foo not a func").call(&mut caller, &[], &mut v).expect("call failed");
        dbg!(v);
    });

    let from_host2 = wasmtime::Func::wrap(&mut store, |mut _caller: wasmtime::Caller<'_, ()>| {
        println!("called from wasm2");
        10
    });

    // Instantiation of a module requires specifying its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get_typed_func`.
    let instance = wasmtime::Instance::new(&mut store, &module, &[from_host.into(), from_host2.into()])?;
    let sum = instance.get_typed_func::<(i32,), i32>(&mut store, "sum")?;

    // And finally we can call the wasm!
    println!("wasm said: {}", sum.call(&mut store, (1,))?);

    Ok(())
}

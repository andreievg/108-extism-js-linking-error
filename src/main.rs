use extism::{Manifest, PluginBuilder, Wasm};

fn main() {
    let manifest = Manifest::new([Wasm::file("jsplugin/jsplugin.wasm")]);
    let _ = PluginBuilder::new(manifest)
        .with_wasi(true)
        .with_cache_disabled()
        .build()
        .unwrap();
}

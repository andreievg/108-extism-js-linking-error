with `extism-js 1.3.0`

```bash
cd jsplugin && extism-js jsplugin.js -i jsplugin.d.ts -o jsplugin.wasm && cd ../
```

```bash
cargo run
```

See error

```
thread 'main' panicked at src/main.rs:9:10:
called `Result::unwrap()` on an `Err` value: unknown import: `extism:host/env::http_headers` has not been defined
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

This error didn't exist with previous `extism-js`. Updating to `extism 1.9.1` seems to fix it. Also without lock file 1.5.0 seems to work
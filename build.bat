cargo build --target wasm32-unknown-unknown
if %errorlevel% == 0 wasm-bindgen --out-dir html --target web  target\wasm32-unknown-unknown\debug\fungi_rs.wasm
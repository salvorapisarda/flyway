# flyway

A small arcade number block shooter.

## What it does

- You control the yellow player circle.
- Green meteorites fall from the top of the screen.
- Press `Space` to shoot red bullets.
- If a meteorite hits the player, the game ends.
- Press `Space` again on the game-over screen to restart.

## Controls

- `← ↑ ↓ →`: move
- `Space`: shoot / restart after game over

## Tech stack

- Rust
- [`macroquad`](https://crates.io/crates/macroquad)
- Native desktop build with Cargo
- WebAssembly build for browser deployment

## Project layout

- `src/main.rs` — game logic
- `index.html` — local web page template using hosted JS loader
- `deploy/index.html` — deployable web page using local assets
- `deploy/mq_js_bundle.js` — Macroquad JS loader
- `deploy/flyway.wasm` — compiled WebAssembly binary

## Run locally (native)

```bash
cargo run
```

## Build native release

```bash
cargo build --release
```

## Build for the browser

Build the wasm target:

```bash
cargo build --target wasm32-unknown-unknown --release
```

Then copy the generated wasm into the deploy folder:

```bash
cp target/wasm32-unknown-unknown/release/flyway.wasm deploy/flyway.wasm
```

## Run in the browser

Serve the `deploy/` folder over HTTP. Do not open the HTML file directly with `file://`.

```bash
cd deploy
python3 -m http.server 8080
```

Then open:

```text
http://localhost:8080
```

## Deploy folder contents

The browser build expects these files inside `deploy/`:

- `index.html`
- `mq_js_bundle.js`
- `flyway.wasm`

## Troubleshooting

### Black screen

If the page opens but stays black, check these first:

1. `deploy/mq_js_bundle.js` exists.
2. `deploy/flyway.wasm` was rebuilt and copied from the latest release build.
3. You are serving the files over HTTP, not opening `index.html` directly.
4. Browser DevTools shows `200` for both `mq_js_bundle.js` and `flyway.wasm`.
5. Browser DevTools Console shows no JavaScript or wasm load errors.

A quick refresh flow:

```bash
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/flyway.wasm deploy/flyway.wasm
cd deploy
python3 -m http.server 8080
```

### Controls do not respond

Click the canvas once to ensure it has keyboard focus, then try the arrow keys and `Space` again.

## Notes

The game currently uses simple shapes only, so there are no external image or audio assets to package.

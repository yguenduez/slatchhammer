# Slatchhammer

Idea: A webassembly 1v1 football game on one machine

Todo (MVP):
- √ Arena: Bounderies, Floor
- √ To WebAssembly to play in the web with e.g. vercel
- √ Goals: to achieve points
- √ Points
- √ Reset After goal - initial position of players
- √ Add a timer for one game
- √ Win game after time is up (Timer)

Further Ideas:
- Ingame Menu - to manually start a match
- Choose character with different properties (mass,velocity,restitution,...)
- Sprint, which depletes, when using
- Items like:
  - Power-ups
  - Items to handicap oponent

- Ranking System:
- Online 1v1
- Online 3v3
- Skins to buy to support this game


# Docu

## Run this game as webassembly.

First compile it to wasm:

```sh
cargo build --release --target wasm32-unknown-unknown
```

Then build the gluecode for the web to load the wasm:

```sh
wasm-bindgen --out-dir out --target web target/wasm32-unknown-unknown/release/slatchhammer.wasm
```


Then load the `init()` function from the gluecode

```
 
<script type="module">
    import init from './out/slatchhammer.js'
    init()
</script>
```

## Run natively

```shell
cargo run (--release)
```

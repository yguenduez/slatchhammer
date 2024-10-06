# Slatchhammer

Idea: A webassembly 1v1 football game on one machine.

## Play it

You can play it live: [slatchhammer.yguenduez.dev](https://slatchhammer.yguenduez.dev)

Player 1: WASD  
Player 2: Arrow keys

**Have fun!**

## Docs

### Run this game as webassembly.

First compile it to wasm:

```sh
cargo build --release --target wasm32-unknown-unknown
```

Then build the gluecode for the web to load the wasm:

```sh
wasm-bindgen --out-dir out --target web target/wasm32-unknown-unknown/release/slatchhammer.wasm --no-typescript
```

Then load the `init()` function from the gluecode

```

<script type="module">
    import init from './out/slatchhammer.js'
    init()
</script>
```

### Run natively

```shell
cargo run (--release)
```

### How to continue from here

Todo (MVP):

- [x] Arena: Bounderies, Floor
- [x] To WebAssembly to play in the web with e.g. vercel
- [x] Goals: to achieve points
- [x] Points
- [x] Reset After goal - initial position of players
- [x] Add a timer for one game
- [x] Win game after time is up (Timer)
- [x] Sprint, which depletes, when using
- [ ] Ingame Menu - to manually start a match

Further Ideas:

- Choose character with different properties (mass,velocity,restitution,...)
- Items like:

  - Power-ups
  - Items to handicap oponent

- Ranking System:
- Online 1v1
- Online 3v3
- Skins to buy to support this game

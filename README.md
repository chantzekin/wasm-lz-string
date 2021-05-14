# `wasm-lz-string`

- 🌟 A WebAssembly implementation of [`lz-string`](https://github.com/pieroxy/lz-string) decompression
- 🔋 Use rust crate [`lz_string`](https://docs.rs/lz-string/0.1.1/lz_string/)
- 📦 Build with [`wasm-pack`](https://github.com/rustwasm/wasm-pack)


## 🚴 Usage

### 🐑 Use `npm` or `yarn` to install package

```bash
npm install wasm-lz-string --save
# Or use yarn
yarn add wasm-lz-string
```

### 📖 Import the package on nodejs

```js
const LzString = require('wasm-lz-string')

// decode_base64: (source: string): string
LzString.decode_base64('BIUwNmD2A0AEDukBOYAmBCIA'); // Hello, world!

```

## 🍔 Devlopment

### 🛠️ Build with `wasm-pack build`

```bash
wasm-pack build --target nodejs
# ⬆️ Just build for cjs, you can build for esm ⬇️
wasm-pack build
```

The `pkg` directory is the package directory

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`lz_string`](https://docs.rs/lz-string/0.1.1/lz_string/) implementation of lz-string decompression

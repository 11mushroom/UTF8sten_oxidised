# Description
basic CLI tools to work with [utf8sten](https://github.com/11mushroom/UTF8sten_lib_oxidised)
it's Rust rewrite of [C++ version of tools](https://github.com/11mushroom/UTF8sten)

# Web

there is Web [page](https://utf8sten.github.io/) with encoder/decoder
for both v1 and v2 versions, where you can use it right in your browser

Note: _your browser needs to support Wasm_

# Dependencies

- [utf8sten rust library](https://github.com/11mushroom/UTF8sten_lib_oxidised)

# Building

- clone repository if not already cloned
  ```
  git clone https://github.com/11mushroom/UTF8sten_oxidised.git
  ```

- to build encoders and decoders:
  ```
  cargo build --release
  ```

  `encoder`,`encoder2` and `decoder`,`decoder2` binaries will be in `target/release` directory

# usage of encoder and decoder scripts
  ```bash
  ./encoder "your message"
  ```
  it will print text, which can be given to decoder to get your message back
  ```bash
  ./decoder "text which encoder gave you"
  ```

  example:
  ```bash
  $ ./encoder "fabric"
  腦蘦襲蘶
  $ ./decoder "腦蘦襲蘶"
  fabric
  ```
  text which encoder gives you doesn't have any meaning in any language

  also you can pipe data in, eg.
  ```bash
  $ echo "fabric"|./encoder
  腦蘦襲蘶
  $ echo "腦蘦襲蘶"|./decoder
  fabric
  ```
  also you can do this
  ```bash
  $ echo "fabric"|./encoder|./decoder
  fabric

  ```

# usage of encoder2 and decoder2 scripts
  almost the same as usage of `encoder` and `decoder`

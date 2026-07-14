# About
UTF8sten is an encoder that allows to store more data in a smaller amount of characters.

This project is a Rust rewrite of [UTF8sten](https://github.com/11mushroom/UTF8sten.git).

Additionally, a library is provided in the folder [UTF8](UTF8).

# Web
There is a [webpage](https://utf8sten.github.io/) with the encoders and decoders found in this project.

Note: _Your browser needs to support WebAssembly_.

# Building

  ```
  git clone https://github.com/11mushroom/UTF8sten_oxidised.git
  cd example
  cargo build --release
  ```

  `encoder`, `encoder2`, `decoder`, and `decoder2` binaries will be in `example/target/release` directory.

# Usage of encoder and decoder scripts
  ```bash
  ./encoder "original message"
  ```
  It will output text, which can be given to decoder to get your message back:
  ```bash
  ./decoder "encoded message"
  ```

  for example:
  ```bash
  $ ./encoder "fabric"
  腦蘦襲蘶
  $ ./decoder "腦蘦襲蘶"
  fabric
  ```
  Text which the encoder gives you doesn't have any meaning in any language.

  Data can be also piped in:
  ```bash
  $ echo "fabric"|./encoder
  腦蘦襲蘶
  $ echo "腦蘦襲蘶"|./decoder
  fabric
  $ echo "fabric"|./encoder|./decoder
  fabric
  ```

# Usage of encoder2 and decoder2 scripts
  The same as usage of `encoder` and `decoder`.

  `decoder` can decode output of both the `encoder` and `encoder2` scripts.


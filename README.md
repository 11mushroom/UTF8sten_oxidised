# description
tools to store data in unicode symbols
it's [UTF8sten](https://github.com/11mushroom/UTF8sten.git) written in Rust
`UTF8` in repository is a library with tools to store and get data

# Building

- clone repository if not already cloned
  ```
  git clone httos://github.com/11mushroom/UTF8sten_oxidised.git
  ```

- to build encoders and decoders:
  
  go to the `example` directory
  ```
  cd example
  ```

  and build it with cargo
  ```
  cargo build --release
  ```

  `encoder`,`encoder2` and `decoder`,`decoder2` binaries will be in `example/target/release` directory

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

  `decoder` can decode output of both `encoder` and `encoder2`


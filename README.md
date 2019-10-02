See https://github.com/rustwasm/wasm-pack
See also https://rustwasm.github.io/
See also https://rustwasm.github.io/docs/wasm-bindgen/

# wasm-bindgen

You need to depend on this and use it in your code.

# wasm-pack

This is the tool that the Rust WebAssembly Working Group wants you to use for
your workflow.  It used to be mostly about deploying to `npm` but it deploys
everywhere now.

1. Run `wasm-pack build --target web` to build

# web-sys

This package lets you interact with the DOM from rust

# wasm-opt

A tool in the `binaryen` system package, this shrinks .wasm files

  $ wasm-opt -Os -o output.wasm input.wasm

# terser

Numerous javascript minimization programs have been released in the past
including yui-compressor, jsmin, minify, uglifyjs (1,2,and3), uglify-es,
and terser.  Terser is a fork of uglify-es that handles the newest ECMA
versions of javascript, is maintained, and gives the best compression
ratio (I ran tests).  It runs on node; install globally with

  $ npm install terser -g




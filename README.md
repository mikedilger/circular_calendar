# Circular Calendar

This will be a WASM webpage generator that dynamically constructs and SVG showing
a Southern Hemisphere based circular seasonal calendar.

## Install Build Tools

### wasm-pack

This is the tool that the Rust WebAssembly Working Group wants you to use for
your workflow.  It used to be mostly about deploying to `npm` but it deploys
everywhere now.

1. Run `wasm-pack build --target web` to build

### wasm-opt

A tool in the `binaryen` system package, this shrinks .wasm files

  $ wasm-opt -Os -o output.wasm input.wasm

### terser

Numerous javascript minimization programs have been released in the past
including yui-compressor, jsmin, minify, uglifyjs (1,2,and3), uglify-es,
and terser.  Terser is a fork of uglify-es that handles the newest ECMA
versions of javascript, is maintained, and gives the best compression
ratio (I ran tests).  It runs on node; install globally with

  $ npm install terser -g

## Building

Run `make` and then `make deploy`.

Output will be in `deploy/` folder, ready to serve from a webserver or via
a `file:///` URL.

## Technology and Notes

WASM related libraries used:

* wasm-bindgen
* web-sys

See also:

* https://github.com/rustwasm/wasm-pack
* https://rustwasm.github.io/
* https://rustwasm.github.io/docs/wasm-bindgen/

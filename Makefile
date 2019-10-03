PKG=circular_calendar

.PHONY: all deploy clean

all:
	wasm-pack build --target web --no-typescript --release

deploy:
	mkdir -p deploy
	wasm-opt -Os -o deploy/$(PKG)_bg.wasm pkg/$(PKG)_bg.wasm
	minify websrc/index.html > deploy/index.html
	minify websrc/simple.css > deploy/a.css
	terser pkg/$(PKG).js -c -m > deploy/$(PKG).js
	terser websrc/module.js -c -m --module > deploy/module.min.js

clean:
	rm -r pkg/ target/ deploy/

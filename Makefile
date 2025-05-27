.PHONY: wasm wasm-release serve lint dev release build test

wasm:
	wasm-pack build --out-dir client/pkg/test-wasm --out-name index --target web --dev

wasm-release:
	wasm-pack build --out-dir client/pkg/test-wasm --out-name index --target web --release

serve:
	cd client && npm run dev

lint:
	cd client && npm run lint

dev: wasm serve

release: wasm-release serve

build: wasm-release
	cd client && npm run build

test:
	wasm-pack test --headless --chrome


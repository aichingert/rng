build:
	wasm-pack build --target no-modules --out-dir www/pkg

host-py: build
	python3 -m http.server 8080 -d ./www

host-npx: build
	npx http-server www -p 8080

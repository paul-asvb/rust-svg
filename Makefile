build:
	wasm-pack build --target web --release

dev:
	cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build" 

serve: 
	basic-http-server -a 0.0.0.0:4000

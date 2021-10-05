.PHONY: web

web:
	wasm-pack build --target web
	cp index.html pkg/
	mkdir -p pkg/assets/
	cp assets/*.png pkg/assets
	cd pkg && zip -r chevalchemy.zip *
	mv pkg/chevalchemy.zip ./

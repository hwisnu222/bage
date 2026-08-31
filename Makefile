.PHONY: encrypt decrypt


encrypt:
	cargo run -- encrypt -p ./assets

decrypt:
	cargo run -- decrypt -p ./assets

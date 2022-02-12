.PHONY: frontend backend release

default: backend

frontend:
	cd onair-fe && npm run build

backend: frontend
	cd onair-be && cargo run

release: frontend
	cd onair-be && cargo build --release

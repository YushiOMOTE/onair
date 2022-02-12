.PHONY: setup frontend backend release

default: backend

setup:
	cd onair-fe && npm install

frontend:
	cd onair-fe && npm run build

backend: frontend
	cd onair-be && cargo run

release: frontend
	cd onair-be && cargo build --release

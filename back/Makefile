ENV ?= dev


clean:
	cargo clean

build-ws:
	cargo lambda build -p gomoku-ws --release --target=x86_64-unknown-linux-musl
	upx --best target/lambda/gomoku-ws/bootstrap 
deploy-ws:
	cargo lambda deploy --include Settings.toml:./gomoku-ws/Settings.toml --binary-name gomoku-ws gomoku-ws-$(ENV)

build-api:
	cargo lambda build -p gomoku-api --release --target=x86_64-unknown-linux-musl
	upx --best target/lambda/gomoku-api/bootstrap 
deploy-api:
	cargo lambda deploy --include Settings.toml:./gomoku-api/Settings.toml --binary-name gomoku-api gomoku-api-$(ENV)
	
build-q:
deploy-q:
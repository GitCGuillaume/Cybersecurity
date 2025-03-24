all:
	cargo build --release
	cp ./target/release/spider .

leak:
	RUSTFLAGS=-Zsanitizer=address \
		cargo +nightly build -Zbuild-std --target x86_64-unknown-linux-gnu

clean:
	cargo clean
	rm -rf ./data

fclean:
	cargo clean
	rm -rf ./data
	rm -f ./spider

re:
	fclean all

.PHONY: all leak clean fclean re
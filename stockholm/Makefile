all:
	cargo build --release

leak:
	RUSTFLAGS=-Zsanitizer=address \
		cargo +nightly build -Zbuild-std --target x86_64-unknown-linux-gnu

clean:
	cargo clean

fclean:
	cargo clean

re:
	fclean all

test:
	cargo test

.PHONY: all leak clean fclean re test

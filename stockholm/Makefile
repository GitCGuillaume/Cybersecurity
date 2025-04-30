all:
	cargo build --release
	cp ./target/release/stockholm .

leak:
	RUSTFLAGS=-Zsanitizer=address \
		cargo +nightly build -Zbuild-std --target x86_64-unknown-linux-gnu

race:
	RUSTFLAGS=-Zsanitizer=thread RUSTDOCFLAGS=-Zsanitizer=thread \
		cargo +nightly build -Zbuild-std --target x86_64-unknown-linux-gnu

clean:
	cargo clean

fclean:
	cargo clean
	rm -f ./stockholm

re:
	fclean all

.PHONY: all leak race clean fclean re

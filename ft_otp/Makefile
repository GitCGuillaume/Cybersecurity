all:
	cargo build --release
	cp ./target/release/ft_otp .

leak:
	RUSTFLAGS=-Zsanitizer=address \
		cargo +nightly build -Zbuild-std --target x86_64-unknown-linux-gnu

clean:
	cargo clean

fclean:
	cargo clean
	rm -f ft_otp
	rm -f ft_otp.key

re:
	fclean all

.PHONY: all leak clean fclean re

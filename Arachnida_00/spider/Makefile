all:
	cargo build --release
	cp ./target/release/spider .

leak:
	RUSTFLAGS=-Zsanitizer=address \
		rustup run nightly-x86_64-unknown-linux-gnu cargo +nightly build -Zbuild-std --target x86_64-unknown-linux-gnu

clean:
	cargo clean
	rm -rf ./data

fclean:
	cargo clean
	rm -rf ./data
	rm -f ./spider

re:
	fclean all

show:
	@echo "https://demo.cyotek.com/html/elements/img.php"
	@echo "https://webscraper.io/test-sites"

.PHONY: all leak clean fclean re show
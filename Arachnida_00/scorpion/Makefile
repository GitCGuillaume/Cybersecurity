all:
	cargo build --release
	cp ./target/release/scorpion .

download:
	rm -rf ./exif
	rm -rf ./no_exif
	mkdir -p exif
	mkdir -p no_exif
	wget -P ./no_exif https://fr.wikipedia.org/static/images/icons/wikipedia.png
	wget -P ./no_exif https://upload.wikimedia.org/wikipedia/commons/3/3f/JPEG_example_flower.jpg
	wget -P ./no_exif https://upload.wikimedia.org/wikipedia/commons/8/8a/AnalogClockAnimation1_2hands_1h_in_6sec.gif
	wget -P ./no_exif https://samples-files.com/samples/images/bmp/480-360-sample.bmp
	wget -P ./exif https://upload.wikimedia.org/wikipedia/commons/3/3d/Solmeta_Geotagger_N2_Kompass_GPS_on_Nikon_D300.jpg

leak:
	RUSTFLAGS=-Zsanitizer=address \
		cargo +nightly build -Zbuild-std --target x86_64-unknown-linux-gnu

clean:
	cargo clean

fclean:
	cargo clean
	rm -rf ./exif
	rm -rf ./no_exif
	rm -f scorpion

re:
	fclean all

.PHONY: all download leak clean fclean re
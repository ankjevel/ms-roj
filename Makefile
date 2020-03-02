INSTALL=install
INSTALL_PROGRAM=$(INSTALL)
INSTALL_DATA=$(INSTALL) -m 644

bin_dir=$(HOME)/.local/bin
data_dir=$(HOME)/.local/share
name=com.github.Iteam13337.ms-roj

.PHONY : clean install uninstall

target/release/ms-roj : src
	cargo build --release

install : target/release/ms-roj
	# Install binary
	$(INSTALL_PROGRAM) target/release/ms-roj $(bin_dir)/$(name)


	# Install icon(s)
	# $(INSTALL_DATA) resources/$(name).svg $(data_dir)/icons/hicolor/scalable/apps/$(name).svg
	# $(INSTALL_DATA) resources/$(name).64.png $(data_dir)/icons/hicolor/64x64/apps/$(name).png
	# $(INSTALL_DATA) resources/$(name).128.png $(data_dir)/icons/hicolor/128x128/apps/$(name).png

	# Force icon cache refresh
	# touch $(data_dir)/icons/hicolor

	# Install desktop file
	$(INSTALL_DATA) resources/$(name).desktop $(data_dir)/applications/$(name).desktop

uninstall :
	# Remove the desktop file
	rm -f $(data_dir)/applications/$(name).desktop

	# Remove the icon(s)
	# rm -f $(data_dir)/icons/hicolor/scalable/apps/$(name).svg
	# rm -f $(data_dir)/icons/hicolor/64x64/apps/$(name).png
	# rm -f $(data_dir)/icons/hicolor/128x128/apps/$(name).png

	# Remove the binary
	rm -f $(bin_dir)/bin/$(name)

clean :
	cargo clean


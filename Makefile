INSTALL=install
INSTALL_PROGRAM=$(INSTALL)
INSTALL_DATA=$(INSTALL) -m 644

bin_dir=$(HOME)/.local/bin
data_dir=$(HOME)/.local/share
name=com.github.ankjevel.ms-roj

all: build

.PHONY: app build clean release

build:
	@cargo build

clean:
	@cargo clean

release:
	@cargo build --release

app:
	@cd bundle/app && xcodebuild

target/release/ms-roj: src
	cargo build --release

install: target/release/ms-roj
	# Install binary
	$(INSTALL_PROGRAM) target/release/ms-roj $(bin_dir)/$(name)

	# Install desktop file
	$(INSTALL_DATA) bundle/$(name).desktop $(data_dir)/applications/$(name).desktop

uninstall:
	# Remove the desktop file
	rm -f $(data_dir)/applications/$(name).desktop

	# Remove the binary
	rm -f $(bin_dir)/bin/$(name)


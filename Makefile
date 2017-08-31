LIB_DIR=lib

PANDOC_VERSION=1.19.2.1
WKTOX_VERSION=0.12.4

PANDOC_DL=pandoc-${PANDOC_VERSION}-1-amd64.deb
WKTOX_DL=wkhtmltox-${WKTOX_VERSION}_linux-generic-amd64.tar.xz


build:
	@cargo build
	cp $(LIB_DIR)/* target/debug/

release:
	@cargo build --release
	cp $(LIB_DIR)/* target/release/

lib_path:
	@rm -rf $(LIB_DIR)
	@mkdir -p $(LIB_DIR)
	@rm -f /usr/lib/libsciter-gtk-64.so

pandoc: lib_path
	wget https://github.com/jgm/pandoc/releases/download/${PANDOC_VERSION}/${PANDOC_DL} -O $(LIB_DIR)/$(PANDOC_DL)
	ar p $(LIB_DIR)/$(PANDOC_DL) data.tar.gz | tar xz --strip-components 2 -C $(LIB_DIR)
	mv $(LIB_DIR)/bin/* $(LIB_DIR)
	cd $(LIB_DIR) && rm -r bin/ share/ $(PANDOC_DL)

wktox: lib_path
	wget https://github.com/wkhtmltopdf/wkhtmltopdf/releases/download/${WKTOX_VERSION}/${WKTOX_DL} -O $(LIB_DIR)/$(WKTOX_DL)
	tar -xJf $(LIB_DIR)/$(WKTOX_DL) -C $(LIB_DIR)
	mv $(LIB_DIR)/wkhtmltox/lib/* $(LIB_DIR)
	cd $(LIB_DIR) && rm -r wkhtmltox/ $(WKTOX_DL)

sciter: lib_path
	wget https://sciter.com/sdk/sciter-sdk.zip -O $(LIB_DIR)/sciter.zip
	unzip -j $(LIB_DIR)/sciter.zip "bin.gtk/libsciter-gtk-64.so" -d $(LIB_DIR)
	ln -sfP $(LIB_DIR)/libsciter-gtk-64.so /usr/lib/libsciter-gtk-64.so
	rm $(LIB_DIR)/sciter.zip

lib: pandoc wktox sciter

test:
	cargo test
	cargo test --release

.PHONY: build pandoc lib_path wktox

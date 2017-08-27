LIB_DIR=lib

PANDOC_VERSION=1.19.2.1
WKTOX_VERSION=0.12.4

PANDOC_DL=pandoc-${PANDOC_VERSION}-1-amd64.deb
WKTOX_DL=wkhtmltox-${WKTOX_VERSION}_linux-generic-amd64.tar.xz

lib_path:
	@rm -rf $(LIB_DIR)
	@mkdir -p $(LIB_DIR)

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

lib: pandoc wktox

.PHONY: pandoc lib_path wktox


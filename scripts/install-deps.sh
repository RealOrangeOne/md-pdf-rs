#!/usr/bin/env bash

set -e

LIB_DIR=lib/

PANDOC_VERSION="1.19.2.1"
WKTOX_VERSION="0.12.4"

PANDOC_DL="pandoc-${PANDOC_VERSION}-1-amd64.deb"
WKTOX_DL="wkhtmltox-${WKTOX_VERSION}_linux-generic-amd64.tar.xz"

rm -rf $LIB_DIR
mkdir -p $LIB_DIR
cd $LIB_DIR

echo "> Downloading Pandoc..."
wget https://github.com/jgm/pandoc/releases/download/${PANDOC_VERSION}/${PANDOC_DL}

echo ">> Unpacking pandoc binaries..."
ar p $PANDOC_DL data.tar.gz | tar xz --strip-components 2 -C .
mv bin/* .

echo ">> Cleaning up..."
rm -r bin/ share/ $PANDOC_DL


echo "> Downloading wkhtmltox..."
wget https://github.com/wkhtmltopdf/wkhtmltopdf/releases/download/${WKTOX_VERSION}/${WKTOX_DL}

echo ">> Unpacking wkhtmltox..."
tar -xJf $WKTOX_DL
mv wkhtmltox/lib/* .

echo ">> Cleaning up..."
rm -r wkhtmltox/ $WKTOX_DL

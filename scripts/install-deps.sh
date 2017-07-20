#!/usr/bin/env bash

set -e

mkdir -p lib/
cd lib/

echo "> Downloading Pandoc..."
wget https://github.com/jgm/pandoc/releases/download/1.19.2.1/pandoc-1.19.2.1-1-amd64.deb
ar p pandoc-1.19.2.1-1-amd64.deb data.tar.gz | tar xz --strip-components 2 -C .
mv bin/* .
rm -r bin/ share/ pandoc-1.19.2.1-1-amd64.deb

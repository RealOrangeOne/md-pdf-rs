#!/usr/bin/env bash

# 'stolen' from https://github.com/JazzCore/python-pdfkit/blob/master/travis/before-script.sh

set -e

apt-get install -y openssl build-essential libssl-dev
wget https://downloads.wkhtmltopdf.org/0.12/0.12.4/wkhtmltox-0.12.4_linux-generic-amd64.tar.xz
tar -xJf wkhtmltox-0.12.4_linux-generic-amd64.tar.xz
cd wkhtmltox
chown root:root bin/wkhtmltopdf
cp -r * /usr/

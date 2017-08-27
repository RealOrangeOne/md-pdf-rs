#!/usr/bin/env bash

# https://github.com/JazzCore/python-pdfkit/wiki/Using-wkhtmltopdf-without-X-server

set -e

apt-get install xvfb -y

wget https://downloads.wkhtmltopdf.org/0.12/0.12.4/wkhtmltox-0.12.4_linux-generic-amd64.tar.xz
tar -xJf wkhtmltox-0.12.4_linux-generic-amd64.tar.xz
cd wkhtmltox
chown root:root bin/wkhtmltopdf
cp -r * /usr/

echo -e '#!/bin/bash\nxvfb-run -a --server-args="-screen 0, 1024x768x24" /usr/bin/wkhtmltopdf $*' > /usr/bin/wkhtmltopdf.sh
chmod a+x /usr/bin/wkhtmltopdf.sh
ln -s /usr/bin/wkhtmltopdf.sh /usr/local/bin/wkhtmltopdf

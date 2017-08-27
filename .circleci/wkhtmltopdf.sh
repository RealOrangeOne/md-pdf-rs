#!/usr/bin/env bash

# https://github.com/JazzCore/python-pdfkit/wiki/Using-wkhtmltopdf-without-X-server

set -e

apt-get install wkhtmltopdf xvfb -y
echo -e '#!/bin/bash\nxvfb-run -a --server-args="-screen 0, 1024x768x24" /usr/bin/wkhtmltopdf -q $*' > /usr/bin/wkhtmltopdf.sh
chmod a+x /usr/bin/wkhtmltopdf.sh
ln -s /usr/bin/wkhtmltopdf.sh /usr/local/bin/wkhtmltopdf

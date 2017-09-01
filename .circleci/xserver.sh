#!/usr/bin/env bash

set -e

xvfb-run -a --server-args="-screen 0 800x600x24" $@

#!/usr/bin/env bash
set -euo pipefail

if [ -e /tmp/onair_cli_detect_test ]; then
    echo "test"
    exit 1
fi

exit 0

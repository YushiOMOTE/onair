#!/usr/bin/env bash

if [ "$(ps aux | grep zoom | grep cpt)" != "" ]; then
    echo "zoom"
    exit 1
fi

exit 0

#!/usr/bin/env bash

pid=$(ps aux | grep slack | grep AudioService | awk '{ print $2 }')

if [ "$pid" == "" ]; then
    exit 0
fi

count=$(ps -M $pid | wc -l)

if [ $count -gt 10 ]; then
    echo "huddle"
    exit 1
fi

exit 0

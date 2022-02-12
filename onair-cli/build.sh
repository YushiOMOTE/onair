#!/usr/bin/env bash

poetry run pyinstaller --add-data='onair_cli/detectors/*:detectors' onair_cli/onair_cli.py

#!/usr/bin/env python

import time
import subprocess
import glob
import requests
import json
import os
from os.path import dirname, realpath


SCRIPT = dirname(realpath(__file__))
SERVER = os.environ.get("SERVER", "192.168.3.200:8080")
ENDPOINT = f"http://{SERVER}/state"


def detect():
    found = False

    for f in glob.glob(f"{SCRIPT}/detectors/*.sh"):
        if subprocess.run(f).returncode:
            found = True

    return found


def update():
    onair = detect()
    headers = {"content-type": "application/json"}
    data = {"onair": onair}
    try:
        res = requests.post(ENDPOINT, headers=headers, data=json.dumps(data))
    except Exception as e:
        print(e)


while True:
    update()
    time.sleep(5)

#!/usr/bin/env bash

python3 -m venv venv
. venv/bin/activate
pip install -r music21/requirements.txt
python -m test
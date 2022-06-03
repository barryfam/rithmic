#!/usr/bin/env python3

from pathlib import Path
from subprocess import run

for p in Path(__file__).parent.glob('*.rs'):
    run(['cargo', 'run', '--example', p.stem], check=True)

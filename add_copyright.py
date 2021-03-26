#!/usr/bin/env python

import os
import argparse
import pathlib
import re

parser = argparse.ArgumentParser()
parser.add_argument("file", type=str)
parser.add_argument("dir", type=str)
args = parser.parse_args()
out = pathlib.Path(args.dir) / args.file

copyright = copyright_line = ""
with open("copyright") as f:
    copyright = f.read()
for line in copyright.splitlines():
    if '(C)' in line:
        copyright_line = line.strip("/*/\r\n").rstrip(" ")
        break

with open(args.file) as input, out.open('w') as o:
    for line in input:
        if re.search(r"\@\@copyright\@\@", line):
            o.write(copyright)
            continue
        out_line = re.sub(r"\@\@copyright_oneline\@\@", copyright_line, line)
        o.write(out_line)

    out.chmod(0o555 if os.access(out, os.X_OK) else 0o444)

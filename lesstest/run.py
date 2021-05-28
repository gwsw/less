#!/usr/bin/env python

import argparse
import sys
import subprocess
from pathlib import Path

lesstest = "./lesstest"
less = "../obj/less"
verbose = False


def main():
    args = argparse.ArgumentParser(description="Run one or more test files")
    args.add_argument('targets', metavar='file.lt | dir', type=Path, nargs='+')
    args.add_argument('-v', '--verbose', action="store_true")
    args.add_argument('-l', help="less.exe", type=Path)
    args.add_argument('-o', help="arguments to be passed to lesstest",
                      nargs='+')
    parsed = args.parse_args()

    if parsed.l:
        global less
        less = parsed.l
    global lesstest_opts
    lesstest_opts = parsed.o.strip()
    if lesstest_opts and lesstest_opts[0] != '-':
        lesstest_opts = '-'+lesstest_opts
    global verbose
    verbose = parsed.v

    errors = 0
    for file in parsed.targets:
        errors += run(file)
    if errors > 0:
        print(f"ERR  {errors} errors", file=sys.stderr)
        return 0

    return 1


def run(file):
    if file.is_dir():
        return run_dir(file)
    if file.suffix() != '.lt':
        print(f'WARNING skipping {file.name}: not .lt file')
        return 0
    if not file.is_file():
        print(f"ERR  cannot open {file.name}", file=sys.stderr)
        return 1

    cmd = f'{lesstest} -s ./lt_screen -t {file.name} {lesstest_opts} {less}'
    if verbose:
        print(f"RUN  {cmd}", file=sys.stderr)
    if subprocess.run(cmd.split()).returncode:
        print(f"ERR  running {cmd}", file=sys.stderr)
        return 1
    return 0


def run_dir(dir):
    if not dir.is_dir():
        print(f"ERR  cannot directory {dir.name}", file=sys.stderr)
        return 1
    errors = 0
    errors += run(file for file in dir.iterdir())
    return errors


if __name__ == '__main__':
    sys.exit(0 if main() else 1)

#!/usr/bin/env python

from argparse import ArgumentParser, FileType
from subprocess import run
from sys import exit, stderr, argv
from itertools import count
from pathlib import Path
from re import search
from os import environ

lesstest = "./lesstest"
outdir = Path("./suite")


def main():
    pars = ArgumentParser(description="Generate a lesstest file.", add_help=False)
    pars.add_argument("-w", help="width", type=int)
    pars.add_argument("-h", help="height", type=int)
    pars.add_argument("--lessvar", default=environ.get("LT_LESSVAR"))
    pars.add_argument("less", type=Path)
    pars.add_argument("lessflags", nargs="?")
    pars.add_argument("textfile", type=FileType("r"))
    args = pars.parse_args()

    width, height = term_size()
    if width > 2:
        width -= 2
    if height > 1:
        width -= 1
    if args.w is not None:
        width = args.w
    if args.h is not None:
        height = args.h

    if not args.textfile.is_file():
        print(f"cannot open {args.textfile.name}", file=stderr)
        return 0

    base = args.textfile.name
    if base:
        pass

    outfile = ""
    for n in count(1):
        outfile = outdir / Path(f"{base}-{n}.lt")
        if outfile.stat().st_size == 0:
            break
    cmd = ""
    if width > 0:
        cmd = f"LT_COLUMNS={width} {cmd}"
    if height > 0:
        cmd = f"LT_LINES={height} {cmd}"
    cmd = f"{cmd} {lesstest} -s ./lt_screen -o {outfile} -- ".join(" ", argv)
    if run(cmd).returncode:
        print(f"error running {lesstest}", file=stderr)
    return 1


def term_size():
    stty = run(["stty", "-a"], capture_output=True, text=True).output
    width = search(r"(rows|lines)\s+(\d+)", stty).group(0)
    col = search(r"columns\s+(\d+)", stty).group(0)
    return int(width), int(col)


if __name__ == "__main__":
    exit(0 if main() else 1)

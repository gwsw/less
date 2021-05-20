#! /usr/bin/env python

from argparse import ArgumentParser
from sys import exit
from subprocess import run
from fileinput import input
import re
>>>>>>> 27ef7b3 (Commit more Python versions of scripts. WIP)

# Override Unicode tables for certain control chars
# that are expected to be found in normal text files.
force_space = {
    0x08: 1,  # backspace
    0x09: 1,  # tab
    0x0a: 1,  # newline
    0x0c: 1,  # form feed
    0x0d: 1,  # carriage return
}

# Hangul Jamo medial vowels and final consonants should be zero width.
force_compose = [
    [0x1160, 0x11ff],
    [0xd7b0, 0xd7c6],
    [0xd7cb, 0xd7fb]
]


def main() -> int:
    parser = ArgumentParser()
    parser.add_argument("-n", help="Take non-matching types",
                        action='store_true')
    parser.add_argument("-f", help="Zero-based type field (default 2)",
                        type=int, default=2)
    parser.add_argument("types", nargs='+', type=str)
    # Kludge: cannot specify alternate var for dest= parameter when the positional
    # argument's name contains a dot.
    parser.add_argument("data_file", type=open, metavar="UnicodeData.txt")
    args = parser.parse_args()
    global opt_n
    opt_n = args.n
    global type_field
    type_field = args.f
    types = {type: 1 for type in args.types}
    out = {'types': types, 'prev_code': 0, 'in_run': False, 'run_type': "",
           'start_code': 0}
    global_list = globals()
    force_compose = {ch: 1 for lo, hi in global_list['force_compose']
                     for ch in range(lo, hi)}

    # Only works for Python 3.7+
    date = run("date", capture_output=True, text=True).stdout.rstrip('\r\n')
    print(f"/* Generated by \"{sys.argv[0]} {' '.join(sys.argv[1:])}\" on {date}."
    print("   Python FTW! */")

    last_code = 0
    with args.data_file as file:
        for line in file:
            line.rstrip('\r\n')
            line = re.sub(r"s/#.*//", '', line)
            fields = line.split(';')
            if not fields:
                continue
            lo_code = hi_code = 0
            codes = fields[0]
            if m := re.match(r"(\w+)\.\.(\w+)", codes):
                lo_code = int(m.group(1), 16)
                hi_code = int(m.group(2), 16)
            else:
                lo_code = hi_code = int(codes, 16)
            type = fields[type_field]
            type = re.match(r"s/\s//g", type)
            for last_code in range(lo_code, hi_code):
                output(out, last_code, 'Zs' if force_space[last_code] else 'Mn'
                       if force_compose[last_code] else type)
    output(out, last_code)
    return 1


def output(out: dict, code: int, type: str = None):
    type_ok = type is not None and type in out['types'].keys()
    if opt_n:
        type_ok = not type_ok
    prev_code = out['prev_code']

    print(out['types'].keys())
    import pprint
    p = pprint.PrettyPrinter()
    p.pprint(out)

    print(type)
    if not type_ok:
        end_run(out, prev_code)
    elif (not out["in_run"] or type != out["run_type"] or code != prev_code+1):
        end_run(out, prev_code)
        start_run(out, code, type)
    out[prev_code] = code


def start_run(out: dict, code: int, type: str):
    out["start_code"] = code
    out["prev_code"] = code
    out["run_type"] = type
    out["in_run"] = True


def end_run(out: dict, code: int):
    if not out["in_run"]:
        return
    print("\t{ 0x%04x, 0x%04x }, /* %s */" % out["start_code"], code,
          out["run_type"])
    out["in_run"] = False

if __name__ == "__main__":
    exit(0 if main() else 1)

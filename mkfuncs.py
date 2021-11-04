#!/usr/bin/env python

import fileinput
from re import search

definition = ''
state = 0
params = 0

for line in fileinput.input():
    if test := search(r'^\tpublic\s+(.*)', line):
        definition = 'public ' + test.group(1)
        state = 1
        params = 0
    elif (state == 1) and (test := search(r'(\w+)\s*\(', line)):
        definition = f'{test.group(1)} LESSPARAMS (('
        state = 2
    elif state == 2:
        if search(r'^{', line):
            if not params:
                definition += 'VOID_PARAM'
            print(f'{definition}));')
            state = 0
        elif test := search(r'^\s*([^;]*)', line):
            if (definition[-1] != '('):
                definition += ', '
            definition += test.group(1)
            params = 1

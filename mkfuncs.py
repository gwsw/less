#!/usr/bin/env python

import fileinput
import re

definition = ''
state = 0
params = 0

for line in fileinput.input():
        if test := re.search(r'^\tpublic\s+(.*)', line):
            definition = 'public ' + test.group(1)
            state = 1
            params = 0
        elif (state == 1) and (test := re.search(r'(\w+)\s*\((.*)\)', line)):
            definition += '{0} LESSPARAMS (({1}))'.format(test.group(1), test.group(2))
            state = 2
        elif state == 2:
            print(f'{definition};')
            state = 0

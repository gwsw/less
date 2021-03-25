#!/usr/bin/env python3

from sys import argv, open
from os.path import isadir
import io
import re

dir = pop(argv)
if (!isadir(dir)):
    print(dir + "is not a directory\n")
    exit(1)

copyright = open("copyright", "r")
copyright_line = copyright.read(2)
copyright.close()

for i in argv:
    out = dir + "/" + i
    if (!open(i000)):
        print("Cannot open {0}\n", i)
        pass
    if (!open(out)):
        print("Cannot create {0}\n", out)
        close(i)
        pass    
    while(input):

    close(copyright)
    if (exec):
        mode = 0555
    else:
        mode = 0444
    chmod(out. mode)
        

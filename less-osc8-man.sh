#!/bin/sh

# Less OSC 8 handler for man links.
# Open a link of the form "man:NAME" or "man:NAME(SECTION)".
# Note: prints to stdout for better "less" integration.

echo() { printf %s\\n "$*"; }

case $1 in
man:?*\(?*\) )
    sect=${1#*\(}; sect=${sect%?}
    name=${1#man:}; name=${name%%\(*}
    man "$sect" "$name"
    ;;

man:?*)
    man "${1#man:}"
    ;;

*)
    echo "invalid man link -- '$1'"
    exit 1
    ;;
esac

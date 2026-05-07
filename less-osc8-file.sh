#!/bin/sh

# Less OSC 8 handler for file links.
# Open a link of the form "file://HOST/PATH" or "file:///PATH"
# Note: prints to stdout for better "less" integration.

echo() { printf %s\\n "$*"; }

case $1 in
file:///* | file://localhost/*)
    less -- "/${1#file://*/}"
    ;;

file://*/*)
    linkhost=${1#file://}; linkhost=${linkhost%%/*}
    localhost=${HOSTNAME:-$(bash -c 'printf %s "$HOSTNAME"' || uname -n)} 2>/dev/null

    case $localhost in
    */* | '')
        echo "unable to detect local hostname -- '$localhost'"
        exit 1
        ;;
    "$linkhost")
        less -- "/${1#file://*/}"
        ;;
    *)
        echo "cannot open remote file -- '$1'"
        exit 1
    ;;
    esac
    ;;

*)
    echo "invalid file link -- '$1'"
    exit 1
    ;;
esac

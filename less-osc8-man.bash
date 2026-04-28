#!/usr/bin/env bash
# Less OSC 8 handler for man links.
# Open a link of the form "man:NAME" or "man:NAME(SECTION)".

if [[ "$1" =~ ^man:([^\(]+)(\((.+)\))?$ ]]; then
    _NAME="${BASH_REMATCH[1]}"
    _SECT="${BASH_REMATCH[3]}"
    if [[ -z "$_SECT" ]]; then
        man "$_NAME"
    else
        man "$_SECT" "$_NAME"
    fi
else
    echo "Invalid man link"
fi

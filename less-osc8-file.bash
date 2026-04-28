#!/usr/bin/env bash
# Less OSC 8 handler for file links.
# Open a link of the form "file://HOST/PATH".

if [[ "$1" =~ ^file://([^/]+)(/.+)$ ]]; then
    _HOST="${BASH_REMATCH[1]}"
    _PATH="${BASH_REMATCH[2]}"
    if [[ "$_HOST" = localhost || "$_HOST" = "$HOSTNAME" ]]; then
        less "$_PATH"
    else
        echo "Cannot open remote file on $_HOST"
    fi
else
    echo "Invalid file link"
fi

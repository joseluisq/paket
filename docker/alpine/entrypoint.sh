#!/bin/sh

set -e

# Check if incomming command contains flags.
if [ "${1#-}" != "$1" ]; then
    set -- fish -c "paket $@"
fi

exec "$@"

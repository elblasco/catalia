#!/usr/bin/env bash

set -euo pipefail

RED='\033[0;31m'
PUR='\033[0;35m'
GRE='\033[0;32m'
NC='\033[0m' # No Color
FILENAME="even.smt2"
LOGFILE="log.txt"

function loading_animation() {
    local pid=$1
    local delay=0.5
    local spin='/-\|'
    local i=0

    while kill -0 $pid 2>/dev/null; do
        i=$(( (i+1) % 4 ))
        printf "\rProcessing... ${spin:i:1}"
        sleep $delay
    done
	echo ""
}

RUSTFLAGS="-A warnings" cargo build

echo -e "${GRE}Starting execution${NC} with $FILENAME"

./target/debug/catalia -vvvv "adts/$FILENAME" > "$LOGFILE" &

loading_animation $!

echo -e "${GRE}Log produced${NC} in $LOGFILE"

less "$LOGFILE"

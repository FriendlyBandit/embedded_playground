#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=pi@beepboops.local
readonly TARGET_PATH=/home/pi/reaction_game_rpi_rust
readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/reaction_game_rpi_rust
readonly PASS=pi2042

cargo build --release --target=${TARGET_ARCH}
sshpass -p ${PASS} rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
sshpass -p ${PASS} ssh -t ${TARGET_HOST} ${TARGET_PATH}
#!/bin/bash
#
# run locally
#

set -o errexit
set -o pipefail
set -o nounset


SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

echo "INFO: build started at $(date -u +%Y-%m-%dT%H:%M:%SZ)"

echo "INFO: dir=${SCRIPT_DIR}"
cd "${SCRIPT_DIR}"

if [ ! -d "${SCRIPT_DIR}/node_modules" ]; then
    echo "INFO: installing javascript dependencies"
    npm install
fi

npm run build

echo "INFO: build started at $(date -u +%Y-%m-%dT%H:%M:%SZ)"

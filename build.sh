#!/usr/bin/env bash
#
# build locally for testing
#

set -o errexit
set -o pipefail
set -o nounset

ENV_FILE="${1:-./.env}"
if [ -f "${ENV_FILE}" ]; then
    echo "INFO: loading '${ENV_FILE}'!"
    export $(cat "${ENV_FILE}")
fi

export VERSION=${VERSION:-local}

export LASTMOD=$(date -u +%Y-%m-%dT%H:%M:%SZ)

if [[ $(git status --short) != '' ]]; then
  export COMMIT="$(git rev-parse --short HEAD) (dirty)"
else
  export COMMIT="$(git rev-parse --short HEAD)"
fi

export BUILTBY="build.sh"


echo "INFO: build started at $(date -u +%Y-%m-%dT%H:%M:%SZ)"

cargo build --release

echo "INFO: build complete at $(date -u +%Y-%m-%dT%H:%M:%SZ)"

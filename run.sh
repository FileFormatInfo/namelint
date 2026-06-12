#!/usr/bin/env bash
#
# run locally for dev
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

export BUILTBY="run.sh"

# install https://github.com/canop/bacon
if ! command -v bacon &> /dev/null; then
	echo "INFO: 'bacon' not found, installing..."
	cargo install --locked bacon
fi

# run the app
bacon run -- --bin namelint -- --version --verbose

#!/usr/bin/env bash
#
# make a new release by pushing a new tag to git
#

set -o errexit
set -o pipefail
set -o nounset

echo "INFO: Starting release process at $(date -u +%Y-%m-%dT%H:%M:%SZ)"

TAG=${1:-DEFAULT}

if [[ "$TAG" == "DEFAULT" ]]; then
	echo "INFO: No tag provided, calculating next version based on git tags"
	# get tags from remote
	git fetch --tags
	# get the latest tag from git and increment the patch version
	LATEST_TAG=$(git tag --sort=v:refname | tail -1)
	echo "INFO: Latest tag is '${LATEST_TAG}'"
	# if none, start at 0.1.0
	if [[ -z "$LATEST_TAG" ]]; then
		TAG="0.1.0"
	else
		IFS='.' read -r -a VERSION_PARTS <<< "$LATEST_TAG"
		MAJOR=${VERSION_PARTS[0]}
		MINOR=${VERSION_PARTS[1]}
		PATCH=${VERSION_PARTS[2]}
		PATCH=$((PATCH + 1))
		TAG="$MAJOR.$MINOR.$PATCH"
	fi
fi

echo "INFO: Releasing version ${TAG}"

git tag -a "$TAG" -m "Release version ${TAG}"
git push origin "$TAG"

echo "INFO: Completed release process at $(date -u +%Y-%m-%dT%H:%M:%SZ)"

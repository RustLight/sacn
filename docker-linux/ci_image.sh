#!/usr/bin/env bash
# Gets CI's test environment image, the `env` stage of docker-linux/Dockerfile,
# and tags it sacn-test-env for the test jobs.
#
# The image is published to the GitHub Container Registry, tagged with a hash of
# the files it is built from, so it is only built when one of them changes:
# the first job to need a new image builds and pushes it, and later runs pull it.
# Jobs that can't push (pull requests from forks) build it themselves.
#
# Environment:
#   GITHUB_REPOSITORY: owner/name of the repository; the image is
#                      ghcr.io/<owner>/<name>/test-env (lowercased)
#   IMAGE_TAG:         the image's tag, a hash of the files it is built from
#   PUSH:              "true" to push a newly built image
#   GH_TOKEN:          a token that can read (and to push, write) the
#                      repository's packages
#   GH_ACTOR:          the user name for the token
set -euo pipefail
: "${GITHUB_REPOSITORY:?}" "${IMAGE_TAG:?}" "${GH_TOKEN:?}" "${GH_ACTOR:?}"

registry=ghcr.io
IMAGE=$registry/${GITHUB_REPOSITORY,,}/test-env:$IMAGE_TAG
logged_in=false
if docker login "$registry" -u "$GH_ACTOR" --password-stdin <<< "$GH_TOKEN"; then
    logged_in=true
fi

if docker pull "$IMAGE"; then
    echo "Using the published image $IMAGE"
else
    echo "Building $IMAGE"
    docker build --target env -t "$IMAGE" -f docker-linux/Dockerfile .
    if [[ "${PUSH:-}" == true && "$logged_in" == true ]]; then
        # Another job may have pushed it in the meantime; that is fine, as both
        # images are built from the same files.
        docker push "$IMAGE" || echo "::warning::Could not push $IMAGE"
    fi
fi

docker tag "$IMAGE" sacn-test-env

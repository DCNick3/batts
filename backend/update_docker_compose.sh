#!/bin/bash

set -euo pipefail

if [[ -n "$(git status --porcelain)" ]]; then
  echo 'There are uncommitted changes. Please commit them before deploying.'
  exit 1
fi

bash -c 'cd .. && docker buildx build -t "ghcr.io/dcnick3/batts:backend-$(git rev-parse HEAD)" . -f Dockerfile-backend --push'

# yq command to rewrite image in k8s deployment
yq -i -Y '.services.backend.image = "ghcr.io/dcnick3/batts:backend-'$(git rev-parse HEAD)'"' docker-compose.yaml
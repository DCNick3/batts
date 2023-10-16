#!/bin/bash

set -euo pipefail

if [[ -n "$(git status --porcelain)" ]]; then
  echo 'There are uncommitted changes. Please commit them before deploying.'
  exit 1
fi

docker buildx build -t "ghcr.io/dcnick3/batts:frontend-$(git rev-parse HEAD)" . -f Dockerfile-frontend --push
docker buildx build -t "ghcr.io/dcnick3/batts:backend-$(git rev-parse HEAD)" . -f Dockerfile-backend --push

# yq command to rewrite image in k8s deployment
yq -i -Y '.spec.template.spec.containers[0].image = "ghcr.io/dcnick3/batts:backend-'$(git rev-parse HEAD)'"' deployment/deployment.yaml
yq -i -Y '.spec.template.spec.containers[1].image = "ghcr.io/dcnick3/batts:frontend-'$(git rev-parse HEAD)'"' deployment/deployment.yaml

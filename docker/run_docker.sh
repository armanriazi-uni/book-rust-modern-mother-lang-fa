#!/bin/bash

### This script runs a docker container based on the image
### created by the `docker/build_docker.sh` script.
### It runs the container and then executes a shell within that container,
### which allows

set -e

DOCKER_TAG="book-rust-modern-mother-lang-to-fa:Dockerfile"

# DOCKER_DIR is the directory containing this docker script and the Dockerfile
DOCKER_DIR=$(dirname $(readlink -f ${BASH_SOURCE}))
### book-rust-modern-mother-lang-to-fa_BASE_DIR is the base directory of the book-rust-modern-mother-lang-to-fa repository.
book-rust-modern-mother-lang-to-fa_BASE_DIR=$(readlink -f ${DOCKER_DIR}/.. )

### Always run this script with the `docker` directory as the working directory.
cd ${DOCKER_DIR}

# Run the actual docker image as a local container.
# Rather than copy files, we just mount the host machine's book-rust-modern-mother-lang-to-fa directory within the docker container.
# This mounting approach also allows the changes to persist after the docker container is exited.
# We also run the container using the current host user, allowing seamless sharing of builds and source across host and container.
#
# The DISPLAY and X11 arguments allow QEMU to create and run in a graphical window.
docker run \
    -v ${book-rust-modern-mother-lang-to-fa_BASE_DIR}:/book-rust-modern-mother-lang-to-fa \
    -w="/book-rust-modern-mother-lang-to-fa" \
    -u="$(id -u):$(id -g)" \
    --network host \
    -e DISPLAY=$DISPLAY \
    -v /tmp/.X11-unix:/tmp/.X11-unix \
    -ti ${DOCKER_TAG}

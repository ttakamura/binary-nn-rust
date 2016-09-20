FROM ubuntu:16.04

RUN apt-get update && \
    apt-get install -y \
        build-essential \
        cmake \
        curl \
        file \
        git \
        sudo \
        xutils-dev

RUN apt-get update && \
    apt-get install -y ca-certificates libc6-dev emacs python python-dev python-pip

RUN mkdir /rust
WORKDIR /rust

RUN apt-get install -y bash-completion
ENV SHELL bash

RUN cd /rust && \
    curl -O https://s3.amazonaws.com/mozilla-games/emscripten/releases/emsdk-portable.tar.gz && \
    tar xzvf emsdk-portable.tar.gz

RUN cd /rust/emsdk_portable && /bin/bash -c 'source ./emsdk_env.sh && emsdk update && emsdk install sdk-incoming-64bit && emsdk activate sdk-incoming-64bit'

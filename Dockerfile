FROM debian:stretch

RUN apt-get update && apt-get install -y curl build-essential

WORKDIR /usr/src/reconfix

COPY . ./
RUN ./ci/install.sh && ./ci/test.sh

FROM debian:stretch

RUN apt-get update && apt-get install -y curl build-essential libssl-dev pkg-config

WORKDIR /usr/src/reconfix

COPY . ./

#CMD exec /bin/bash -c "trap : TERM INT; sleep infinity & wait"
RUN ./ci/install.sh && ./ci/test.sh

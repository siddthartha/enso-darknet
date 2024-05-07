FROM rust:latest as rustBuilder

RUN USER=root cargo new --bin enso-darknet
WORKDIR /enso-darknet

RUN apt-get update \
    && apt-get install -y cmake libclang-dev gcc libc-bin libc-dev-bin libc6 python3 python3-dev  \
    && apt-get install -y tini \
    && rm -rf /var/lib/apt/lists/* \
    && rm -rf ./src

##
## Download libtorch binaries
##

RUN wget https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-2.0.1%2Bcpu.zip \
    && unzip ./libtorch-cxx11-abi-shared-with-deps-2.0.1+cpu.zip \
    && rm -f ./libtorch-cxx11-abi-shared-with-deps-2.0.1+cpu.zip

ENV LIBTORCH=/enso-darknet/libtorch
ARG LIBTORCH=/enso-darknet/libtorch

COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

ENV LD_LIBRARY_PATH=${LIBTORCH}/lib:${LD_LIBRARY_PATH}
ENV DEP_TCH_LIBTORCH_LIB=${LIBTORCH}/lib
ENV PATH=/enso-darknet:${PATH}

RUN cp /usr/bin/python3 /usr/bin/python

RUN \
    cargo build --release \
    && cp /enso-darknet/target/release/enso-darknet ./enso-darknet \
    && cp /enso-darknet/target/release/sd-cli ./sd-cli \
    && cp /enso-darknet/target/release/sd-worker ./sd-worker

# cleanup resources needed for rebuild only
#RUN cargo clean \
#    && rm -rf ${CARGO_HOME}/registry/* \
#    && rm -rf /enso-darknet/libtorch/include

COPY ./download-weights.sh ./download-weights.sh

COPY ./data ./data
COPY ./media ./media

# start API server
CMD ["./enso-darknet"]

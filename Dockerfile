FROM rust:latest as rustBuilder

RUN USER=root cargo new --bin enso-darknet
WORKDIR /enso-darknet

RUN apt-get update \
    && apt-get install -y cmake libclang-dev gcc libc-bin libc-dev-bin libc6 python3 python3-dev \
    && rm -rf /var/lib/apt/lists/* \
    && rm -rf ./src

##
## libtorch binaries
##

RUN wget https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-2.0.1%2Bcpu.zip \
    && unzip ./libtorch-cxx11-abi-shared-with-deps-2.0.1+cpu.zip \
    && rm -f ./libtorch-cxx11-abi-shared-with-deps-2.0.1+cpu.zip

ENV LIBTORCH=/enso-darknet/libtorch
ARG LIBTORCH=/enso-darknet/libtorch
#ENV LIBTORCH_LIB={$LIBTORCH}/lib
#ENV LIBTORCH_INCLUDE=${LIBTORCH}/include

COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

##
## PyTorch via miniconda
##

#ENV PATH="/root/miniconda3/bin:${PATH}"
#ARG PATH="/root/miniconda3/bin:${PATH}"

#RUN wget https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh \
#    && mkdir /root/.conda \
#    && bash Miniconda3-latest-Linux-x86_64.sh -b \
#    && rm -f Miniconda3-latest-Linux-x86_64.sh

#RUN conda --version \
#    && conda install pytorch torchvision torchaudio cpuonly -c pytorch

#ENV LIBTORCH_USE_PYTORCH=yes
#ENV LIBTORCH=/root/miniconda3/lib/python3.10/site-packages/torch/

ENV LD_LIBRARY_PATH=${LIBTORCH}/lib:${LD_LIBRARY_PATH}
ENV DEP_TCH_LIBTORCH_LIB=${LIBTORCH}/lib

#ENV C_INCLUDE_PATH=/enso-darknet/libtorch/include:${C_INCLUDE_PATH}
#ENV CPLUS_INCLUDE_PATH=/enso-darknet/libtorch/include:${CPLUS_INCLUDE_PATH}
#ENV LIBRARY_PATH=/enso-darknet/libtorch/lib:${LIBRARY_PATH}

ENV PATH=/enso-darknet:${PATH}
RUN cp /usr/bin/python3 /usr/bin/python

RUN \
    cargo build --release \
    && cp /enso-darknet/target/release/enso-darknet ./enso-darknet \
    && cp /enso-darknet/target/release/tensor-tool ./tensor-tool \
    && cargo clean
#    && rm -rf ${CARGO_HOME}/registry/* \
#    && rm -rf /enso-darknet/libtorch/include

COPY ./download-weights.sh ./download-weights.sh
COPY ./convert-weights.sh ./convert-weights.sh

COPY ./data ./data
COPY ./media ./media

#CMD ["./enso-darknet"]
#ENTRYPOINT ["./enso-darknet"]

##
## Alpine
##

#RUN rustup target add x86_64-unknown-linux-musl \
#    && cargo build --release
#    && RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu

#FROM alpine:latest

#ARG APP_DIR=/app
#ARG USER_NAME=app
#ARG GROUP_NAME=app

#WORKDIR ${APP_DIR}

#RUN addgroup -S ${GROUP_NAME} && adduser -S ${USER_NAME} ${GROUP_NAME}

#RUN apk --no-cache add ca-certificates wget gcc libstdc++6 \
#    && wget -q -O /etc/apk/keys/sgerrand.rsa.pub https://alpine-pkgs.sgerrand.com/sgerrand.rsa.pub \
#    && wget https://github.com/sgerrand/alpine-pkg-glibc/releases/download/2.31-r0/glibc-2.31-r0.apk \
#    && apk add --force-overwrite glibc-2.31-r0.apk \
#    && apk fix --force-overwrite alpine-baselayout-data

#COPY --from=builder /enso-darknet/target/release/enso-darknet ${APP_DIR}/enso-darknet

#RUN chown ${USER_NAME}:${GROUP_NAME} ${APP_DIR}/enso-darknet \
#  && chmod a+x ${APP_DIR}/enso-darknet

#USER ${USER_NAME}

#CMD ./enso-darknet


FROM dlecan/rust-crosscompiler-arm:stable

RUN rustup install nightly && \
    rustup default nightly && \
    rustup target add arm-unknown-linux-gnueabihf

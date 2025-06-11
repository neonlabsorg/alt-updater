FROM ubuntu:24.04 AS builder

RUN apt update && apt install -y build-essential curl
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH

RUN sh -c "$(curl -sSfL https://release.anza.xyz/v2.1.21/install)"
ENV PATH=/root/.local/share/solana/install/active_release/bin:$PATH

COPY . /opt/alt-updater/

RUN cd /opt/alt-updater/program/ && cargo build-sbf --sbf-out-dir=/opt/deploy/alt-updater/

FROM ubuntu:24.04 AS deploy

COPY --from=builder /root/.local/share/solana/install/active_release/bin/solana /opt/solana/bin/
COPY --from=builder /root/.local/share/solana/install/active_release/bin/solana-keygen /opt/solana/bin/
COPY --from=builder /opt/deploy /opt/deploy

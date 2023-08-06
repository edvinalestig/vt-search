FROM rust:1.67

WORKDIR /vt_search
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/vt_search/target \
    cargo install --path .

CMD [ "vt_search" ]
EXPOSE 5000
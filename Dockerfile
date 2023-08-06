FROM rust:1.67

WORKDIR /vt_search
COPY . .

RUN cargo install --path .

CMD [ "vt_search" ]
EXPOSE 5000
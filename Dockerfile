FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm AS build
COPY . /workspaces/nsenter1
WORKDIR /workspaces/nsenter1
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo build --release
FROM scratch AS final
COPY --from=build /workspaces/nsenter1/target/release/nsenter1 /bin/nsenter1
ENTRYPOINT ["/bin/nsenter1"]
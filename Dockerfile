FROM messense/rust-musl-cross:x86_64-musl as chef
ENV SQLX_OFFLINE=true
RUN cargo install cargo-chef
WORKDIR /app

FROM chef as planner
# Copy source code from previous stage
COPY . .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build & cache dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM node:20.8.0 as tailwind
WORKDIR /css
COPY package.json package.json
COPY package-lock.json package-lock.json
RUN npm install
COPY . .
RUN npm run web:prod

# Create a new stage with a minimal image
FROM scratch
COPY ./assets /assets
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/official /official
COPY --from=tailwind /css/assets /assets
ENTRYPOINT ["/official"]
EXPOSE 3000
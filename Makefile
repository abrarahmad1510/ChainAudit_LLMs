.PHONY: help setup build-proxy build-auditor test run-dev docker-build deploy
clean
help:
@echo "Available targets:"
@echo " setup - Install dependencies"
@echo " build-proxy - Build Envoy WASM filter"
@echo " build-auditor- Build auditor service"
@echo " test - Run all tests"
@echo " run-dev - Run development environment with docker-compos
e"
@echo " @echo " deploy @echo " clean docker-build - Build Docker images"
- Deploy to Kubernetes"
- Clean build artifacts"
setup:
@echo "Setting up..."
# Install Rust targets
rustup target add wasm32-unknown-unknown
# Install cargo tools if needed
cargo install cargo-watch
build-proxy:
build/docker/proxy/
cd src/proxy && cargo build --target wasm32-unknown-unknown --release
cp src/proxy/target/wasm32-unknown-unknown/release/verillm_proxy.wasm
build-auditor:
cd src/auditor && cargo build --release
test:
cargo test --workspace
run-dev:
docker-compose -f docker-compose.dev.yml up
docker-build:
docker build -f build/docker/proxy/Dockerfile -t verillm/proxy:latest
.
docker build -f build/docker/auditor/Dockerfile -t verillm/auditor:lat
est .
deploy:
kubectl apply -f build/kubernetes/
clean:
cargo clean
rm -rf build/docker/proxy/*.wasm

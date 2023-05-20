BUILD_VERSION := $(or $(BUILD_VERSION),git-`git rev-parse --short HEAD`)
IMAGE_NAME = ghcr.io/riipandi/fastrue
CONTAINER_NAME = fastrue

# Application envars
BIND_PORT = 9999

clean:
	@cargo clean

deps:
	@echo Installing dependencies
	@pnpm install
	@cargo update

run:
	@cargo run --release

run-docs:
	@cd docs && zola serve

dev:
	@pnpm concurrently --kill-others "cargo watch -qcx run" "pnpm dev"

build-web:
	@pnpm build

build: build-web
	@echo Running Build version $(BUILD_VERSION)
	@RUSTFLAGS="-C target-cpu=native" cargo build --release
	@ls -lh target/release

build-m1: build-web
	@echo Running Build version $(BUILD_VERSION)
	@RUSTFLAGS="-C target-cpu=apple-m1" cargo build --release
	@ls -lh target/release

migrate:
	@cargo run -- migrate

# --------------------------------------------------------------------------------------------------
# BUILD_VERSION=0.0.0-local make docker-build
# --------------------------------------------------------------------------------------------------

docker-build:
	@echo Running Docker Build version $(BUILD_VERSION)
	@DOCKER_BUILDKIT=1 docker build -f $(PWD)/Dockerfile \
		--build-arg BUILD_VERSION=$(BUILD_VERSION) \
		-t $(IMAGE_NAME):$(BUILD_VERSION) \
		-t $(IMAGE_NAME):latest .

docker-push:
	@echo Publishing container version $(BUILD_VERSION)
	@docker push $(IMAGE_NAME):$(BUILD_VERSION)

docker-run:
	@docker run --rm -it --name $(CONTAINER_NAME) --env-file $(PWD)/.env.docker \
		-p $(BIND_PORT):$(BIND_PORT) $(IMAGE_NAME):latest

docker-migrate:
	@docker run --rm -it --env-file $(PWD)/.env.docker -e ARGS="migrate --force" $(IMAGE_NAME):latest

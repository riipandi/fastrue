BUILD_VERSION := $(or $(BUILD_VERSION),git-`git rev-parse --short HEAD`)
IMAGE_NAME = ghcr.io/riipandi/trusty
REGISTRY_USERNAME = riipandi
CONTAINER_NAME = trusty

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

load-test:
	@k6 run --iterations=100 --vus=100 --summary-trend-stats="med,p(95),p(99.9)" $(PWD)/test_script.js

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
	echo $GITHUB_TOKEN | docker login ghcr.io --username $(REGISTRY_USERNAME) --password-stdin
	docker push $(IMAGE_NAME):$(BUILD_VERSION)
	docker push $(IMAGE_NAME):latest

docker-run:
	@docker run --rm -it --name $(CONTAINER_NAME) -e BIND_PORT=$(BIND_PORT) \
		-p $(BIND_PORT):$(BIND_PORT) $(IMAGE_NAME):latest

docker-shell:
	docker run --rm -it --entrypoint sh $(IMAGE_NAME):latest

docker-migrate:
	docker exec --env-file=env.docker $(CONTAINER_NAME) /usr/bin/trusty migrate

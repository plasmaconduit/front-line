IMAGE=plasmaconduit

.PHONY: build-image
build-image:
	@docker build -t "$(IMAGE)" -f ./dev.docker .

.PHONY: build-dev
build-dev: build-image
	cp ./docker/pre-commit-hook.sh .git/hooks/pre-commit
	IMAGE=$(IMAGE) ./docker/run.sh pre-commit install-hooks
	IMAGE=$(IMAGE) ./docker/run.sh cargo build

.PHONY: build-ci
build-ci: build-dev
	@docker save "$(IMAGE)" > ./build-cache/docker-image.tar

.PHONY: load-ci
load-ci:
	@docker load < ./build-cache/docker-image.tar

.PHONY: test
test: build-dev
	@IMAGE=$(IMAGE) ./docker/run.sh cargo test

.PHONY: pre-commit
pre-commit: build-dev
	@IMAGE=$(IMAGE) ./docker/run.sh pre-commit run --color=always

.PHONY: pre-commit-all
pre-commit-all: build-dev
	@IMAGE=$(IMAGE) ./docker/run.sh pre-commit run --all-files --color=always

.PHONY: shell
shell: build-dev
	@IMAGE=$(IMAGE) ./docker/run.sh bash

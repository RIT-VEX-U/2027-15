.PHONY: all build clean rebuild upload run stop

ifeq ($(OS),Windows_NT)
PYTHON ?= py -3
else
PYTHON ?= python3
endif
BUILD_MODULE := build_system

ifneq ($(filter run,$(MAKECMDGOALS)),)
RUN_SLOT := $(filter-out run,$(MAKECMDGOALS))
ifneq ($(filter-out 1 2 3 4 5 6 7 8,$(RUN_SLOT)),)
$(error Usage: make run [1-8])
endif
.PHONY: 1 2 3 4 5 6 7 8
1 2 3 4 5 6 7 8:
	@:
endif

ifneq ($(filter upload,$(MAKECMDGOALS)),)
UPLOAD_SLOT := $(filter-out upload,$(MAKECMDGOALS))
ifneq ($(filter-out 1 2 3 4 5 6 7 8,$(UPLOAD_SLOT)),)
$(error Usage: make upload [1-8])
endif
.PHONY: 1 2 3 4 5 6 7 8
1 2 3 4 5 6 7 8:
	@:
endif

all: build

build:
	@$(PYTHON) -m $(BUILD_MODULE) build $(ARGS)

clean:
	@$(PYTHON) -m $(BUILD_MODULE) clean $(ARGS)

rebuild:
	@$(PYTHON) -m $(BUILD_MODULE) rebuild $(ARGS)

upload:
	@$(PYTHON) -m $(BUILD_MODULE) upload $(if $(UPLOAD_SLOT),--slot $(UPLOAD_SLOT)) $(ARGS)

run:
	@$(PYTHON) -m $(BUILD_MODULE) run $(RUN_SLOT)

stop:
	@$(PYTHON) -m $(BUILD_MODULE) stop

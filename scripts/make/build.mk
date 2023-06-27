# Main building script

$(OUT_DIR):
	mkdir -p $@

ifeq ($(STD), y)
include scripts/make/std.mk
else
build_std:
endif

ifeq ($(APP_LANG), c)
  include ulib/c_libax/build.mk
else
  rust_package := $(shell cat $(APP)/Cargo.toml | sed -n 's/name = "\([a-z0-9A-Z_\-]*\)"/\1/p')
  ifeq ($(STD), y)
    rust_target_dir := $(CURDIR)/target/$(STD_TARGET)/$(MODE)
  else
    rust_target_dir := $(CURDIR)/target/$(TARGET)/$(MODE)
  endif
  rust_elf := $(rust_target_dir)/$(rust_package)
endif

_cargo_build: build_std
	@printf "    $(GREEN_C)Building$(END_C) $(MAJOR_DST) App: $(APP_NAME), Arch: $(ARCH), Platform: $(PLATFORM), Language: $(APP_LANG)\n"
ifeq ($(APP_LANG), rust)
  ifeq ($(STD), y)
# TODO: call cargo_build
	rm -rf $(rust_target_dir)
	$(call run_cmd,cargo rustc,\
		--manifest-path $(APP)/Cargo.toml \
		--target $(STD_TARGET) \
		--target-dir $(CURDIR)/target \
		--features "$(STD_ARGS)" \
		--release \
		-- -Clink-args="-T$(LD_SCRIPT) -no-pie")
  else
	$(call cargo_build,--manifest-path $(APP)/Cargo.toml)
  endif
	@cp $(rust_elf) $(OUT_ELF)
else ifeq ($(APP_LANG), c)
	$(call cargo_build,-p libax)
endif

$(OUT_BIN): _cargo_build $(OUT_ELF)
	$(call run_cmd,$(OBJCOPY),$(OUT_ELF) --strip-all -O binary $@)

.PHONY: _cargo_build build_std

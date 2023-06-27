# Building script for stdapps

host_target := $(shell rustc -vV | grep host | cut -d: -f2 | tr -d " ")

ifeq ($(ARCH), x86_64)
  STD_TARGET := x86_64-unknown-arceos
else
  $(error "ARCH" must be "x86_64" when "STD" is enabled)
endif

sysroot := $(CURDIR)/sysroot
rustlib_dir := $(sysroot)/lib/rustlib/$(STD_TARGET)
rustlib_host_dir := $(sysroot)/lib/rustlib/$(host_target)
rust_src := $(CURDIR)/third_party/rust

std_features := compiler-builtins-mem $(features-y)
rustflags := \
  --sysroot $(sysroot) \
  -C embed-bitcode=yes \
  -Z force-unstable-if-unmarked
build_std_args := \
  --target $(STD_TARGET) \
  --release \
  --manifest-path $(rust_src)/library/std/Cargo.toml

ifneq ($(V),)
  build_std_args += --verbose
endif

ifeq ($(STD_FEATURES),)
  std_features += libax/default
endif

export RUSTFLAGS=$(rustflags)

$(rustlib_dir):
	@printf "    $(GREEN_C)Creating$(END_C) sysroot\n"
	$(call run_cmd,mkdir,-p $(rustlib_dir) $(rustlib_host_dir))
	$(call run_cmd,ln,-sf $(rust_src)/target/$(STD_TARGET)/release/deps $(rustlib_dir)/lib)
	$(call run_cmd,ln,-sf $(rust_src)/target/release/deps $(rustlib_host_dir)/lib)
	$(call run_cmd,ln,-sf $(CURDIR)/$(STD_TARGET).json $(rustlib_dir)/target.json)

build_std: $(rustlib_dir)
	@printf "    $(GREEN_C)Building$(END_C) rust-std: Features: $(std_features)\n"
# stage 1: build the core and alloc libraries first which are required by ArceOS
	$(call run_cmd,cargo build,$(build_std_args) -p core -p alloc --features "compiler-builtins-mem")
# stage 2: build ArceOS and the std library with specified features
	$(call run_cmd,cargo build,$(build_std_args) -p std --features "$(std_features)")

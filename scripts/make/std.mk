# Building script for stdapps

LIB_ARCHIVE := $(OUT_DIR)/libarceos.a
LIB_ARCHIVE_RETAINED := $(LIB_ARCHIVE).retained
LIB_ARCHIVE_ORIG := $(LIB_ARCHIVE).orig

rust_package := $(shell cat $(APP)/Cargo.toml | sed -n 's/name = "\([a-z0-9A-Z_\-]*\)"/\1/p')
rust_elf := $(CURDIR)/target/x86_64-unknown-arceos/$(MODE)/$(rust_package)
rust_dep := $(subst -,_,$(rust_package))
rust_dep := $(CURDIR)/target/x86_64-unknown-arceos/$(MODE)/deps/$(rust_dep)*
rust_dep_std := $(HOME)/.xargo/lib/rustlib/x86_64-unknown-arceos/.hash

$(OUT_ELF): $(LIB_ARCHIVE)
	@printf "    $(GREEN_C)Building$(END_C) App: $(APP_NAME), Arch: $(ARCH), Platform: $(PLATFORM), Language: $(APP_LANG)\n"
	# Remove app's dep file, or elf won't be rebuilt! Same for rust std library!
	rm -f $(rust_dep) $(rust_dep_std)
	$(warning "+++++++++++++++++++ $(STD_ARGS)")
	RUST_TARGET_PATH=`pwd` xargo rustc --manifest-path $(APP)/Cargo.toml \
		--target x86_64-unknown-arceos \
		--release \
		--features "$(STD_ARGS)" \
		-- -L $(OUT_DIR) -l static=arceos \
		-Clink-args="-T$(LD_SCRIPT) -no-pie"
	@printf "    $(GREEN_C)Copy$(END_C) [$(rust_elf)] [$(OUT_ELF)]\n"
	cp $(rust_elf) $(OUT_ELF)

$(LIB_ARCHIVE): $(LIB_ARCHIVE_RETAINED)
	@mv $(LIB_ARCHIVE_RETAINED) $(LIB_ARCHIVE)

$(LIB_ARCHIVE_RETAINED): $(LIB_ARCHIVE_ORIG) $(CURDIR)/libarceos.redefine-syms
	@printf "    $(GREEN_C)Retain$(END_C) $(LIB_ARCHIVE_ORIG)\n"
	objcopy --redefine-syms=$(CURDIR)/libarceos.redefine-syms $@

$(LIB_ARCHIVE_ORIG): $(rust_target_dir)/libarceos.a
	cp $(CURDIR)/libarceos.redefine-syms.template $(CURDIR)/libarceos.redefine-syms
	objdump -t $(rust_target_dir)/libarceos.a | grep "sys_" | \
		awk -F '[ ]+' '{print $$5}' | \
		xargs -Isymbol echo 'libarceos_symbol symbol' \
		>> $(CURDIR)/libarceos.redefine-syms
	@printf "    $(GREEN_C)Copy$(END_C) LibArceOS as $@\n"
	@cp $(rust_target_dir)/libarceos.a $@
	objcopy --prefix-symbols=libarceos_ $(LIB_ARCHIVE_ORIG) $(LIB_ARCHIVE_RETAINED)
	rm $(LIB_ARCHIVE_ORIG)

ifneq ($(MAKECMDGOALS), test)
ifeq ($(STD), y)
APP_CONFIG := $(shell RUSTFLAGS="" cargo run \
	--manifest-path tools/parse_features/Cargo.toml\
	--release\
	$(APP)/Cargo.toml\
	$(STD_FEATURES))

$(eval include $(APP_CONFIG))
$(info "###### $(APP_CONFIG) : $(STD_FEATURES) ######")
$(info Config [STD] APP: $(APP), MULTITASK=$(MULTITASK), IRQ=$(IRQ), FS=$(FS), NET=$(NET), FS_TYPE=$(FS_TYPE))
endif
endif

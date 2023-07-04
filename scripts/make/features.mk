# Features resolving.
#
# Inputs:
#   - `FEATURES`: a list of features to be enabled split by space. Each feature
#     should starts with a prefix, which can be one of `ax/`, `lib/` or `app/`
#     and be used to set the features of ArceOS modules, the use library, and
#     the app itself, respectively. (e.g, `ax/feature1 lib/feature2 app/feature3`).
#
# Outputs:
#   - `AX_FEAT`: features to be enabled for ArceOS modules (crate `axfeat`).
#   - `LIB_FEAT`: features to be enabled for the user library (crate `libax`).
#   - `APP_FEAT`: features to be enabled for the app.

ax_prefix := ax/
lib_prefix := lib/
app_prefix := app/

ifeq ($(APP_TYPE),c)
  ax_prefix_after := axfeat/
  lib_prefix_after := libax/
  app_prefix_after :=
else ifeq ($(APP_TYPE),rust_std)
  ax_prefix_after := axfeat/
  lib_prefix_after := arceos_api/
  app_prefix_after :=
else
  ax_prefix_after := axstd/
  lib_prefix_after := axstd/
  app_prefix_after :=
endif

feature_has_fd = $(strip $(foreach feat,fs net pipe select epoll,$(filter lib/$(feat),$(FEATURES))))
feature_has_fp_simd = $(findstring fp_simd,$(FEATURES))

override FEATURES := $(shell echo $(FEATURES) | tr ',' ' ')

ifeq ($(APP_TYPE), c)
  ifneq ($(wildcard $(APP)/features.txt),)    # check features.txt exists
    override FEATURES += $(shell cat $(APP)/features.txt)
  endif
  ifneq ($(call feature_has_fd),)
    override FEATURES += lib/fd
  endif
  override FEATURES := $(strip $(FEATURES) lib/cbindings)
else ifeq ($(APP_TYPE), rust_std)
  std_feat := $(shell cat $(APP)/Cargo.toml | grep "axstd = " | sed -n 's/.*features = \[\(.*\)\].*/\1/p' | tr ',"' ' ')
  override FEATURES += $(addprefix $(lib_prefix),$(std_feat))
endif

ax_feat := platform-$(PLATFORM)
lib_feat :=

ifneq ($(filter $(LOG),off error warn info debug trace),)
  ax_feat += log-level-$(LOG)
else
  $(error "LOG" must be one of "off", "error", "warn", "info", "debug", "trace")
endif

ifeq ($(BUS),pci)
  ax_feat += bus-pci
endif

ifeq ($(shell test $(SMP) -gt 1; echo $$?),0)
  ax_feat += smp
  ifeq ($(APP_TYPE), c)
    lib_feat += smp
  endif
endif

AX_FEAT := $(patsubst $(ax_prefix)%,$(ax_prefix_after)%,$(filter $(ax_prefix)%,$(FEATURES)))
AX_FEAT += $(addprefix $(ax_prefix_after),$(ax_feat))
AX_FEAT := $(strip $(AX_FEAT))

LIB_FEAT := $(patsubst $(lib_prefix)%,$(lib_prefix_after)%,$(filter $(lib_prefix)%,$(FEATURES)))
LIB_FEAT += $(addprefix $(lib_prefix_after),$(lib_feat))
LIB_FEAT := $(strip $(LIB_FEAT))

APP_FEAT := $(patsubst $(app_prefix)%,$(app_prefix_after)%,$(filter $(app_prefix)%,$(FEATURES)))
APP_FEAT := $(strip $(APP_FEAT))

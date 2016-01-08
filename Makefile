ifeq ($(RELEASE), 1)
O ?= target/aarch64/release
DEP_O ?= target/aarch64/release/deps
else
O ?= target/aarch64/debug
DEP_O ?= target/aarch64/debug/deps
endif

ARCH ?= aarch64
export ARCH

TARGET_FILE=src/arch/$(ARCH)/$(ARCH).json

include src/arch/$(ARCH)/Makefile.include

# Don't use default rules
.SUFFIXES:

RUSTC ?= rustc
CC = $(CROSS_COMPILE)gcc
AR = $(CROSS_COMPILE)ar
AS = $(CROSS_COMPILE)as
OBJCOPY = $(CROSS_COMPILE)objcopy
OBJDUMP = $(CROSS_COMPILE)objdump

COMMON_FLAGS += -Wall -nostdlib

ifneq ($(SELFTEST),)
CARGOFLAGS += --features=selftest
endif

ifneq ($(RELEASE),)
CFLAGS += -O3
CARGOFLAGS += --release
else
CFLAGS += -O0
COMMON_FLAGS += -g
endif

AFLAGS += -D__ASSEMBLY__ $(COMMON_FLAGS) -Irt/include
CFLAGS += $(COMMON_FLAGS) -Irt/include
LDFLAGS += $(COMMON_FLAGS)

RSFLAGS += --cfg arch_$(ARCH)
LDFLAGS +=
CFLAGS += -Irt/$(ARCH)/include/

AFLAGS += -D__ASSEMBLY__ $(COMMON_FLAGS) -Irt/include

QEMUFLAGS += -nographic -machine vexpress-a15 -cpu cortex-a57 -m 2048 -semihosting

.PHONY: all

all: $(O)/titanos.hex $(O)/titanos.bin

RT_SRCS = rt/$(ARCH)/head.S
RT_OBJS = $(RT_SRCS:.c=.o)
RT_OBJS := $(RT_OBJS:.S=.o)

RT_OBJS := $(addprefix $(O)/,$(RT_OBJS))

RT_OBJS_DEPS := $(RT_OBJS:.o=.o.d)

CLEAN += $(RT_OBJS_DEPS) $(RT_OBJS)

-include $(RT_OBJS_DEPS)

$(O)/%.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@
	$(CC) $(CFLAGS) -MM -MT$@ -MF$@.d -c $< -o $@

$(O)/%.o: %.S
	@mkdir -p $(dir $@)
	$(CC) $(AFLAGS) -c $< -o $@
	$(CC) $(AFLAGS) -MM -MT$@ -MF$@.d -c $<

CLEAN += $(DEP_O)/libcompiler-rt.a
$(DEP_O)/libcompiler-rt.a: $(RT_OBJS) $(TARGET_FILE)
	@mkdir -p $(dir $@)
	$(AR) rcs $@ $(RT_OBJS)

$(O)/titanos: $(DEP_O)/libcompiler-rt.a FORCE
	PATH=wrappers/:$$PATH cargo build $(CARGOFLAGS) --target $(TARGET_FILE) --verbose

.PHONY: doc
doc:
	@echo "Sorry, this does not work ATM: https://github.com/rust-lang/cargo/issues/1427"
	PATH=wrappers/:$$PATH cargo doc $(CARGOFLAGS) --target $(TARGET_FILE)

$(O)/titanos.hex: $(O)/titanos
	$(OBJCOPY) -O ihex $(O)/titanos $(O)/titanos.hex

$(O)/titanos.bin: $(O)/titanos
	$(OBJCOPY) -O binary $(O)/titanos $(O)/titanos.bin

.PHONY: clean
clean:
	cargo clean
	rm -f $(CLEAN)

.PHONY: objdump
objdump:
	$(OBJDUMP) -D $(O)/titanos

.PHONY: run
run: qemu

.PHONY: debug
debug: qemu-gdb

.PHONY: qemu
qemu: qemu-$(ARCH)

.PHONY: qemu-gdb
qemu-gdb: qemu-$(ARCH)-gdb

.PHONY: qemu-aarch64
qemu-aarch64: $(O)/titanos.bin
	qemu-system-aarch64 $(QEMUFLAGS) -kernel $(O)/titanos.bin

.PHONY: qemu-aarch64-gdb
qemu-aarch64-gdb: $(O)/titanos.bin
	qemu-system-aarch64 -S -s $(QEMUFLAGS) -kernel $(O)/titanos.bin

.PHONY: gdb
gdb:
	$(CROSS_COMPILE)gdb -s $(O)/titanos -ex "target remote localhost:1234"

.PHONY: FORCE


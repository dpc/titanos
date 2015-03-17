ifeq ($(RELEASE), 1)
O ?= target/target/release
DEP_O ?= target/target/release/deps
else
O ?= target/target/debug
DEP_O ?= target/target/debug/deps
endif

ARCH ?= aarch64
export ARCH

TARGET_FILE=src/arch/$(ARCH)/target.json

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

ifeq ($(RELEASE), 1)
RSFLAGS += -O -g
CFLAGS += -O2
CARGOFLAGS += --release
else
RSFLAGS += -g
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

.PHONY: all

all: $(O)/titanos.hex $(O)/titanos.bin

RT_SRCS=rt/$(ARCH)/head.S

RT_OBJS = $(RT_SRCS:.c=.o)
RT_OBJS := $(RT_OBJS:.S=.o)

RT_OBJS := $(addprefix $(O)/,$(RT_OBJS))

RT_OBJS_DEPS := $(RT_OBJS:.o=.o.d)

CLEAN += $(RT_OBJS_DEPS) $(RT_OBJS)

-include $(RT_OBJS_DEPS)

$(O)/%.o: %.c
	mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@
	$(CC) $(CFLAGS) -MM -MT$@ -MF$@.d -c $< -o $@

$(O)/%.o: %.S
	mkdir -p $(dir $@)
	$(CC) $(AFLAGS) -c $< -o $@
	$(CC) $(AFLAGS) -MM -MT$@ -MF$@.d -c $<

CLEAN += $(DEP_O)/libcompiler-rt.a
$(DEP_O)/libcompiler-rt.a: $(RT_OBJS) $(TARGET_FILE)
	mkdir -p $(dir $@)
	$(AR) rcs $@ $(RT_OBJS)

$(O)/titanos: $(DEP_O)/libcompiler-rt.a FORCE
	PATH=wrappers/:$$PATH cargo build $(CARGOFLAGS) --target $(TARGET_FILE) --verbose

.PHONY: doc
doc: FORCE
	echo "Sorry, this does not work ATM: https://github.com/rust-lang/cargo/issues/1427"
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
qemu-aarch64:
	qemu-system-aarch64 -nographic -machine vexpress-a15 -cpu cortex-a57 -m 2048 -kernel $(O)/titanos.bin

.PHONY: qemu-aarch64-gdb
qemu-aarch64-gdb:
	qemu-system-aarch64 -S -s -nographic -machine vexpress-a15 -cpu cortex-a57 -m 2048 -kernel $(O)/titanos.bin

.PHONY: gdb
gdb:
	$(CROSS_COMPILE)gdb -s $(O)/titanos -ex "target remote localhost:1234"

.PHONY: FORCE


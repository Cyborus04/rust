# This test checks that C linking with Rust does not encounter any errors, with dynamic libraries.
# See https://github.com/rust-lang/rust/issues/10434

# ignore-cross-compile
include ../tools.mk

all: $(TMPDIR)/$(call BIN,bar)
	$(call RUN,bar)
	$(call REMOVE_DYLIBS,foo)
	$(call FAIL,bar)

ifdef IS_MSVC
$(TMPDIR)/$(call BIN,bar): $(call DYLIB,foo)
	$(CC) bar.c $(TMPDIR)/foo.dll.lib $(call OUT_EXE,bar)
else
$(TMPDIR)/$(call BIN,bar): $(call DYLIB,foo)
	$(CC) bar.c -lfoo -o $(call RUN_BINFILE,bar) -L $(TMPDIR)
endif

$(call DYLIB,foo): foo.rs
	$(RUSTC) foo.rs

RUSTC := rustc
DGLR_SRC := dglr.rs
DGLR := $(foreach file,$(shell $(RUSTC) --crate-file-name $(DGLR_SRC)),$(file))

$(DGLR): dglr.rs
	rustc dglr.rs

test:	dglr
	RUST_LOG=dglr=4 ./dglr

dglr:	dglr.rs
	rustc --test dglr.rs

clean:
	rm dglr $(DGLR)

print-targets:
	@echo $(DGLR)

.PHONY: clean print-targets

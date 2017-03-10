RUST_BOOK_ID := ffi-rust rust-untuk-web

.PHONY: all $(RUST_BOOK_ID)

all: $(RUST_BOOK_ID)

clean: $(RUST_BOOK_ID)

$(RUST_BOOK_ID):
	$(MAKE) -C $@ $(MAKECMDGOALS)

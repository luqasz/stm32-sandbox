BIN_NAME=sandbox


flash:
	cargo objcopy --bin $(BIN_NAME) --release -- -O binary $(BIN_NAME).bin
	st-flash --reset write $(BIN_NAME).bin 0x08000000

clean:
	cargo clean

.PHONY: clean flash

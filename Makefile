build:
	cargo build --release
	arm-none-eabi-objcopy -O binary \
    	target/thumbv7m-none-eabi/release/embedded-watering \
    	target/thumbv7m-none-eabi/release/embedded-watering.bin


deploy:
	st-flash write target/thumbv7m-none-eabi/release/embedded-watering.bin 0x8000000

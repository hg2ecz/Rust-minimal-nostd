all:
	@ # Linux
	@ cargo rustc --release -- -C link-arg=-nostartfiles --emit asm
	@ # Windows
	@ #cargo rustc --release -- -C link-args="/ENTRY:_start /SUBSYSTEM:console" --emit asm
	@ # macOS
	@ #cargo rustc --release -- -C link-args="-e __start -static -nostartfiles" --emit asm

	@strip target/release/no_std_test

clean:
	cargo clean

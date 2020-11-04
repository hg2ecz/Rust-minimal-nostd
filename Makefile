all:
	@ # Linux
	@ cargo rustc --release -- -C link-arg=-nostartfiles
	@ # Windows
	@ #cargo rustc --release -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
	@ # macOS
	@ #cargo rustc --release -- -C link-args="-e __start -static -nostartfiles"




# create make task that allow provide arg

example:
	@echo "Hello $(arg)"

r:
	cargo run -q --example $(n)

## use make r n=c02

rw:
	cargo watch -q -c -x "run --example $(n)"

## use by make rw n=c02
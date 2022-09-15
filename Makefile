# Builds the `opt` profile for the build machine's target.
# std is built from source, to get the smallest size possible.
opt:
	cargo build -Z build-std=std,panic_abort --target $$(rustc -vV | grep -oP 'host: \K(.*)') --profile opt

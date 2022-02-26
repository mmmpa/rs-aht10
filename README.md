# aht10

This is to use aht10 with Rust.

- [AHT10 Technical Manual](https://server4.eca.ir/eshop/AHT10/Aosong_AHT10_en_draft_0c.pdf)

# example for raspi zero

## env

```sh
export I2C_DEVICE_PATH="/dev/i2c-1"
export I2C_DEVICE_ADDRESS="0x38"
```

## .cargo/config

```toml
[target.arm-unknown-linux-gnueabi]
linker = "arm-linux-gnueabi-gcc"
```

## build

```shell
cargo build --example printer --target arm-unknown-linux-gnueabi --release --features std
```

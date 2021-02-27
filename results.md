## My results

Running `cargo +1.46.0 run -q --release -- ./wasm/app-1.46.0.wasm ./wasm/app-nightly.wasm && cargo +nightly -q run --release -- ./wasm/app-1.46.0.wasm ./wasm/app-nightly.wasm` on various systems

### Windows (i7-7700)

- rustc 1.46.0 (04488afe3 2020-08-24)

```
Loading module "./wasm/app-1.46.0.wasm"
Module "./wasm/app-1.46.0.wasm" loaded in 122.5561ms

Loading module "./wasm/app-nightly.wasm"
Module "./wasm/app-nightly.wasm" loaded in 146.743ms
```

- rustc 1.52.0-nightly (98f8cce6d 2021-02-25)

```
Loading module "./wasm/app-1.46.0.wasm"
Module "./wasm/app-1.46.0.wasm" loaded in 99.987ms

Loading module "./wasm/app-nightly.wasm"
Module "./wasm/app-nightly.wasm" loaded in 137.3825ms
```

### Linux (same machine)

- rustc 1.46.0 (04488afe3 2020-08-24)

```
Loading module "./wasm/app-1.46.0.wasm"
Module "./wasm/app-1.46.0.wasm" loaded in 109.7042ms

Loading module "./wasm/app-nightly.wasm"
Module "./wasm/app-nightly.wasm" loaded in 118.7751ms
```

- rustc 1.51.0-nightly (a2f8f6281 2021-01-27)

```
Loading module "./wasm/app-1.46.0.wasm"
Module "./wasm/app-1.46.0.wasm" loaded in 120.6095ms

Loading module "./wasm/app-nightly.wasm"
Module "./wasm/app-nightly.wasm" loaded in 142.5472ms
```
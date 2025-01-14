Build

```sh
$ cargo build
```

Run by pywasm

```sh
$ pip install pywasm

$ python
import pywasm
runtime = pywasm.core.Runtime()
m = runtime.instance_from_file('target/wasm32-unknown-unknown/debug/ckb-vm-wasm.wasm')
print(runtime.invocate(m, '_start', []))
```

Run by wasmer

```sh
$ wget https://github.com/wasmerio/wasmer/releases/download/v4.4.0/wasmer-linux-amd64.tar.gz
$ tar -xvf wasmer-linux-amd64.tar.gz

$ bin/wasmer target/wasm32-unknown-unknown/debug/ckb-vm-wasm.wasm
```

# wasm-pre-init

Quick and dirty testing [wizer](https://github.com/bytecodealliance/wizer) 
against wasm binaries built with rust and ran using [wazero](https://github.com/tetratelabs/wazero/) runtime.

Spoiler: it works perfectly :)

## Explanation
`wizer` is a pre-initializer for Wasm - think `wasm-opt`. 

This would allow you to do something resource intensive at build time, rather
than runtime. For example, you could pre-initialize multiple regexes that are
normally quite expensive.

## This Test

This test verifies that `wizer` works with wasm artifacts produced by `wazero`.

There are two test cases:

**`wasm-regex` and `wasm-sleep`**

### wasm-regex

This test case has two wasm files - `wasm-regex-regular.wasm` and `wasm-regex-init.wizer.wasm`.

* `*regular*` contains regex init code directly in the `_start()` func.
* `*init.wizer*` contains regex init code in the `init()` func.

`make run` will show how long `wazero run ...` took to run both of the wasm files.

`time` isn't precise enough to give a definitive answer as to what's faster, so
need another more test that gives us a clearer picture.

### wasm-sleep

In this case, `*regular.wasm` will sleep for 5s before printing "John Doe". 
`*init.wizer.wasm` will sleep for 5s in the `init()` func and print "John Doe"
in the `_start()` func.

Same as with `wasm-regex`:

* `*regular*` contains sleep code directly in the `_start()` func.
* `*init.wizer*` contains sleep code in the `init()` func.

You should see that `*init.wizer.wasm` will take <1s to run while `*regular.wasm` will take >5s.

## Result
```bash
~/Code/streamdal/wasm-pre-init master*                                                                                                                                                 ◒
❯ make run
Testing wasm-regex-regular...
time wazero run -env NUM=123 build/wasm_regex_regular.wasm
Matched!
        3.52 real         3.49 user         0.03 sys
Testing wasm-regex-init...
time wazero run -env NUM=123 build/wasm_regex_init.wizer.wasm
Matched!
        3.48 real         3.46 user         0.02 sys
Testing wasm-sleep-regular...
time wazero run build/wasm_sleep_regular.wasm
Name: John Doe
        5.07 real         0.06 user         0.01 sys
Testing wasm-sleep-init...
time wazero run build/wasm_sleep_init.wizer.wasm
Name: John Doe
        0.07 real         0.06 user         0.01 sys
```


## Setup
1. Install `wazero` CLI binary; install `wizer` via cargo.
1. Run `make build` <- this will create .wasm artifacts in `./build/*`
1. Run `make run` <- this will perform `time wazero run ...` on each one of the wasm files in the `./build` dir.

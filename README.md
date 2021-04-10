# Rust FIX toolbox

This library help to build [FIX](https://www.fixtrading.org/) aware applications using Rust.

## Goals / non goals

Goals:

- Provide simple binding for FIX protocol messages and fields
- Provide user friendly object
- Data **must be** generated from XML dictionaries

Non goals:

- Produce latency sensitive code.

  Performances is important. But we are not targeting nano seconds
  serialization / deserialisation. Just switch to FPGA for this kind
  of latencies.

- Provide a full FIX engine

## Code samples

Here some integration tests to show code usages:

- [serialization](./quickfix-messages/tests/test_serialize.rs)

## State of the project

For now the project is in its early beginning.
So, this project is still not fully production ready.

Any help and PR are welcome :smiley: !

DONE:

- FIX dictionary XML parser + data model
- _Field_ gerator
- Toolchain to generate library from generated code

TODO:

- _Message_ generator
- Find target library name :thinking: + publish to crates.io
- Add example usages

## Why I do not provide full FIX engine

There is already so much generic server implementation for Rust.
I just do not want to force user to use specific server / client framework.

This library is focus on message correctness and have user friendly object to decode / encode FIX messages.

## Few extra links

- <https://en.wikipedia.org/wiki/Financial_Information_eXchange>
- <http://www.quickfixengine.org/>
- <https://www.fixtrading.org/>

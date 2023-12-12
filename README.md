# Rust FIX toolbox

**ARCHIVE PROJECT: PLEASE CHECK OUT THIS [NEW](https://github.com/arthurlm/quickfix-rs/) PROJECT FOR REPLACEMENT / IMPROVMENT.**

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

  See [benchmark](https://github.com/arthurlm/openfix-rs/wiki/Benchmark-results) for more information.

- Provide a full FIX engine

## Code samples

Here some integration tests to show code usages:

- [serialization with no trailers](./openfix-messages/tests/serialize_empty_trailers.rs)
- [serialization with signature](./openfix-messages/tests/serialize_with_trailers.rs)
- [deserialization](./openfix-messages/tests/deserialize.rs)

## State of the project

For now the project is in its early beginning.
So, this project is still not fully production ready.

Any help and PR are welcome :smiley: !

DONE:

- FIX dictionary XML parser + data model
- _Field_ gerator
- _Message_ generator
- Toolchain to generate library from generated code
- Message encoder
- Message decoder
- Helper for message checksum and length
- Find target library name :thinking:

TODO:

- Message builder from MsgType field
- Publish to crates.io
- Add example usages

## Why I do not provide full FIX engine

There is already so much generic server implementation for Rust.
I just do not want to force user to use specific server / client framework.

This library is focus on message correctness and have user friendly object to decode / encode FIX messages.

## Few extra links

- <https://en.wikipedia.org/wiki/Financial_Information_eXchange>
- <http://www.quickfixengine.org/>
- <https://www.fixtrading.org/>

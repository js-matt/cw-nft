# Commands

Build the project using the following command:
```
cargo wasm
```

Run unit test using the following command:
```
cargo unit-test
```

# Structure of the Project
```
.
├── .cargo
│   └── config.toml
├── src
│   └── testing
│   │   └── tests.rs
│   │   └── mod.rs
│   └── contract.rs
│   └── lib.rs
│   └── msg.rs
│   └── state.rs
├── README.MD
├── deploy_testnet.py
└── Cargo.toml

```
## .cargo/config.toml
cargo alias: `wasm` & `unit-test`

## src
### /contract.rs

- Defines a specialized type for a CW721 contract that includes additional metadata extensions.

### /lib.rs
- This file defines the main structure of the contract, exposing the necessary entry points for interacting with it.
### msg.rs
- Define a type alias for ExecuteMsg & QueryMsg
### /state.rs
- Metadata Struct: Contains detailed information about a music track, extending the standard CW721 metadata.
- Extension Type Alias: Represents an optional Metadata structure, allowing tokens to include additional metadata if needed.

### /testing/tests.rs
- Unit test for the contract

# Deploy
## Prerequests
- Python3
- Osmosis testnet configuration
- Gas fee

## Command
```
python3 deploy_testnet.py
```

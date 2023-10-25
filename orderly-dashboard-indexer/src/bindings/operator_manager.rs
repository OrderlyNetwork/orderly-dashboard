pub use operator_manager::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod operator_manager {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("checkCefiDown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkCefiDown"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("eventUpload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("eventUpload"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct EventTypes.EventUpload",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("futuresTradeUpload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("futuresTradeUpload"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct PerpTypes.FuturesTradeUploadData",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operatorPing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("operatorPing"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("perpPriceUpload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("perpPriceUpload"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct MarketTypes.UploadPerpPrice",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setCefiEventUploadAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setCefiEventUploadAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_cefiEventUploadAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setCefiMarketUploadAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setCefiMarketUploadAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_cefiMarketUploadAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setCefiPerpTradeUploadAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setCefiPerpTradeUploadAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_cefiPerpTradeUploadAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setCefiSpotTradeUploadAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setCefiSpotTradeUploadAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_cefiSpotTradeUploadAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setLedger"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setLedger"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ledger"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setMarketManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setMarketManager"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_marketManagerAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setOperator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_operatorAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sumUnitaryFundingsUpload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "sumUnitaryFundingsUpload",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct MarketTypes.UploadSumUnitaryFundings",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ChangeCefiUpload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChangeCefiUpload"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("types"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ChangeLedger"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChangeLedger"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ChangeMarketManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ChangeMarketManager",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ChangeOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChangeOperator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("types"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EventUpload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EventUpload"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("batchId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("blocktime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EventUpload"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("batchId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FuturesTradeUpload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FuturesTradeUpload"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("batchId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("blocktime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FuturesTradeUpload"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("batchId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AccountIdInvalid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AccountIdInvalid"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressZero"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AddressZero"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BalanceNotEnough"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("BalanceNotEnough"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BatchIdNotMatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("BatchIdNotMatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("batchId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "futuresUploadBatchId",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BrokerNotAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("BrokerNotAllowed"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CountNotMatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("CountNotMatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("length"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("count"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsurancePositionQtyInvalid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsurancePositionQtyInvalid",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "adlPositionQtyTransfer",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("userPositionQty"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsuranceTransferAmountInvalid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsuranceTransferAmountInvalid",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "insuranceTransferAmount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("settledAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsuranceTransferToSelf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsuranceTransferToSelf",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidBizType"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidBizType"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bizType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyCrossChainManagerCanCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OnlyCrossChainManagerCanCall",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyOperatorCanCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OnlyOperatorCanCall",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SignatureNotMatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SignatureNotMatch"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SymbolNotAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SymbolNotAllowed"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenNotAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TokenNotAllowed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TotalSettleAmountNotMatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TotalSettleAmountNotMatch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UserPerpPositionQtyZero"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UserPerpPositionQtyZero",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("accountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbolHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static OPERATOR_MANAGER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct operator_manager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for operator_manager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for operator_manager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for operator_manager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for operator_manager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(operator_manager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> operator_manager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    OPERATOR_MANAGER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `checkCefiDown` (0x1118582d) function
        pub fn check_cefi_down(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([17, 24, 88, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eventUpload` (0xe8bb8f8f) function
        pub fn event_upload(
            &self,
            data: EventUpload,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([232, 187, 143, 143], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `futuresTradeUpload` (0x2604b9f1) function
        pub fn futures_trade_upload(
            &self,
            data: FuturesTradeUploadData,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 4, 185, 241], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x8129fc1c) function
        pub fn initialize(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorPing` (0x6ecb9886) function
        pub fn operator_ping(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 203, 152, 134], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `perpPriceUpload` (0x79b0d946) function
        pub fn perp_price_upload(
            &self,
            data: UploadPerpPrice,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 176, 217, 70], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setCefiEventUploadAddress` (0x6a8797ea) function
        pub fn set_cefi_event_upload_address(
            &self,
            cefi_event_upload_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([106, 135, 151, 234], cefi_event_upload_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setCefiMarketUploadAddress` (0x72ab8e21) function
        pub fn set_cefi_market_upload_address(
            &self,
            cefi_market_upload_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 171, 142, 33], cefi_market_upload_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setCefiPerpTradeUploadAddress` (0xf9391a18) function
        pub fn set_cefi_perp_trade_upload_address(
            &self,
            cefi_perp_trade_upload_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 57, 26, 24], cefi_perp_trade_upload_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setCefiSpotTradeUploadAddress` (0xb7c888f2) function
        pub fn set_cefi_spot_trade_upload_address(
            &self,
            cefi_spot_trade_upload_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 200, 136, 242], cefi_spot_trade_upload_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLedger` (0x3246887d) function
        pub fn set_ledger(
            &self,
            ledger: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 70, 136, 125], ledger)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMarketManager` (0xd82aff11) function
        pub fn set_market_manager(
            &self,
            market_manager_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([216, 42, 255, 17], market_manager_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOperator` (0xb3ab15fb) function
        pub fn set_operator(
            &self,
            operator_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([179, 171, 21, 251], operator_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sumUnitaryFundingsUpload` (0x757a69fd) function
        pub fn sum_unitary_fundings_upload(
            &self,
            data: UploadSumUnitaryFundings,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([117, 122, 105, 253], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ChangeCefiUpload` event
        pub fn change_cefi_upload_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangeCefiUploadFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangeLedger` event
        pub fn change_ledger_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangeLedgerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangeMarketManager` event
        pub fn change_market_manager_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangeMarketManagerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangeOperator` event
        pub fn change_operator_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangeOperatorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EventUpload` event
        pub fn event_upload_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EventUpload1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EventUpload` event
        pub fn event_upload_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EventUpload2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FuturesTradeUpload` event
        pub fn futures_trade_upload_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FuturesTradeUpload1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FuturesTradeUpload` event
        pub fn futures_trade_upload_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FuturesTradeUpload2Filter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            operator_managerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for operator_manager<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AccountIdInvalid` with signature `AccountIdInvalid()` and selector `0xc7ee9ce6`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AccountIdInvalid", abi = "AccountIdInvalid()")]
    pub struct AccountIdInvalid;
    ///Custom Error type `AddressZero` with signature `AddressZero()` and selector `0x9fabe1c1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AddressZero", abi = "AddressZero()")]
    pub struct AddressZero;
    ///Custom Error type `BalanceNotEnough` with signature `BalanceNotEnough(uint128,int128)` and selector `0x127c97f3`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "BalanceNotEnough", abi = "BalanceNotEnough(uint128,int128)")]
    pub struct BalanceNotEnough {
        pub balance: u128,
        pub amount: i128,
    }
    ///Custom Error type `BatchIdNotMatch` with signature `BatchIdNotMatch(uint64,uint64)` and selector `0x7e66023f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "BatchIdNotMatch", abi = "BatchIdNotMatch(uint64,uint64)")]
    pub struct BatchIdNotMatch {
        pub batch_id: u64,
        pub futures_upload_batch_id: u64,
    }
    ///Custom Error type `BrokerNotAllowed` with signature `BrokerNotAllowed()` and selector `0x59d9b863`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "BrokerNotAllowed", abi = "BrokerNotAllowed()")]
    pub struct BrokerNotAllowed;
    ///Custom Error type `CountNotMatch` with signature `CountNotMatch(uint256,uint256)` and selector `0x75ec4356`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "CountNotMatch", abi = "CountNotMatch(uint256,uint256)")]
    pub struct CountNotMatch {
        pub length: ::ethers::core::types::U256,
        pub count: ::ethers::core::types::U256,
    }
    ///Custom Error type `InsurancePositionQtyInvalid` with signature `InsurancePositionQtyInvalid(int128,int128)` and selector `0xc7536dca`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "InsurancePositionQtyInvalid",
        abi = "InsurancePositionQtyInvalid(int128,int128)"
    )]
    pub struct InsurancePositionQtyInvalid {
        pub adl_position_qty_transfer: i128,
        pub user_position_qty: i128,
    }
    ///Custom Error type `InsuranceTransferAmountInvalid` with signature `InsuranceTransferAmountInvalid(uint128,uint128,int128)` and selector `0x63cc1f6e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "InsuranceTransferAmountInvalid",
        abi = "InsuranceTransferAmountInvalid(uint128,uint128,int128)"
    )]
    pub struct InsuranceTransferAmountInvalid {
        pub balance: u128,
        pub insurance_transfer_amount: u128,
        pub settled_amount: i128,
    }
    ///Custom Error type `InsuranceTransferToSelf` with signature `InsuranceTransferToSelf()` and selector `0xd3cc8a83`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InsuranceTransferToSelf", abi = "InsuranceTransferToSelf()")]
    pub struct InsuranceTransferToSelf;
    ///Custom Error type `InvalidBizType` with signature `InvalidBizType(uint8)` and selector `0xaee5e5c4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidBizType", abi = "InvalidBizType(uint8)")]
    pub struct InvalidBizType {
        pub biz_type: u8,
    }
    ///Custom Error type `OnlyCrossChainManagerCanCall` with signature `OnlyCrossChainManagerCanCall()` and selector `0x833d33e7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "OnlyCrossChainManagerCanCall",
        abi = "OnlyCrossChainManagerCanCall()"
    )]
    pub struct OnlyCrossChainManagerCanCall;
    ///Custom Error type `OnlyOperatorCanCall` with signature `OnlyOperatorCanCall()` and selector `0x515b6e4b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OnlyOperatorCanCall", abi = "OnlyOperatorCanCall()")]
    pub struct OnlyOperatorCanCall;
    ///Custom Error type `SignatureNotMatch` with signature `SignatureNotMatch()` and selector `0x0510cc45`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SignatureNotMatch", abi = "SignatureNotMatch()")]
    pub struct SignatureNotMatch;
    ///Custom Error type `SymbolNotAllowed` with signature `SymbolNotAllowed()` and selector `0xb602958d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SymbolNotAllowed", abi = "SymbolNotAllowed()")]
    pub struct SymbolNotAllowed;
    ///Custom Error type `TokenNotAllowed` with signature `TokenNotAllowed(bytes32,uint256)` and selector `0x7c334f8b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "TokenNotAllowed", abi = "TokenNotAllowed(bytes32,uint256)")]
    pub struct TokenNotAllowed {
        pub token_hash: [u8; 32],
        pub chain_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `TotalSettleAmountNotMatch` with signature `TotalSettleAmountNotMatch(int128)` and selector `0x4ddd71b0`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "TotalSettleAmountNotMatch",
        abi = "TotalSettleAmountNotMatch(int128)"
    )]
    pub struct TotalSettleAmountNotMatch {
        pub amount: i128,
    }
    ///Custom Error type `UserPerpPositionQtyZero` with signature `UserPerpPositionQtyZero(bytes32,bytes32)` and selector `0x38cc3765`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "UserPerpPositionQtyZero",
        abi = "UserPerpPositionQtyZero(bytes32,bytes32)"
    )]
    pub struct UserPerpPositionQtyZero {
        pub account_id: [u8; 32],
        pub symbol_hash: [u8; 32],
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum operator_managerErrors {
        AccountIdInvalid(AccountIdInvalid),
        AddressZero(AddressZero),
        BalanceNotEnough(BalanceNotEnough),
        BatchIdNotMatch(BatchIdNotMatch),
        BrokerNotAllowed(BrokerNotAllowed),
        CountNotMatch(CountNotMatch),
        InsurancePositionQtyInvalid(InsurancePositionQtyInvalid),
        InsuranceTransferAmountInvalid(InsuranceTransferAmountInvalid),
        InsuranceTransferToSelf(InsuranceTransferToSelf),
        InvalidBizType(InvalidBizType),
        OnlyCrossChainManagerCanCall(OnlyCrossChainManagerCanCall),
        OnlyOperatorCanCall(OnlyOperatorCanCall),
        SignatureNotMatch(SignatureNotMatch),
        SymbolNotAllowed(SymbolNotAllowed),
        TokenNotAllowed(TokenNotAllowed),
        TotalSettleAmountNotMatch(TotalSettleAmountNotMatch),
        UserPerpPositionQtyZero(UserPerpPositionQtyZero),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for operator_managerErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AccountIdInvalid as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccountIdInvalid(decoded));
            }
            if let Ok(decoded) = <AddressZero as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressZero(decoded));
            }
            if let Ok(decoded) = <BalanceNotEnough as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceNotEnough(decoded));
            }
            if let Ok(decoded) = <BatchIdNotMatch as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BatchIdNotMatch(decoded));
            }
            if let Ok(decoded) = <BrokerNotAllowed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BrokerNotAllowed(decoded));
            }
            if let Ok(decoded) = <CountNotMatch as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CountNotMatch(decoded));
            }
            if let Ok(decoded) = <InsurancePositionQtyInvalid as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsurancePositionQtyInvalid(decoded));
            }
            if let Ok(decoded) = <InsuranceTransferAmountInvalid as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsuranceTransferAmountInvalid(decoded));
            }
            if let Ok(decoded) = <InsuranceTransferToSelf as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsuranceTransferToSelf(decoded));
            }
            if let Ok(decoded) = <InvalidBizType as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidBizType(decoded));
            }
            if let Ok(decoded) = <OnlyCrossChainManagerCanCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnlyCrossChainManagerCanCall(decoded));
            }
            if let Ok(decoded) = <OnlyOperatorCanCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnlyOperatorCanCall(decoded));
            }
            if let Ok(decoded) = <SignatureNotMatch as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SignatureNotMatch(decoded));
            }
            if let Ok(decoded) = <SymbolNotAllowed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SymbolNotAllowed(decoded));
            }
            if let Ok(decoded) = <TokenNotAllowed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenNotAllowed(decoded));
            }
            if let Ok(decoded) = <TotalSettleAmountNotMatch as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalSettleAmountNotMatch(decoded));
            }
            if let Ok(decoded) = <UserPerpPositionQtyZero as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UserPerpPositionQtyZero(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for operator_managerErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AccountIdInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceNotEnough(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BatchIdNotMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BrokerNotAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CountNotMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsurancePositionQtyInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsuranceTransferAmountInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsuranceTransferToSelf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidBizType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyCrossChainManagerCanCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyOperatorCanCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignatureNotMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SymbolNotAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenNotAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSettleAmountNotMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UserPerpPositionQtyZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for operator_managerErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AccountIdInvalid as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AddressZero as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <BalanceNotEnough as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BatchIdNotMatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BrokerNotAllowed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CountNotMatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsurancePositionQtyInvalid as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsuranceTransferAmountInvalid as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsuranceTransferToSelf as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidBizType as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyCrossChainManagerCanCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyOperatorCanCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SignatureNotMatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SymbolNotAllowed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TokenNotAllowed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TotalSettleAmountNotMatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UserPerpPositionQtyZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for operator_managerErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccountIdInvalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceNotEnough(element) => ::core::fmt::Display::fmt(element, f),
                Self::BatchIdNotMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::BrokerNotAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::CountNotMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsurancePositionQtyInvalid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsuranceTransferAmountInvalid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsuranceTransferToSelf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidBizType(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyCrossChainManagerCanCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnlyOperatorCanCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SignatureNotMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::SymbolNotAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenNotAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSettleAmountNotMatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UserPerpPositionQtyZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for operator_managerErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccountIdInvalid> for operator_managerErrors {
        fn from(value: AccountIdInvalid) -> Self {
            Self::AccountIdInvalid(value)
        }
    }
    impl ::core::convert::From<AddressZero> for operator_managerErrors {
        fn from(value: AddressZero) -> Self {
            Self::AddressZero(value)
        }
    }
    impl ::core::convert::From<BalanceNotEnough> for operator_managerErrors {
        fn from(value: BalanceNotEnough) -> Self {
            Self::BalanceNotEnough(value)
        }
    }
    impl ::core::convert::From<BatchIdNotMatch> for operator_managerErrors {
        fn from(value: BatchIdNotMatch) -> Self {
            Self::BatchIdNotMatch(value)
        }
    }
    impl ::core::convert::From<BrokerNotAllowed> for operator_managerErrors {
        fn from(value: BrokerNotAllowed) -> Self {
            Self::BrokerNotAllowed(value)
        }
    }
    impl ::core::convert::From<CountNotMatch> for operator_managerErrors {
        fn from(value: CountNotMatch) -> Self {
            Self::CountNotMatch(value)
        }
    }
    impl ::core::convert::From<InsurancePositionQtyInvalid> for operator_managerErrors {
        fn from(value: InsurancePositionQtyInvalid) -> Self {
            Self::InsurancePositionQtyInvalid(value)
        }
    }
    impl ::core::convert::From<InsuranceTransferAmountInvalid>
    for operator_managerErrors {
        fn from(value: InsuranceTransferAmountInvalid) -> Self {
            Self::InsuranceTransferAmountInvalid(value)
        }
    }
    impl ::core::convert::From<InsuranceTransferToSelf> for operator_managerErrors {
        fn from(value: InsuranceTransferToSelf) -> Self {
            Self::InsuranceTransferToSelf(value)
        }
    }
    impl ::core::convert::From<InvalidBizType> for operator_managerErrors {
        fn from(value: InvalidBizType) -> Self {
            Self::InvalidBizType(value)
        }
    }
    impl ::core::convert::From<OnlyCrossChainManagerCanCall> for operator_managerErrors {
        fn from(value: OnlyCrossChainManagerCanCall) -> Self {
            Self::OnlyCrossChainManagerCanCall(value)
        }
    }
    impl ::core::convert::From<OnlyOperatorCanCall> for operator_managerErrors {
        fn from(value: OnlyOperatorCanCall) -> Self {
            Self::OnlyOperatorCanCall(value)
        }
    }
    impl ::core::convert::From<SignatureNotMatch> for operator_managerErrors {
        fn from(value: SignatureNotMatch) -> Self {
            Self::SignatureNotMatch(value)
        }
    }
    impl ::core::convert::From<SymbolNotAllowed> for operator_managerErrors {
        fn from(value: SymbolNotAllowed) -> Self {
            Self::SymbolNotAllowed(value)
        }
    }
    impl ::core::convert::From<TokenNotAllowed> for operator_managerErrors {
        fn from(value: TokenNotAllowed) -> Self {
            Self::TokenNotAllowed(value)
        }
    }
    impl ::core::convert::From<TotalSettleAmountNotMatch> for operator_managerErrors {
        fn from(value: TotalSettleAmountNotMatch) -> Self {
            Self::TotalSettleAmountNotMatch(value)
        }
    }
    impl ::core::convert::From<UserPerpPositionQtyZero> for operator_managerErrors {
        fn from(value: UserPerpPositionQtyZero) -> Self {
            Self::UserPerpPositionQtyZero(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "ChangeCefiUpload",
        abi = "ChangeCefiUpload(uint8,address,address)"
    )]
    pub struct ChangeCefiUploadFilter {
        #[ethevent(indexed)]
        pub types: u8,
        pub old_address: ::ethers::core::types::Address,
        pub new_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ChangeLedger", abi = "ChangeLedger(address,address)")]
    pub struct ChangeLedgerFilter {
        pub old_address: ::ethers::core::types::Address,
        pub new_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "ChangeMarketManager",
        abi = "ChangeMarketManager(address,address)"
    )]
    pub struct ChangeMarketManagerFilter {
        pub old_address: ::ethers::core::types::Address,
        pub new_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ChangeOperator", abi = "ChangeOperator(uint8,address,address)")]
    pub struct ChangeOperatorFilter {
        #[ethevent(indexed)]
        pub types: u8,
        pub old_address: ::ethers::core::types::Address,
        pub new_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "EventUpload", abi = "EventUpload(uint64,uint256)")]
    pub struct EventUpload1Filter {
        #[ethevent(indexed)]
        pub batch_id: u64,
        pub blocktime: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "EventUpload", abi = "EventUpload(uint64)")]
    pub struct EventUpload2Filter {
        #[ethevent(indexed)]
        pub batch_id: u64,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "FuturesTradeUpload", abi = "FuturesTradeUpload(uint64,uint256)")]
    pub struct FuturesTradeUpload1Filter {
        #[ethevent(indexed)]
        pub batch_id: u64,
        pub blocktime: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "FuturesTradeUpload", abi = "FuturesTradeUpload(uint64)")]
    pub struct FuturesTradeUpload2Filter {
        #[ethevent(indexed)]
        pub batch_id: u64,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum operator_managerEvents {
        ChangeCefiUploadFilter(ChangeCefiUploadFilter),
        ChangeLedgerFilter(ChangeLedgerFilter),
        ChangeMarketManagerFilter(ChangeMarketManagerFilter),
        ChangeOperatorFilter(ChangeOperatorFilter),
        EventUpload1Filter(EventUpload1Filter),
        EventUpload2Filter(EventUpload2Filter),
        FuturesTradeUpload1Filter(FuturesTradeUpload1Filter),
        FuturesTradeUpload2Filter(FuturesTradeUpload2Filter),
    }
    impl ::ethers::contract::EthLogDecode for operator_managerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ChangeCefiUploadFilter::decode_log(log) {
                return Ok(operator_managerEvents::ChangeCefiUploadFilter(decoded));
            }
            if let Ok(decoded) = ChangeLedgerFilter::decode_log(log) {
                return Ok(operator_managerEvents::ChangeLedgerFilter(decoded));
            }
            if let Ok(decoded) = ChangeMarketManagerFilter::decode_log(log) {
                return Ok(operator_managerEvents::ChangeMarketManagerFilter(decoded));
            }
            if let Ok(decoded) = ChangeOperatorFilter::decode_log(log) {
                return Ok(operator_managerEvents::ChangeOperatorFilter(decoded));
            }
            if let Ok(decoded) = EventUpload1Filter::decode_log(log) {
                return Ok(operator_managerEvents::EventUpload1Filter(decoded));
            }
            if let Ok(decoded) = EventUpload2Filter::decode_log(log) {
                return Ok(operator_managerEvents::EventUpload2Filter(decoded));
            }
            if let Ok(decoded) = FuturesTradeUpload1Filter::decode_log(log) {
                return Ok(operator_managerEvents::FuturesTradeUpload1Filter(decoded));
            }
            if let Ok(decoded) = FuturesTradeUpload2Filter::decode_log(log) {
                return Ok(operator_managerEvents::FuturesTradeUpload2Filter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for operator_managerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ChangeCefiUploadFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeLedgerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeMarketManagerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeOperatorFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EventUpload1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EventUpload2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FuturesTradeUpload1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FuturesTradeUpload2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ChangeCefiUploadFilter> for operator_managerEvents {
        fn from(value: ChangeCefiUploadFilter) -> Self {
            Self::ChangeCefiUploadFilter(value)
        }
    }
    impl ::core::convert::From<ChangeLedgerFilter> for operator_managerEvents {
        fn from(value: ChangeLedgerFilter) -> Self {
            Self::ChangeLedgerFilter(value)
        }
    }
    impl ::core::convert::From<ChangeMarketManagerFilter> for operator_managerEvents {
        fn from(value: ChangeMarketManagerFilter) -> Self {
            Self::ChangeMarketManagerFilter(value)
        }
    }
    impl ::core::convert::From<ChangeOperatorFilter> for operator_managerEvents {
        fn from(value: ChangeOperatorFilter) -> Self {
            Self::ChangeOperatorFilter(value)
        }
    }
    impl ::core::convert::From<EventUpload1Filter> for operator_managerEvents {
        fn from(value: EventUpload1Filter) -> Self {
            Self::EventUpload1Filter(value)
        }
    }
    impl ::core::convert::From<EventUpload2Filter> for operator_managerEvents {
        fn from(value: EventUpload2Filter) -> Self {
            Self::EventUpload2Filter(value)
        }
    }
    impl ::core::convert::From<FuturesTradeUpload1Filter> for operator_managerEvents {
        fn from(value: FuturesTradeUpload1Filter) -> Self {
            Self::FuturesTradeUpload1Filter(value)
        }
    }
    impl ::core::convert::From<FuturesTradeUpload2Filter> for operator_managerEvents {
        fn from(value: FuturesTradeUpload2Filter) -> Self {
            Self::FuturesTradeUpload2Filter(value)
        }
    }
    ///Container type for all input parameters for the `checkCefiDown` function with signature `checkCefiDown()` and selector `0x1118582d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "checkCefiDown", abi = "checkCefiDown()")]
    pub struct CheckCefiDownCall;
    ///Container type for all input parameters for the `eventUpload` function with signature `eventUpload(((uint8,uint64,bytes)[],bytes32,bytes32,uint8,uint8,uint64))` and selector `0xe8bb8f8f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "eventUpload",
        abi = "eventUpload(((uint8,uint64,bytes)[],bytes32,bytes32,uint8,uint8,uint64))"
    )]
    pub struct EventUploadCall {
        pub data: EventUpload,
    }
    ///Container type for all input parameters for the `futuresTradeUpload` function with signature `futuresTradeUpload((bytes32,bytes32,uint8,uint64,uint8,(bytes32,bytes32,bytes32,int128,int128,uint128,uint128,int128,uint64,uint64,uint64,bool)[]))` and selector `0x2604b9f1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "futuresTradeUpload",
        abi = "futuresTradeUpload((bytes32,bytes32,uint8,uint64,uint8,(bytes32,bytes32,bytes32,int128,int128,uint128,uint128,int128,uint64,uint64,uint64,bool)[]))"
    )]
    pub struct FuturesTradeUploadCall {
        pub data: FuturesTradeUploadData,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize()` and selector `0x8129fc1c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initialize", abi = "initialize()")]
    pub struct InitializeCall;
    ///Container type for all input parameters for the `operatorPing` function with signature `operatorPing()` and selector `0x6ecb9886`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "operatorPing", abi = "operatorPing()")]
    pub struct OperatorPingCall;
    ///Container type for all input parameters for the `perpPriceUpload` function with signature `perpPriceUpload((bytes32,bytes32,uint8,uint64,(bytes32,uint128,uint128,uint64)[]))` and selector `0x79b0d946`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "perpPriceUpload",
        abi = "perpPriceUpload((bytes32,bytes32,uint8,uint64,(bytes32,uint128,uint128,uint64)[]))"
    )]
    pub struct PerpPriceUploadCall {
        pub data: UploadPerpPrice,
    }
    ///Container type for all input parameters for the `setCefiEventUploadAddress` function with signature `setCefiEventUploadAddress(address)` and selector `0x6a8797ea`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setCefiEventUploadAddress",
        abi = "setCefiEventUploadAddress(address)"
    )]
    pub struct SetCefiEventUploadAddressCall {
        pub cefi_event_upload_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setCefiMarketUploadAddress` function with signature `setCefiMarketUploadAddress(address)` and selector `0x72ab8e21`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setCefiMarketUploadAddress",
        abi = "setCefiMarketUploadAddress(address)"
    )]
    pub struct SetCefiMarketUploadAddressCall {
        pub cefi_market_upload_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setCefiPerpTradeUploadAddress` function with signature `setCefiPerpTradeUploadAddress(address)` and selector `0xf9391a18`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setCefiPerpTradeUploadAddress",
        abi = "setCefiPerpTradeUploadAddress(address)"
    )]
    pub struct SetCefiPerpTradeUploadAddressCall {
        pub cefi_perp_trade_upload_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setCefiSpotTradeUploadAddress` function with signature `setCefiSpotTradeUploadAddress(address)` and selector `0xb7c888f2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setCefiSpotTradeUploadAddress",
        abi = "setCefiSpotTradeUploadAddress(address)"
    )]
    pub struct SetCefiSpotTradeUploadAddressCall {
        pub cefi_spot_trade_upload_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setLedger` function with signature `setLedger(address)` and selector `0x3246887d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setLedger", abi = "setLedger(address)")]
    pub struct SetLedgerCall {
        pub ledger: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setMarketManager` function with signature `setMarketManager(address)` and selector `0xd82aff11`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setMarketManager", abi = "setMarketManager(address)")]
    pub struct SetMarketManagerCall {
        pub market_manager_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setOperator` function with signature `setOperator(address)` and selector `0xb3ab15fb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setOperator", abi = "setOperator(address)")]
    pub struct SetOperatorCall {
        pub operator_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `sumUnitaryFundingsUpload` function with signature `sumUnitaryFundingsUpload((bytes32,bytes32,uint8,uint64,(bytes32,int128,uint64)[]))` and selector `0x757a69fd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "sumUnitaryFundingsUpload",
        abi = "sumUnitaryFundingsUpload((bytes32,bytes32,uint8,uint64,(bytes32,int128,uint64)[]))"
    )]
    pub struct SumUnitaryFundingsUploadCall {
        pub data: UploadSumUnitaryFundings,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum operator_managerCalls {
        CheckCefiDown(CheckCefiDownCall),
        EventUpload(EventUploadCall),
        FuturesTradeUpload(FuturesTradeUploadCall),
        Initialize(InitializeCall),
        OperatorPing(OperatorPingCall),
        PerpPriceUpload(PerpPriceUploadCall),
        SetCefiEventUploadAddress(SetCefiEventUploadAddressCall),
        SetCefiMarketUploadAddress(SetCefiMarketUploadAddressCall),
        SetCefiPerpTradeUploadAddress(SetCefiPerpTradeUploadAddressCall),
        SetCefiSpotTradeUploadAddress(SetCefiSpotTradeUploadAddressCall),
        SetLedger(SetLedgerCall),
        SetMarketManager(SetMarketManagerCall),
        SetOperator(SetOperatorCall),
        SumUnitaryFundingsUpload(SumUnitaryFundingsUploadCall),
    }
    impl ::ethers::core::abi::AbiDecode for operator_managerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CheckCefiDownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckCefiDown(decoded));
            }
            if let Ok(decoded) = <EventUploadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EventUpload(decoded));
            }
            if let Ok(decoded) = <FuturesTradeUploadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FuturesTradeUpload(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <OperatorPingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorPing(decoded));
            }
            if let Ok(decoded) = <PerpPriceUploadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PerpPriceUpload(decoded));
            }
            if let Ok(decoded) = <SetCefiEventUploadAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetCefiEventUploadAddress(decoded));
            }
            if let Ok(decoded) = <SetCefiMarketUploadAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetCefiMarketUploadAddress(decoded));
            }
            if let Ok(decoded) = <SetCefiPerpTradeUploadAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetCefiPerpTradeUploadAddress(decoded));
            }
            if let Ok(decoded) = <SetCefiSpotTradeUploadAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetCefiSpotTradeUploadAddress(decoded));
            }
            if let Ok(decoded) = <SetLedgerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetLedger(decoded));
            }
            if let Ok(decoded) = <SetMarketManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetMarketManager(decoded));
            }
            if let Ok(decoded) = <SetOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetOperator(decoded));
            }
            if let Ok(decoded) = <SumUnitaryFundingsUploadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SumUnitaryFundingsUpload(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for operator_managerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckCefiDown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EventUpload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FuturesTradeUpload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorPing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PerpPriceUpload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetCefiEventUploadAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetCefiMarketUploadAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetCefiPerpTradeUploadAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetCefiSpotTradeUploadAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetLedger(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMarketManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SumUnitaryFundingsUpload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for operator_managerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckCefiDown(element) => ::core::fmt::Display::fmt(element, f),
                Self::EventUpload(element) => ::core::fmt::Display::fmt(element, f),
                Self::FuturesTradeUpload(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorPing(element) => ::core::fmt::Display::fmt(element, f),
                Self::PerpPriceUpload(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCefiEventUploadAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetCefiMarketUploadAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetCefiPerpTradeUploadAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetCefiSpotTradeUploadAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetLedger(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMarketManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::SumUnitaryFundingsUpload(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CheckCefiDownCall> for operator_managerCalls {
        fn from(value: CheckCefiDownCall) -> Self {
            Self::CheckCefiDown(value)
        }
    }
    impl ::core::convert::From<EventUploadCall> for operator_managerCalls {
        fn from(value: EventUploadCall) -> Self {
            Self::EventUpload(value)
        }
    }
    impl ::core::convert::From<FuturesTradeUploadCall> for operator_managerCalls {
        fn from(value: FuturesTradeUploadCall) -> Self {
            Self::FuturesTradeUpload(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for operator_managerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<OperatorPingCall> for operator_managerCalls {
        fn from(value: OperatorPingCall) -> Self {
            Self::OperatorPing(value)
        }
    }
    impl ::core::convert::From<PerpPriceUploadCall> for operator_managerCalls {
        fn from(value: PerpPriceUploadCall) -> Self {
            Self::PerpPriceUpload(value)
        }
    }
    impl ::core::convert::From<SetCefiEventUploadAddressCall> for operator_managerCalls {
        fn from(value: SetCefiEventUploadAddressCall) -> Self {
            Self::SetCefiEventUploadAddress(value)
        }
    }
    impl ::core::convert::From<SetCefiMarketUploadAddressCall>
    for operator_managerCalls {
        fn from(value: SetCefiMarketUploadAddressCall) -> Self {
            Self::SetCefiMarketUploadAddress(value)
        }
    }
    impl ::core::convert::From<SetCefiPerpTradeUploadAddressCall>
    for operator_managerCalls {
        fn from(value: SetCefiPerpTradeUploadAddressCall) -> Self {
            Self::SetCefiPerpTradeUploadAddress(value)
        }
    }
    impl ::core::convert::From<SetCefiSpotTradeUploadAddressCall>
    for operator_managerCalls {
        fn from(value: SetCefiSpotTradeUploadAddressCall) -> Self {
            Self::SetCefiSpotTradeUploadAddress(value)
        }
    }
    impl ::core::convert::From<SetLedgerCall> for operator_managerCalls {
        fn from(value: SetLedgerCall) -> Self {
            Self::SetLedger(value)
        }
    }
    impl ::core::convert::From<SetMarketManagerCall> for operator_managerCalls {
        fn from(value: SetMarketManagerCall) -> Self {
            Self::SetMarketManager(value)
        }
    }
    impl ::core::convert::From<SetOperatorCall> for operator_managerCalls {
        fn from(value: SetOperatorCall) -> Self {
            Self::SetOperator(value)
        }
    }
    impl ::core::convert::From<SumUnitaryFundingsUploadCall> for operator_managerCalls {
        fn from(value: SumUnitaryFundingsUploadCall) -> Self {
            Self::SumUnitaryFundingsUpload(value)
        }
    }
    ///Container type for all return fields from the `checkCefiDown` function with signature `checkCefiDown()` and selector `0x1118582d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CheckCefiDownReturn(pub bool);
    ///`EventUpload((uint8,uint64,bytes)[],bytes32,bytes32,uint8,uint8,uint64)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EventUpload {
        pub events: ::std::vec::Vec<EventUploadData>,
        pub r: [u8; 32],
        pub s: [u8; 32],
        pub v: u8,
        pub count: u8,
        pub batch_id: u64,
    }
    ///`EventUploadData(uint8,uint64,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EventUploadData {
        pub biz_type: u8,
        pub event_id: u64,
        pub data: ::ethers::core::types::Bytes,
    }
    ///`PerpPrice(bytes32,uint128,uint128,uint64)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PerpPrice {
        pub symbol_hash: [u8; 32],
        pub index_price: u128,
        pub mark_price: u128,
        pub timestamp: u64,
    }
    ///`SumUnitaryFunding(bytes32,int128,uint64)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SumUnitaryFunding {
        pub symbol_hash: [u8; 32],
        pub sum_unitary_funding: i128,
        pub timestamp: u64,
    }
    ///`UploadPerpPrice(bytes32,bytes32,uint8,uint64,(bytes32,uint128,uint128,uint64)[])`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UploadPerpPrice {
        pub r: [u8; 32],
        pub s: [u8; 32],
        pub v: u8,
        pub max_timestamp: u64,
        pub perp_prices: ::std::vec::Vec<PerpPrice>,
    }
    ///`UploadSumUnitaryFundings(bytes32,bytes32,uint8,uint64,(bytes32,int128,uint64)[])`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UploadSumUnitaryFundings {
        pub r: [u8; 32],
        pub s: [u8; 32],
        pub v: u8,
        pub max_timestamp: u64,
        pub sum_unitary_fundings: ::std::vec::Vec<SumUnitaryFunding>,
    }
    ///`FuturesTradeUpload(bytes32,bytes32,bytes32,int128,int128,uint128,uint128,int128,uint64,uint64,uint64,bool)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FuturesTradeUpload {
        pub account_id: [u8; 32],
        pub symbol_hash: [u8; 32],
        pub fee_asset_hash: [u8; 32],
        pub trade_qty: i128,
        pub notional: i128,
        pub executed_price: u128,
        pub fee: u128,
        pub sum_unitary_fundings: i128,
        pub trade_id: u64,
        pub match_id: u64,
        pub timestamp: u64,
        pub side: bool,
    }
    ///`FuturesTradeUploadData(bytes32,bytes32,uint8,uint64,uint8,(bytes32,bytes32,bytes32,int128,int128,uint128,uint128,int128,uint64,uint64,uint64,bool)[])`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FuturesTradeUploadData {
        pub r: [u8; 32],
        pub s: [u8; 32],
        pub v: u8,
        pub batch_id: u64,
        pub count: u8,
        pub trades: ::std::vec::Vec<FuturesTradeUpload>,
    }
}

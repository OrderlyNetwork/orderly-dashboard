pub use user_ledger::*;
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
pub mod user_ledger {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("accountDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("accountDeposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct AccountTypes.AccountDeposit",
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
                    ::std::borrow::ToOwned::to_owned("accountWithDrawFinish"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "accountWithDrawFinish",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("withdraw"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct AccountTypes.AccountWithdraw",
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
                    ::std::borrow::ToOwned::to_owned("batchGetUserLedger"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("batchGetUserLedger"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("accountIds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct AccountTypes.AccountSnapshot[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("batchGetUserLedger"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("accountIds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbols"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("accountSnapshots"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct AccountTypes.AccountSnapshot[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("crossChainManagerAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "crossChainManagerAddress",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeAdl"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeAdl"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("adl"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct EventTypes.Adl"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("eventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("executeLiquidation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeLiquidation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
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
                                            "struct EventTypes.Liquidation",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("eventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("executeProcessValidatedFutures"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeProcessValidatedFutures",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("trade"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
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
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct PerpTypes.FuturesTradeUpload",
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
                    ::std::borrow::ToOwned::to_owned("executeRebalanceBurn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeRebalanceBurn",
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct RebalanceTypes.RebalanceBurnUploadData",
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
                    ::std::borrow::ToOwned::to_owned("executeRebalanceMint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeRebalanceMint",
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct RebalanceTypes.RebalanceMintUploadData",
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
                    ::std::borrow::ToOwned::to_owned("executeSettlement"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeSettlement"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("settlement"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct EventTypes.Settlement",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("eventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("executeWithdrawAction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeWithdrawAction",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("withdraw"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct EventTypes.WithdrawData",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("eventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("feeManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feeManager"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IFeeManager"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getFrozenWithdrawNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getFrozenWithdrawNonce",
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
                                    name: ::std::borrow::ToOwned::to_owned("withdrawNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("globalDepositId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("globalDepositId"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("globalEventId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("globalEventId"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("marketManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("marketManager"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IMarketManager"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operatorManagerAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "operatorManagerAddress",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rebalanceBurnFinish"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "rebalanceBurnFinish",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct RebalanceTypes.RebalanceBurnCCFinishData",
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
                    ::std::borrow::ToOwned::to_owned("rebalanceMintFinish"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "rebalanceMintFinish",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct RebalanceTypes.RebalanceMintCCFinishData",
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setCrossChainManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setCrossChainManager",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_crossChainManagerAddress",
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
                    ::std::borrow::ToOwned::to_owned("setFeeManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFeeManager"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_feeManagerAddress",
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
                    ::std::borrow::ToOwned::to_owned("setOperatorManagerAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setOperatorManagerAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_operatorManagerAddress",
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
                    ::std::borrow::ToOwned::to_owned("setVaultManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setVaultManager"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_vaultManagerAddress",
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
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
                    ::std::borrow::ToOwned::to_owned("vaultManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("vaultManager"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IVaultManager"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AccountDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AccountDeposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("accountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("depositNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("userAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("srcChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "srcChainDepositNonce",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("brokerHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AccountDeposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("accountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("depositNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("userAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("srcChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "srcChainDepositNonce",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("brokerHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
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
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AccountRegister"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AccountRegister"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("accountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("brokerId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("userAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AccountRegister"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("accountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("brokerId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("userAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AccountWithdrawApprove"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccountWithdrawApprove",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("accountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("brokerHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccountWithdrawApprove",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("accountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("brokerHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
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
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AccountWithdrawFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccountWithdrawFail",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("accountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("brokerHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("failReson"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccountWithdrawFail",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("accountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("brokerHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("blocktime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("failReson"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AccountWithdrawFinish"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccountWithdrawFinish",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("accountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("brokerHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccountWithdrawFinish",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("accountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("brokerHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
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
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AdlResult"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AdlResult"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("accountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "insuranceAccountId",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbolHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "positionQtyTransfer",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "costPositionTransfer",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("adlPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "sumUnitaryFundings",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lastEngineEventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ChangeCrossChainManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ChangeCrossChainManager",
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
                    ::std::borrow::ToOwned::to_owned("ChangeFeeManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChangeFeeManager"),
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
                    ::std::borrow::ToOwned::to_owned("ChangeOperatorManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ChangeOperatorManager",
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
                    ::std::borrow::ToOwned::to_owned("ChangeVaultManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChangeVaultManager"),
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
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LiquidationResult"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LiquidationResult"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "liquidatedAccountId",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "insuranceAccountId",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "liquidatedAssetHash",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "insuranceTransferAmount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lastEngineEventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LiquidationTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LiquidationTransfer",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "liquidationTransferId",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "liquidatorAccountId",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbolHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "positionQtyTransfer",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "costPositionTransfer",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidatorFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("insuranceFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidationFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("markPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "sumUnitaryFundings",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProcessValidatedFutures"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ProcessValidatedFutures",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("accountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbolHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("feeAssetHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tradeQty"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("notional"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("executedPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "sumUnitaryFundings",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tradeId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("matchId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("side"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SettlementExecution"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SettlementExecution",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbolHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("markPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "sumUnitaryFundings",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("settledAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SettlementResult"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SettlementResult"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("accountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("settledAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("settledAssetHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "insuranceAccountId",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "insuranceTransferAmount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "settlementExecutionsCount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lastEngineEventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
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
                    ::std::borrow::ToOwned::to_owned("BrokerNotAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("BrokerNotAllowed"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FrozenBalanceInconsistent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FrozenBalanceInconsistent",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("SafeCastOverflow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SafeCastOverflow"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeCastUnderflow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SafeCastUnderflow"),
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
    pub static USER_LEDGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct user_ledger<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for user_ledger<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for user_ledger<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for user_ledger<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for user_ledger<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(user_ledger))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> user_ledger<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    USER_LEDGER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `accountDeposit` (0x11e0cff4) function
        pub fn account_deposit(
            &self,
            data: AccountDeposit,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([17, 224, 207, 244], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `accountWithDrawFinish` (0x3f83073e) function
        pub fn account_with_draw_finish(
            &self,
            withdraw: AccountWithdraw,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 131, 7, 62], (withdraw,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `batchGetUserLedger` (0x1757cb37) function
        pub fn batch_get_user_ledger(
            &self,
            account_ids: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<AccountSnapshot>,
        > {
            self.0
                .method_hash([23, 87, 203, 55], account_ids)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `batchGetUserLedger` (0x5f225799) function
        pub fn batch_get_user_ledger_with_tokens(
            &self,
            account_ids: ::std::vec::Vec<[u8; 32]>,
            tokens: ::std::vec::Vec<[u8; 32]>,
            symbols: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<AccountSnapshot>,
        > {
            self.0
                .method_hash([95, 34, 87, 153], (account_ids, tokens, symbols))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `crossChainManagerAddress` (0xb182dc69) function
        pub fn cross_chain_manager_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([177, 130, 220, 105], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeAdl` (0xc61ca104) function
        pub fn execute_adl(
            &self,
            adl: Adl,
            event_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 28, 161, 4], (adl, event_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeLiquidation` (0x619fa7fe) function
        pub fn execute_liquidation(
            &self,
            liquidation: Liquidation,
            event_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 159, 167, 254], (liquidation, event_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeProcessValidatedFutures` (0x1522239a) function
        pub fn execute_process_validated_futures(
            &self,
            trade: FuturesTradeUpload,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 34, 35, 154], (trade,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeRebalanceBurn` (0xb76c1210) function
        pub fn execute_rebalance_burn(
            &self,
            data: RebalanceBurnUploadData,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 108, 18, 16], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeRebalanceMint` (0x6fc4bc94) function
        pub fn execute_rebalance_mint(
            &self,
            data: RebalanceMintUploadData,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 196, 188, 148], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeSettlement` (0x7c6c3bd5) function
        pub fn execute_settlement(
            &self,
            settlement: Settlement,
            event_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 108, 59, 213], (settlement, event_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeWithdrawAction` (0x965a1cba) function
        pub fn execute_withdraw_action(
            &self,
            withdraw: WithdrawData,
            event_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 90, 28, 186], (withdraw, event_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeManager` (0xd0fb0203) function
        pub fn fee_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([208, 251, 2, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFrozenWithdrawNonce` (0x782e97e3) function
        pub fn get_frozen_withdraw_nonce(
            &self,
            account_id: [u8; 32],
            withdraw_nonce: u64,
            token_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash(
                    [120, 46, 151, 227],
                    (account_id, withdraw_nonce, token_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `globalDepositId` (0xaae2844b) function
        pub fn global_deposit_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([170, 226, 132, 75], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `globalEventId` (0x08b85a26) function
        pub fn global_event_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([8, 184, 90, 38], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x8129fc1c) function
        pub fn initialize(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `marketManager` (0x41ed2c12) function
        pub fn market_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([65, 237, 44, 18], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorManagerAddress` (0x75bf9f6d) function
        pub fn operator_manager_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([117, 191, 159, 109], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rebalanceBurnFinish` (0x97f8903e) function
        pub fn rebalance_burn_finish(
            &self,
            data: RebalanceBurnCCFinishData,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([151, 248, 144, 62], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rebalanceMintFinish` (0xefb556ed) function
        pub fn rebalance_mint_finish(
            &self,
            data: RebalanceMintCCFinishData,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 181, 86, 237], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setCrossChainManager` (0x5e1eb4ce) function
        pub fn set_cross_chain_manager(
            &self,
            cross_chain_manager_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 30, 180, 206], cross_chain_manager_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeManager` (0x472d35b9) function
        pub fn set_fee_manager(
            &self,
            fee_manager_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 45, 53, 185], fee_manager_address)
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
        ///Calls the contract's `setOperatorManagerAddress` (0xde0c9c86) function
        pub fn set_operator_manager_address(
            &self,
            operator_manager_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 12, 156, 134], operator_manager_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setVaultManager` (0xb543503e) function
        pub fn set_vault_manager(
            &self,
            vault_manager_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 67, 80, 62], vault_manager_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vaultManager` (0x8a4adf24) function
        pub fn vault_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([138, 74, 223, 36], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AccountDeposit` event
        pub fn account_deposit_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountDeposit1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AccountDeposit` event
        pub fn account_deposit_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountDeposit2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AccountRegister` event
        pub fn account_register_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountRegister1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AccountRegister` event
        pub fn account_register_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountRegister2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AccountWithdrawApprove` event
        pub fn account_withdraw_approve_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountWithdrawApprove1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AccountWithdrawApprove` event
        pub fn account_withdraw_approve_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountWithdrawApprove2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AccountWithdrawFail` event
        pub fn account_withdraw_fail_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountWithdrawFail1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AccountWithdrawFail` event
        pub fn account_withdraw_fail_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountWithdrawFail2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AccountWithdrawFinish` event
        pub fn account_withdraw_finish_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountWithdrawFinish1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AccountWithdrawFinish` event
        pub fn account_withdraw_finish_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountWithdrawFinish2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AdlResult` event
        pub fn adl_result_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AdlResultFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangeCrossChainManager` event
        pub fn change_cross_chain_manager_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangeCrossChainManagerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangeFeeManager` event
        pub fn change_fee_manager_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangeFeeManagerFilter,
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
        ///Gets the contract's `ChangeOperatorManager` event
        pub fn change_operator_manager_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangeOperatorManagerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangeVaultManager` event
        pub fn change_vault_manager_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangeVaultManagerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LiquidationResult` event
        pub fn liquidation_result_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LiquidationResultFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LiquidationTransfer` event
        pub fn liquidation_transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LiquidationTransferFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProcessValidatedFutures` event
        pub fn process_validated_futures_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProcessValidatedFuturesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SettlementExecution` event
        pub fn settlement_execution_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SettlementExecutionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SettlementResult` event
        pub fn settlement_result_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SettlementResultFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            user_ledgerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for user_ledger<M> {
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
    ///Custom Error type `FrozenBalanceInconsistent` with signature `FrozenBalanceInconsistent()` and selector `0xdc6db21a`
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
    #[etherror(name = "FrozenBalanceInconsistent", abi = "FrozenBalanceInconsistent()")]
    pub struct FrozenBalanceInconsistent;
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
    ///Custom Error type `SafeCastOverflow` with signature `SafeCastOverflow()` and selector `0x93dafdf1`
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
    #[etherror(name = "SafeCastOverflow", abi = "SafeCastOverflow()")]
    pub struct SafeCastOverflow;
    ///Custom Error type `SafeCastUnderflow` with signature `SafeCastUnderflow()` and selector `0x0101bd74`
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
    #[etherror(name = "SafeCastUnderflow", abi = "SafeCastUnderflow()")]
    pub struct SafeCastUnderflow;
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
    pub enum user_ledgerErrors {
        AccountIdInvalid(AccountIdInvalid),
        AddressZero(AddressZero),
        BalanceNotEnough(BalanceNotEnough),
        BrokerNotAllowed(BrokerNotAllowed),
        FrozenBalanceInconsistent(FrozenBalanceInconsistent),
        InsurancePositionQtyInvalid(InsurancePositionQtyInvalid),
        InsuranceTransferAmountInvalid(InsuranceTransferAmountInvalid),
        InsuranceTransferToSelf(InsuranceTransferToSelf),
        OnlyCrossChainManagerCanCall(OnlyCrossChainManagerCanCall),
        OnlyOperatorCanCall(OnlyOperatorCanCall),
        SafeCastOverflow(SafeCastOverflow),
        SafeCastUnderflow(SafeCastUnderflow),
        SymbolNotAllowed(SymbolNotAllowed),
        TokenNotAllowed(TokenNotAllowed),
        TotalSettleAmountNotMatch(TotalSettleAmountNotMatch),
        UserPerpPositionQtyZero(UserPerpPositionQtyZero),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for user_ledgerErrors {
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
            if let Ok(decoded) = <BrokerNotAllowed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BrokerNotAllowed(decoded));
            }
            if let Ok(decoded) = <FrozenBalanceInconsistent as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FrozenBalanceInconsistent(decoded));
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
            if let Ok(decoded) = <SafeCastOverflow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeCastOverflow(decoded));
            }
            if let Ok(decoded) = <SafeCastUnderflow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeCastUnderflow(decoded));
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
    impl ::ethers::core::abi::AbiEncode for user_ledgerErrors {
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
                Self::BrokerNotAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FrozenBalanceInconsistent(element) => {
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
                Self::OnlyCrossChainManagerCanCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyOperatorCanCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeCastOverflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeCastUnderflow(element) => {
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
    impl ::ethers::contract::ContractRevert for user_ledgerErrors {
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
                    == <BrokerNotAllowed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FrozenBalanceInconsistent as ::ethers::contract::EthError>::selector() => {
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
                    == <OnlyCrossChainManagerCanCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyOperatorCanCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeCastOverflow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeCastUnderflow as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for user_ledgerErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccountIdInvalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceNotEnough(element) => ::core::fmt::Display::fmt(element, f),
                Self::BrokerNotAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::FrozenBalanceInconsistent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsurancePositionQtyInvalid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsuranceTransferAmountInvalid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsuranceTransferToSelf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnlyCrossChainManagerCanCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnlyOperatorCanCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeCastOverflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeCastUnderflow(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<::std::string::String> for user_ledgerErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccountIdInvalid> for user_ledgerErrors {
        fn from(value: AccountIdInvalid) -> Self {
            Self::AccountIdInvalid(value)
        }
    }
    impl ::core::convert::From<AddressZero> for user_ledgerErrors {
        fn from(value: AddressZero) -> Self {
            Self::AddressZero(value)
        }
    }
    impl ::core::convert::From<BalanceNotEnough> for user_ledgerErrors {
        fn from(value: BalanceNotEnough) -> Self {
            Self::BalanceNotEnough(value)
        }
    }
    impl ::core::convert::From<BrokerNotAllowed> for user_ledgerErrors {
        fn from(value: BrokerNotAllowed) -> Self {
            Self::BrokerNotAllowed(value)
        }
    }
    impl ::core::convert::From<FrozenBalanceInconsistent> for user_ledgerErrors {
        fn from(value: FrozenBalanceInconsistent) -> Self {
            Self::FrozenBalanceInconsistent(value)
        }
    }
    impl ::core::convert::From<InsurancePositionQtyInvalid> for user_ledgerErrors {
        fn from(value: InsurancePositionQtyInvalid) -> Self {
            Self::InsurancePositionQtyInvalid(value)
        }
    }
    impl ::core::convert::From<InsuranceTransferAmountInvalid> for user_ledgerErrors {
        fn from(value: InsuranceTransferAmountInvalid) -> Self {
            Self::InsuranceTransferAmountInvalid(value)
        }
    }
    impl ::core::convert::From<InsuranceTransferToSelf> for user_ledgerErrors {
        fn from(value: InsuranceTransferToSelf) -> Self {
            Self::InsuranceTransferToSelf(value)
        }
    }
    impl ::core::convert::From<OnlyCrossChainManagerCanCall> for user_ledgerErrors {
        fn from(value: OnlyCrossChainManagerCanCall) -> Self {
            Self::OnlyCrossChainManagerCanCall(value)
        }
    }
    impl ::core::convert::From<OnlyOperatorCanCall> for user_ledgerErrors {
        fn from(value: OnlyOperatorCanCall) -> Self {
            Self::OnlyOperatorCanCall(value)
        }
    }
    impl ::core::convert::From<SafeCastOverflow> for user_ledgerErrors {
        fn from(value: SafeCastOverflow) -> Self {
            Self::SafeCastOverflow(value)
        }
    }
    impl ::core::convert::From<SafeCastUnderflow> for user_ledgerErrors {
        fn from(value: SafeCastUnderflow) -> Self {
            Self::SafeCastUnderflow(value)
        }
    }
    impl ::core::convert::From<SymbolNotAllowed> for user_ledgerErrors {
        fn from(value: SymbolNotAllowed) -> Self {
            Self::SymbolNotAllowed(value)
        }
    }
    impl ::core::convert::From<TokenNotAllowed> for user_ledgerErrors {
        fn from(value: TokenNotAllowed) -> Self {
            Self::TokenNotAllowed(value)
        }
    }
    impl ::core::convert::From<TotalSettleAmountNotMatch> for user_ledgerErrors {
        fn from(value: TotalSettleAmountNotMatch) -> Self {
            Self::TotalSettleAmountNotMatch(value)
        }
    }
    impl ::core::convert::From<UserPerpPositionQtyZero> for user_ledgerErrors {
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
        name = "AccountDeposit",
        abi = "AccountDeposit(bytes32,uint64,uint64,address,bytes32,uint128,uint256,uint64,bytes32)"
    )]
    pub struct AccountDeposit1Filter {
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        #[ethevent(indexed)]
        pub deposit_nonce: u64,
        #[ethevent(indexed)]
        pub event_id: u64,
        pub user_address: ::ethers::core::types::Address,
        pub token_hash: [u8; 32],
        pub token_amount: u128,
        pub src_chain_id: ::ethers::core::types::U256,
        pub src_chain_deposit_nonce: u64,
        pub broker_hash: [u8; 32],
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
        name = "AccountDeposit",
        abi = "AccountDeposit(bytes32,uint64,uint64,address,bytes32,uint128,uint256,uint64,bytes32,uint256)"
    )]
    pub struct AccountDeposit2Filter {
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        #[ethevent(indexed)]
        pub deposit_nonce: u64,
        #[ethevent(indexed)]
        pub event_id: u64,
        pub user_address: ::ethers::core::types::Address,
        pub token_hash: [u8; 32],
        pub token_amount: u128,
        pub src_chain_id: ::ethers::core::types::U256,
        pub src_chain_deposit_nonce: u64,
        pub broker_hash: [u8; 32],
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
    #[ethevent(
        name = "AccountRegister",
        abi = "AccountRegister(bytes32,bytes32,address)"
    )]
    pub struct AccountRegister1Filter {
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        #[ethevent(indexed)]
        pub broker_id: [u8; 32],
        #[ethevent(indexed)]
        pub user_address: ::ethers::core::types::Address,
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
        name = "AccountRegister",
        abi = "AccountRegister(bytes32,bytes32,address,uint256)"
    )]
    pub struct AccountRegister2Filter {
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        #[ethevent(indexed)]
        pub broker_id: [u8; 32],
        #[ethevent(indexed)]
        pub user_address: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "AccountWithdrawApprove",
        abi = "AccountWithdrawApprove(bytes32,uint64,uint64,bytes32,address,address,uint256,bytes32,uint128,uint128)"
    )]
    pub struct AccountWithdrawApprove1Filter {
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        #[ethevent(indexed)]
        pub withdraw_nonce: u64,
        #[ethevent(indexed)]
        pub event_id: u64,
        pub broker_hash: [u8; 32],
        pub sender: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
        pub chain_id: ::ethers::core::types::U256,
        pub token_hash: [u8; 32],
        pub token_amount: u128,
        pub fee: u128,
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
        name = "AccountWithdrawApprove",
        abi = "AccountWithdrawApprove(bytes32,uint64,uint64,bytes32,address,address,uint256,bytes32,uint128,uint128,uint256)"
    )]
    pub struct AccountWithdrawApprove2Filter {
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        #[ethevent(indexed)]
        pub withdraw_nonce: u64,
        #[ethevent(indexed)]
        pub event_id: u64,
        pub broker_hash: [u8; 32],
        pub sender: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
        pub chain_id: ::ethers::core::types::U256,
        pub token_hash: [u8; 32],
        pub token_amount: u128,
        pub fee: u128,
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
    #[ethevent(
        name = "AccountWithdrawFail",
        abi = "AccountWithdrawFail(bytes32,uint64,uint64,bytes32,address,address,uint256,bytes32,uint128,uint128,uint8)"
    )]
    pub struct AccountWithdrawFail1Filter {
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        #[ethevent(indexed)]
        pub withdraw_nonce: u64,
        #[ethevent(indexed)]
        pub event_id: u64,
        pub broker_hash: [u8; 32],
        pub sender: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
        pub chain_id: ::ethers::core::types::U256,
        pub token_hash: [u8; 32],
        pub token_amount: u128,
        pub fee: u128,
        pub fail_reson: u8,
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
        name = "AccountWithdrawFail",
        abi = "AccountWithdrawFail(bytes32,uint64,uint64,bytes32,address,address,uint256,bytes32,uint128,uint128,uint256,uint8)"
    )]
    pub struct AccountWithdrawFail2Filter {
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        #[ethevent(indexed)]
        pub withdraw_nonce: u64,
        #[ethevent(indexed)]
        pub event_id: u64,
        pub broker_hash: [u8; 32],
        pub sender: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
        pub chain_id: ::ethers::core::types::U256,
        pub token_hash: [u8; 32],
        pub token_amount: u128,
        pub fee: u128,
        pub blocktime: ::ethers::core::types::U256,
        pub fail_reson: u8,
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
        name = "AccountWithdrawFinish",
        abi = "AccountWithdrawFinish(bytes32,uint64,uint64,bytes32,address,address,uint256,bytes32,uint128,uint128)"
    )]
    pub struct AccountWithdrawFinish1Filter {
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        #[ethevent(indexed)]
        pub withdraw_nonce: u64,
        #[ethevent(indexed)]
        pub event_id: u64,
        pub broker_hash: [u8; 32],
        pub sender: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
        pub chain_id: ::ethers::core::types::U256,
        pub token_hash: [u8; 32],
        pub token_amount: u128,
        pub fee: u128,
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
        name = "AccountWithdrawFinish",
        abi = "AccountWithdrawFinish(bytes32,uint64,uint64,bytes32,address,address,uint256,bytes32,uint128,uint128,uint256)"
    )]
    pub struct AccountWithdrawFinish2Filter {
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        #[ethevent(indexed)]
        pub withdraw_nonce: u64,
        #[ethevent(indexed)]
        pub event_id: u64,
        pub broker_hash: [u8; 32],
        pub sender: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
        pub chain_id: ::ethers::core::types::U256,
        pub token_hash: [u8; 32],
        pub token_amount: u128,
        pub fee: u128,
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
    #[ethevent(
        name = "AdlResult",
        abi = "AdlResult(uint64,bytes32,bytes32,bytes32,int128,int128,uint128,int128,uint64)"
    )]
    pub struct AdlResultFilter {
        #[ethevent(indexed)]
        pub event_id: u64,
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        pub insurance_account_id: [u8; 32],
        pub symbol_hash: [u8; 32],
        pub position_qty_transfer: i128,
        pub cost_position_transfer: i128,
        pub adl_price: u128,
        pub sum_unitary_fundings: i128,
        pub last_engine_event_id: u64,
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
        name = "ChangeCrossChainManager",
        abi = "ChangeCrossChainManager(address,address)"
    )]
    pub struct ChangeCrossChainManagerFilter {
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
    #[ethevent(name = "ChangeFeeManager", abi = "ChangeFeeManager(address,address)")]
    pub struct ChangeFeeManagerFilter {
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
    #[ethevent(
        name = "ChangeOperatorManager",
        abi = "ChangeOperatorManager(address,address)"
    )]
    pub struct ChangeOperatorManagerFilter {
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
    #[ethevent(name = "ChangeVaultManager", abi = "ChangeVaultManager(address,address)")]
    pub struct ChangeVaultManagerFilter {
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
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
        name = "LiquidationResult",
        abi = "LiquidationResult(uint64,bytes32,bytes32,bytes32,uint128,uint64)"
    )]
    pub struct LiquidationResultFilter {
        #[ethevent(indexed)]
        pub event_id: u64,
        #[ethevent(indexed)]
        pub liquidated_account_id: [u8; 32],
        #[ethevent(indexed)]
        pub insurance_account_id: [u8; 32],
        pub liquidated_asset_hash: [u8; 32],
        pub insurance_transfer_amount: u128,
        pub last_engine_event_id: u64,
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
        name = "LiquidationTransfer",
        abi = "LiquidationTransfer(uint64,bytes32,bytes32,int128,int128,int128,int128,int128,uint128,int128)"
    )]
    pub struct LiquidationTransferFilter {
        #[ethevent(indexed)]
        pub liquidation_transfer_id: u64,
        #[ethevent(indexed)]
        pub liquidator_account_id: [u8; 32],
        #[ethevent(indexed)]
        pub symbol_hash: [u8; 32],
        pub position_qty_transfer: i128,
        pub cost_position_transfer: i128,
        pub liquidator_fee: i128,
        pub insurance_fee: i128,
        pub liquidation_fee: i128,
        pub mark_price: u128,
        pub sum_unitary_fundings: i128,
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
        name = "ProcessValidatedFutures",
        abi = "ProcessValidatedFutures(bytes32,bytes32,bytes32,int128,int128,uint128,uint128,int128,uint64,uint64,uint64,bool)"
    )]
    pub struct ProcessValidatedFuturesFilter {
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        #[ethevent(indexed)]
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
        name = "SettlementExecution",
        abi = "SettlementExecution(bytes32,uint128,int128,int128)"
    )]
    pub struct SettlementExecutionFilter {
        #[ethevent(indexed)]
        pub symbol_hash: [u8; 32],
        pub mark_price: u128,
        pub sum_unitary_fundings: i128,
        pub settled_amount: i128,
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
        name = "SettlementResult",
        abi = "SettlementResult(uint64,bytes32,int128,bytes32,bytes32,uint128,uint64,uint64)"
    )]
    pub struct SettlementResultFilter {
        #[ethevent(indexed)]
        pub event_id: u64,
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        pub settled_amount: i128,
        pub settled_asset_hash: [u8; 32],
        pub insurance_account_id: [u8; 32],
        pub insurance_transfer_amount: u128,
        pub settlement_executions_count: u64,
        pub last_engine_event_id: u64,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum user_ledgerEvents {
        AccountDeposit1Filter(AccountDeposit1Filter),
        AccountDeposit2Filter(AccountDeposit2Filter),
        AccountRegister1Filter(AccountRegister1Filter),
        AccountRegister2Filter(AccountRegister2Filter),
        AccountWithdrawApprove1Filter(AccountWithdrawApprove1Filter),
        AccountWithdrawApprove2Filter(AccountWithdrawApprove2Filter),
        AccountWithdrawFail1Filter(AccountWithdrawFail1Filter),
        AccountWithdrawFail2Filter(AccountWithdrawFail2Filter),
        AccountWithdrawFinish1Filter(AccountWithdrawFinish1Filter),
        AccountWithdrawFinish2Filter(AccountWithdrawFinish2Filter),
        AdlResultFilter(AdlResultFilter),
        ChangeCrossChainManagerFilter(ChangeCrossChainManagerFilter),
        ChangeFeeManagerFilter(ChangeFeeManagerFilter),
        ChangeMarketManagerFilter(ChangeMarketManagerFilter),
        ChangeOperatorManagerFilter(ChangeOperatorManagerFilter),
        ChangeVaultManagerFilter(ChangeVaultManagerFilter),
        InitializedFilter(InitializedFilter),
        LiquidationResultFilter(LiquidationResultFilter),
        LiquidationTransferFilter(LiquidationTransferFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ProcessValidatedFuturesFilter(ProcessValidatedFuturesFilter),
        SettlementExecutionFilter(SettlementExecutionFilter),
        SettlementResultFilter(SettlementResultFilter),
    }
    impl ::ethers::contract::EthLogDecode for user_ledgerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AccountDeposit1Filter::decode_log(log) {
                return Ok(user_ledgerEvents::AccountDeposit1Filter(decoded));
            }
            if let Ok(decoded) = AccountDeposit2Filter::decode_log(log) {
                return Ok(user_ledgerEvents::AccountDeposit2Filter(decoded));
            }
            if let Ok(decoded) = AccountRegister1Filter::decode_log(log) {
                return Ok(user_ledgerEvents::AccountRegister1Filter(decoded));
            }
            if let Ok(decoded) = AccountRegister2Filter::decode_log(log) {
                return Ok(user_ledgerEvents::AccountRegister2Filter(decoded));
            }
            if let Ok(decoded) = AccountWithdrawApprove1Filter::decode_log(log) {
                return Ok(user_ledgerEvents::AccountWithdrawApprove1Filter(decoded));
            }
            if let Ok(decoded) = AccountWithdrawApprove2Filter::decode_log(log) {
                return Ok(user_ledgerEvents::AccountWithdrawApprove2Filter(decoded));
            }
            if let Ok(decoded) = AccountWithdrawFail1Filter::decode_log(log) {
                return Ok(user_ledgerEvents::AccountWithdrawFail1Filter(decoded));
            }
            if let Ok(decoded) = AccountWithdrawFail2Filter::decode_log(log) {
                return Ok(user_ledgerEvents::AccountWithdrawFail2Filter(decoded));
            }
            if let Ok(decoded) = AccountWithdrawFinish1Filter::decode_log(log) {
                return Ok(user_ledgerEvents::AccountWithdrawFinish1Filter(decoded));
            }
            if let Ok(decoded) = AccountWithdrawFinish2Filter::decode_log(log) {
                return Ok(user_ledgerEvents::AccountWithdrawFinish2Filter(decoded));
            }
            if let Ok(decoded) = AdlResultFilter::decode_log(log) {
                return Ok(user_ledgerEvents::AdlResultFilter(decoded));
            }
            if let Ok(decoded) = ChangeCrossChainManagerFilter::decode_log(log) {
                return Ok(user_ledgerEvents::ChangeCrossChainManagerFilter(decoded));
            }
            if let Ok(decoded) = ChangeFeeManagerFilter::decode_log(log) {
                return Ok(user_ledgerEvents::ChangeFeeManagerFilter(decoded));
            }
            if let Ok(decoded) = ChangeMarketManagerFilter::decode_log(log) {
                return Ok(user_ledgerEvents::ChangeMarketManagerFilter(decoded));
            }
            if let Ok(decoded) = ChangeOperatorManagerFilter::decode_log(log) {
                return Ok(user_ledgerEvents::ChangeOperatorManagerFilter(decoded));
            }
            if let Ok(decoded) = ChangeVaultManagerFilter::decode_log(log) {
                return Ok(user_ledgerEvents::ChangeVaultManagerFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(user_ledgerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = LiquidationResultFilter::decode_log(log) {
                return Ok(user_ledgerEvents::LiquidationResultFilter(decoded));
            }
            if let Ok(decoded) = LiquidationTransferFilter::decode_log(log) {
                return Ok(user_ledgerEvents::LiquidationTransferFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(user_ledgerEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = ProcessValidatedFuturesFilter::decode_log(log) {
                return Ok(user_ledgerEvents::ProcessValidatedFuturesFilter(decoded));
            }
            if let Ok(decoded) = SettlementExecutionFilter::decode_log(log) {
                return Ok(user_ledgerEvents::SettlementExecutionFilter(decoded));
            }
            if let Ok(decoded) = SettlementResultFilter::decode_log(log) {
                return Ok(user_ledgerEvents::SettlementResultFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for user_ledgerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccountDeposit1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccountDeposit2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccountRegister1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccountRegister2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccountWithdrawApprove1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccountWithdrawApprove2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccountWithdrawFail1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccountWithdrawFail2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccountWithdrawFinish1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccountWithdrawFinish2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AdlResultFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeCrossChainManagerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeFeeManagerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeMarketManagerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeOperatorManagerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeVaultManagerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidationResultFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidationTransferFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProcessValidatedFuturesFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SettlementExecutionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SettlementResultFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AccountDeposit1Filter> for user_ledgerEvents {
        fn from(value: AccountDeposit1Filter) -> Self {
            Self::AccountDeposit1Filter(value)
        }
    }
    impl ::core::convert::From<AccountDeposit2Filter> for user_ledgerEvents {
        fn from(value: AccountDeposit2Filter) -> Self {
            Self::AccountDeposit2Filter(value)
        }
    }
    impl ::core::convert::From<AccountRegister1Filter> for user_ledgerEvents {
        fn from(value: AccountRegister1Filter) -> Self {
            Self::AccountRegister1Filter(value)
        }
    }
    impl ::core::convert::From<AccountRegister2Filter> for user_ledgerEvents {
        fn from(value: AccountRegister2Filter) -> Self {
            Self::AccountRegister2Filter(value)
        }
    }
    impl ::core::convert::From<AccountWithdrawApprove1Filter> for user_ledgerEvents {
        fn from(value: AccountWithdrawApprove1Filter) -> Self {
            Self::AccountWithdrawApprove1Filter(value)
        }
    }
    impl ::core::convert::From<AccountWithdrawApprove2Filter> for user_ledgerEvents {
        fn from(value: AccountWithdrawApprove2Filter) -> Self {
            Self::AccountWithdrawApprove2Filter(value)
        }
    }
    impl ::core::convert::From<AccountWithdrawFail1Filter> for user_ledgerEvents {
        fn from(value: AccountWithdrawFail1Filter) -> Self {
            Self::AccountWithdrawFail1Filter(value)
        }
    }
    impl ::core::convert::From<AccountWithdrawFail2Filter> for user_ledgerEvents {
        fn from(value: AccountWithdrawFail2Filter) -> Self {
            Self::AccountWithdrawFail2Filter(value)
        }
    }
    impl ::core::convert::From<AccountWithdrawFinish1Filter> for user_ledgerEvents {
        fn from(value: AccountWithdrawFinish1Filter) -> Self {
            Self::AccountWithdrawFinish1Filter(value)
        }
    }
    impl ::core::convert::From<AccountWithdrawFinish2Filter> for user_ledgerEvents {
        fn from(value: AccountWithdrawFinish2Filter) -> Self {
            Self::AccountWithdrawFinish2Filter(value)
        }
    }
    impl ::core::convert::From<AdlResultFilter> for user_ledgerEvents {
        fn from(value: AdlResultFilter) -> Self {
            Self::AdlResultFilter(value)
        }
    }
    impl ::core::convert::From<ChangeCrossChainManagerFilter> for user_ledgerEvents {
        fn from(value: ChangeCrossChainManagerFilter) -> Self {
            Self::ChangeCrossChainManagerFilter(value)
        }
    }
    impl ::core::convert::From<ChangeFeeManagerFilter> for user_ledgerEvents {
        fn from(value: ChangeFeeManagerFilter) -> Self {
            Self::ChangeFeeManagerFilter(value)
        }
    }
    impl ::core::convert::From<ChangeMarketManagerFilter> for user_ledgerEvents {
        fn from(value: ChangeMarketManagerFilter) -> Self {
            Self::ChangeMarketManagerFilter(value)
        }
    }
    impl ::core::convert::From<ChangeOperatorManagerFilter> for user_ledgerEvents {
        fn from(value: ChangeOperatorManagerFilter) -> Self {
            Self::ChangeOperatorManagerFilter(value)
        }
    }
    impl ::core::convert::From<ChangeVaultManagerFilter> for user_ledgerEvents {
        fn from(value: ChangeVaultManagerFilter) -> Self {
            Self::ChangeVaultManagerFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for user_ledgerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<LiquidationResultFilter> for user_ledgerEvents {
        fn from(value: LiquidationResultFilter) -> Self {
            Self::LiquidationResultFilter(value)
        }
    }
    impl ::core::convert::From<LiquidationTransferFilter> for user_ledgerEvents {
        fn from(value: LiquidationTransferFilter) -> Self {
            Self::LiquidationTransferFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for user_ledgerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<ProcessValidatedFuturesFilter> for user_ledgerEvents {
        fn from(value: ProcessValidatedFuturesFilter) -> Self {
            Self::ProcessValidatedFuturesFilter(value)
        }
    }
    impl ::core::convert::From<SettlementExecutionFilter> for user_ledgerEvents {
        fn from(value: SettlementExecutionFilter) -> Self {
            Self::SettlementExecutionFilter(value)
        }
    }
    impl ::core::convert::From<SettlementResultFilter> for user_ledgerEvents {
        fn from(value: SettlementResultFilter) -> Self {
            Self::SettlementResultFilter(value)
        }
    }
    ///Container type for all input parameters for the `accountDeposit` function with signature `accountDeposit((bytes32,bytes32,address,bytes32,uint256,uint128,uint64))` and selector `0x11e0cff4`
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
        name = "accountDeposit",
        abi = "accountDeposit((bytes32,bytes32,address,bytes32,uint256,uint128,uint64))"
    )]
    pub struct AccountDepositCall {
        pub data: AccountDeposit,
    }
    ///Container type for all input parameters for the `accountWithDrawFinish` function with signature `accountWithDrawFinish((bytes32,address,address,bytes32,bytes32,uint128,uint128,uint256,uint64))` and selector `0x3f83073e`
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
        name = "accountWithDrawFinish",
        abi = "accountWithDrawFinish((bytes32,address,address,bytes32,bytes32,uint128,uint128,uint256,uint64))"
    )]
    pub struct AccountWithDrawFinishCall {
        pub withdraw: AccountWithdraw,
    }
    ///Container type for all input parameters for the `batchGetUserLedger` function with signature `batchGetUserLedger(bytes32[])` and selector `0x1757cb37`
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
    #[ethcall(name = "batchGetUserLedger", abi = "batchGetUserLedger(bytes32[])")]
    pub struct BatchGetUserLedgerCall {
        pub account_ids: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `batchGetUserLedger` function with signature `batchGetUserLedger(bytes32[],bytes32[],bytes32[])` and selector `0x5f225799`
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
        name = "batchGetUserLedger",
        abi = "batchGetUserLedger(bytes32[],bytes32[],bytes32[])"
    )]
    pub struct BatchGetUserLedgerWithTokensCall {
        pub account_ids: ::std::vec::Vec<[u8; 32]>,
        pub tokens: ::std::vec::Vec<[u8; 32]>,
        pub symbols: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `crossChainManagerAddress` function with signature `crossChainManagerAddress()` and selector `0xb182dc69`
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
    #[ethcall(name = "crossChainManagerAddress", abi = "crossChainManagerAddress()")]
    pub struct CrossChainManagerAddressCall;
    ///Container type for all input parameters for the `executeAdl` function with signature `executeAdl((bytes32,bytes32,bytes32,int128,int128,uint128,int128,uint64),uint64)` and selector `0xc61ca104`
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
        name = "executeAdl",
        abi = "executeAdl((bytes32,bytes32,bytes32,int128,int128,uint128,int128,uint64),uint64)"
    )]
    pub struct ExecuteAdlCall {
        pub adl: Adl,
        pub event_id: u64,
    }
    ///Container type for all input parameters for the `executeLiquidation` function with signature `executeLiquidation((bytes32,bytes32,bytes32,uint128,uint64,(bytes32,bytes32,int128,int128,int128,int128,int128,uint128,int128,uint64)[]),uint64)` and selector `0x619fa7fe`
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
        name = "executeLiquidation",
        abi = "executeLiquidation((bytes32,bytes32,bytes32,uint128,uint64,(bytes32,bytes32,int128,int128,int128,int128,int128,uint128,int128,uint64)[]),uint64)"
    )]
    pub struct ExecuteLiquidationCall {
        pub liquidation: Liquidation,
        pub event_id: u64,
    }
    ///Container type for all input parameters for the `executeProcessValidatedFutures` function with signature `executeProcessValidatedFutures((bytes32,bytes32,bytes32,int128,int128,uint128,uint128,int128,uint64,uint64,uint64,bool))` and selector `0x1522239a`
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
        name = "executeProcessValidatedFutures",
        abi = "executeProcessValidatedFutures((bytes32,bytes32,bytes32,int128,int128,uint128,uint128,int128,uint64,uint64,uint64,bool))"
    )]
    pub struct ExecuteProcessValidatedFuturesCall {
        pub trade: FuturesTradeUpload,
    }
    ///Container type for all input parameters for the `executeRebalanceBurn` function with signature `executeRebalanceBurn((bytes32,bytes32,uint8,uint64,uint128,bytes32,uint256,uint256))` and selector `0xb76c1210`
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
        name = "executeRebalanceBurn",
        abi = "executeRebalanceBurn((bytes32,bytes32,uint8,uint64,uint128,bytes32,uint256,uint256))"
    )]
    pub struct ExecuteRebalanceBurnCall {
        pub data: RebalanceBurnUploadData,
    }
    ///Container type for all input parameters for the `executeRebalanceMint` function with signature `executeRebalanceMint((bytes32,bytes32,uint8,uint64,uint128,bytes32,uint256,uint256,bytes,bytes))` and selector `0x6fc4bc94`
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
        name = "executeRebalanceMint",
        abi = "executeRebalanceMint((bytes32,bytes32,uint8,uint64,uint128,bytes32,uint256,uint256,bytes,bytes))"
    )]
    pub struct ExecuteRebalanceMintCall {
        pub data: RebalanceMintUploadData,
    }
    ///Container type for all input parameters for the `executeSettlement` function with signature `executeSettlement((bytes32,bytes32,bytes32,int128,uint128,uint64,(bytes32,uint128,int128,int128)[]),uint64)` and selector `0x7c6c3bd5`
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
        name = "executeSettlement",
        abi = "executeSettlement((bytes32,bytes32,bytes32,int128,uint128,uint64,(bytes32,uint128,int128,int128)[]),uint64)"
    )]
    pub struct ExecuteSettlementCall {
        pub settlement: Settlement,
        pub event_id: u64,
    }
    ///Container type for all input parameters for the `executeWithdrawAction` function with signature `executeWithdrawAction((uint128,uint128,uint256,bytes32,bytes32,bytes32,uint8,address,uint64,address,uint64,string,string),uint64)` and selector `0x965a1cba`
    #[derive(Clone, ::ethers::contract::EthCall, ::ethers::contract::EthDisplay)]
    #[ethcall(
        name = "executeWithdrawAction",
        abi = "executeWithdrawAction((uint128,uint128,uint256,bytes32,bytes32,bytes32,uint8,address,uint64,address,uint64,string,string),uint64)"
    )]
    pub struct ExecuteWithdrawActionCall {
        pub withdraw: WithdrawData,
        pub event_id: u64,
    }
    ///Container type for all input parameters for the `feeManager` function with signature `feeManager()` and selector `0xd0fb0203`
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
    #[ethcall(name = "feeManager", abi = "feeManager()")]
    pub struct FeeManagerCall;
    ///Container type for all input parameters for the `getFrozenWithdrawNonce` function with signature `getFrozenWithdrawNonce(bytes32,uint64,bytes32)` and selector `0x782e97e3`
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
        name = "getFrozenWithdrawNonce",
        abi = "getFrozenWithdrawNonce(bytes32,uint64,bytes32)"
    )]
    pub struct GetFrozenWithdrawNonceCall {
        pub account_id: [u8; 32],
        pub withdraw_nonce: u64,
        pub token_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `globalDepositId` function with signature `globalDepositId()` and selector `0xaae2844b`
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
    #[ethcall(name = "globalDepositId", abi = "globalDepositId()")]
    pub struct GlobalDepositIdCall;
    ///Container type for all input parameters for the `globalEventId` function with signature `globalEventId()` and selector `0x08b85a26`
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
    #[ethcall(name = "globalEventId", abi = "globalEventId()")]
    pub struct GlobalEventIdCall;
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
    ///Container type for all input parameters for the `marketManager` function with signature `marketManager()` and selector `0x41ed2c12`
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
    #[ethcall(name = "marketManager", abi = "marketManager()")]
    pub struct MarketManagerCall;
    ///Container type for all input parameters for the `operatorManagerAddress` function with signature `operatorManagerAddress()` and selector `0x75bf9f6d`
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
    #[ethcall(name = "operatorManagerAddress", abi = "operatorManagerAddress()")]
    pub struct OperatorManagerAddressCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `rebalanceBurnFinish` function with signature `rebalanceBurnFinish((bool,uint64,uint128,bytes32,uint256,uint256))` and selector `0x97f8903e`
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
        name = "rebalanceBurnFinish",
        abi = "rebalanceBurnFinish((bool,uint64,uint128,bytes32,uint256,uint256))"
    )]
    pub struct RebalanceBurnFinishCall {
        pub data: RebalanceBurnCCFinishData,
    }
    ///Container type for all input parameters for the `rebalanceMintFinish` function with signature `rebalanceMintFinish((bool,uint64,uint128,bytes32,uint256,uint256))` and selector `0xefb556ed`
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
        name = "rebalanceMintFinish",
        abi = "rebalanceMintFinish((bool,uint64,uint128,bytes32,uint256,uint256))"
    )]
    pub struct RebalanceMintFinishCall {
        pub data: RebalanceMintCCFinishData,
    }
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setCrossChainManager` function with signature `setCrossChainManager(address)` and selector `0x5e1eb4ce`
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
    #[ethcall(name = "setCrossChainManager", abi = "setCrossChainManager(address)")]
    pub struct SetCrossChainManagerCall {
        pub cross_chain_manager_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setFeeManager` function with signature `setFeeManager(address)` and selector `0x472d35b9`
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
    #[ethcall(name = "setFeeManager", abi = "setFeeManager(address)")]
    pub struct SetFeeManagerCall {
        pub fee_manager_address: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `setOperatorManagerAddress` function with signature `setOperatorManagerAddress(address)` and selector `0xde0c9c86`
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
        name = "setOperatorManagerAddress",
        abi = "setOperatorManagerAddress(address)"
    )]
    pub struct SetOperatorManagerAddressCall {
        pub operator_manager_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setVaultManager` function with signature `setVaultManager(address)` and selector `0xb543503e`
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
    #[ethcall(name = "setVaultManager", abi = "setVaultManager(address)")]
    pub struct SetVaultManagerCall {
        pub vault_manager_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `vaultManager` function with signature `vaultManager()` and selector `0x8a4adf24`
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
    #[ethcall(name = "vaultManager", abi = "vaultManager()")]
    pub struct VaultManagerCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType)]
    pub enum user_ledgerCalls {
        AccountDeposit(AccountDepositCall),
        AccountWithDrawFinish(AccountWithDrawFinishCall),
        BatchGetUserLedger(BatchGetUserLedgerCall),
        BatchGetUserLedgerWithTokens(BatchGetUserLedgerWithTokensCall),
        CrossChainManagerAddress(CrossChainManagerAddressCall),
        ExecuteAdl(ExecuteAdlCall),
        ExecuteLiquidation(ExecuteLiquidationCall),
        ExecuteProcessValidatedFutures(ExecuteProcessValidatedFuturesCall),
        ExecuteRebalanceBurn(ExecuteRebalanceBurnCall),
        ExecuteRebalanceMint(ExecuteRebalanceMintCall),
        ExecuteSettlement(ExecuteSettlementCall),
        ExecuteWithdrawAction(ExecuteWithdrawActionCall),
        FeeManager(FeeManagerCall),
        GetFrozenWithdrawNonce(GetFrozenWithdrawNonceCall),
        GlobalDepositId(GlobalDepositIdCall),
        GlobalEventId(GlobalEventIdCall),
        Initialize(InitializeCall),
        MarketManager(MarketManagerCall),
        OperatorManagerAddress(OperatorManagerAddressCall),
        Owner(OwnerCall),
        RebalanceBurnFinish(RebalanceBurnFinishCall),
        RebalanceMintFinish(RebalanceMintFinishCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetCrossChainManager(SetCrossChainManagerCall),
        SetFeeManager(SetFeeManagerCall),
        SetMarketManager(SetMarketManagerCall),
        SetOperatorManagerAddress(SetOperatorManagerAddressCall),
        SetVaultManager(SetVaultManagerCall),
        TransferOwnership(TransferOwnershipCall),
        VaultManager(VaultManagerCall),
    }
    impl ::ethers::core::abi::AbiDecode for user_ledgerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AccountDepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccountDeposit(decoded));
            }
            if let Ok(decoded) = <AccountWithDrawFinishCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccountWithDrawFinish(decoded));
            }
            if let Ok(decoded) = <BatchGetUserLedgerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BatchGetUserLedger(decoded));
            }
            if let Ok(decoded) = <BatchGetUserLedgerWithTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BatchGetUserLedgerWithTokens(decoded));
            }
            if let Ok(decoded) = <CrossChainManagerAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CrossChainManagerAddress(decoded));
            }
            if let Ok(decoded) = <ExecuteAdlCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteAdl(decoded));
            }
            if let Ok(decoded) = <ExecuteLiquidationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteLiquidation(decoded));
            }
            if let Ok(decoded) = <ExecuteProcessValidatedFuturesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteProcessValidatedFutures(decoded));
            }
            if let Ok(decoded) = <ExecuteRebalanceBurnCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteRebalanceBurn(decoded));
            }
            if let Ok(decoded) = <ExecuteRebalanceMintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteRebalanceMint(decoded));
            }
            if let Ok(decoded) = <ExecuteSettlementCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteSettlement(decoded));
            }
            if let Ok(decoded) = <ExecuteWithdrawActionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteWithdrawAction(decoded));
            }
            if let Ok(decoded) = <FeeManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeManager(decoded));
            }
            if let Ok(decoded) = <GetFrozenWithdrawNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetFrozenWithdrawNonce(decoded));
            }
            if let Ok(decoded) = <GlobalDepositIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GlobalDepositId(decoded));
            }
            if let Ok(decoded) = <GlobalEventIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GlobalEventId(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <MarketManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MarketManager(decoded));
            }
            if let Ok(decoded) = <OperatorManagerAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorManagerAddress(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RebalanceBurnFinishCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RebalanceBurnFinish(decoded));
            }
            if let Ok(decoded) = <RebalanceMintFinishCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RebalanceMintFinish(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetCrossChainManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetCrossChainManager(decoded));
            }
            if let Ok(decoded) = <SetFeeManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetFeeManager(decoded));
            }
            if let Ok(decoded) = <SetMarketManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetMarketManager(decoded));
            }
            if let Ok(decoded) = <SetOperatorManagerAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetOperatorManagerAddress(decoded));
            }
            if let Ok(decoded) = <SetVaultManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetVaultManager(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <VaultManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VaultManager(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for user_ledgerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AccountDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccountWithDrawFinish(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BatchGetUserLedger(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BatchGetUserLedgerWithTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CrossChainManagerAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteAdl(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteLiquidation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteProcessValidatedFutures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteRebalanceBurn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteRebalanceMint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteSettlement(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteWithdrawAction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFrozenWithdrawNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GlobalDepositId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GlobalEventId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MarketManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorManagerAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RebalanceBurnFinish(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RebalanceMintFinish(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetCrossChainManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFeeManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMarketManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetOperatorManagerAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetVaultManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VaultManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for user_ledgerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccountDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccountWithDrawFinish(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BatchGetUserLedger(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BatchGetUserLedgerWithTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CrossChainManagerAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteAdl(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteLiquidation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteProcessValidatedFutures(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteRebalanceBurn(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteRebalanceMint(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteSettlement(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteWithdrawAction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFrozenWithdrawNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GlobalDepositId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GlobalEventId(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::MarketManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorManagerAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceBurnFinish(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RebalanceMintFinish(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCrossChainManager(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetFeeManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMarketManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOperatorManagerAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetVaultManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::VaultManager(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AccountDepositCall> for user_ledgerCalls {
        fn from(value: AccountDepositCall) -> Self {
            Self::AccountDeposit(value)
        }
    }
    impl ::core::convert::From<AccountWithDrawFinishCall> for user_ledgerCalls {
        fn from(value: AccountWithDrawFinishCall) -> Self {
            Self::AccountWithDrawFinish(value)
        }
    }
    impl ::core::convert::From<BatchGetUserLedgerCall> for user_ledgerCalls {
        fn from(value: BatchGetUserLedgerCall) -> Self {
            Self::BatchGetUserLedger(value)
        }
    }
    impl ::core::convert::From<BatchGetUserLedgerWithTokensCall> for user_ledgerCalls {
        fn from(value: BatchGetUserLedgerWithTokensCall) -> Self {
            Self::BatchGetUserLedgerWithTokens(value)
        }
    }
    impl ::core::convert::From<CrossChainManagerAddressCall> for user_ledgerCalls {
        fn from(value: CrossChainManagerAddressCall) -> Self {
            Self::CrossChainManagerAddress(value)
        }
    }
    impl ::core::convert::From<ExecuteAdlCall> for user_ledgerCalls {
        fn from(value: ExecuteAdlCall) -> Self {
            Self::ExecuteAdl(value)
        }
    }
    impl ::core::convert::From<ExecuteLiquidationCall> for user_ledgerCalls {
        fn from(value: ExecuteLiquidationCall) -> Self {
            Self::ExecuteLiquidation(value)
        }
    }
    impl ::core::convert::From<ExecuteProcessValidatedFuturesCall> for user_ledgerCalls {
        fn from(value: ExecuteProcessValidatedFuturesCall) -> Self {
            Self::ExecuteProcessValidatedFutures(value)
        }
    }
    impl ::core::convert::From<ExecuteRebalanceBurnCall> for user_ledgerCalls {
        fn from(value: ExecuteRebalanceBurnCall) -> Self {
            Self::ExecuteRebalanceBurn(value)
        }
    }
    impl ::core::convert::From<ExecuteRebalanceMintCall> for user_ledgerCalls {
        fn from(value: ExecuteRebalanceMintCall) -> Self {
            Self::ExecuteRebalanceMint(value)
        }
    }
    impl ::core::convert::From<ExecuteSettlementCall> for user_ledgerCalls {
        fn from(value: ExecuteSettlementCall) -> Self {
            Self::ExecuteSettlement(value)
        }
    }
    impl ::core::convert::From<ExecuteWithdrawActionCall> for user_ledgerCalls {
        fn from(value: ExecuteWithdrawActionCall) -> Self {
            Self::ExecuteWithdrawAction(value)
        }
    }
    impl ::core::convert::From<FeeManagerCall> for user_ledgerCalls {
        fn from(value: FeeManagerCall) -> Self {
            Self::FeeManager(value)
        }
    }
    impl ::core::convert::From<GetFrozenWithdrawNonceCall> for user_ledgerCalls {
        fn from(value: GetFrozenWithdrawNonceCall) -> Self {
            Self::GetFrozenWithdrawNonce(value)
        }
    }
    impl ::core::convert::From<GlobalDepositIdCall> for user_ledgerCalls {
        fn from(value: GlobalDepositIdCall) -> Self {
            Self::GlobalDepositId(value)
        }
    }
    impl ::core::convert::From<GlobalEventIdCall> for user_ledgerCalls {
        fn from(value: GlobalEventIdCall) -> Self {
            Self::GlobalEventId(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for user_ledgerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<MarketManagerCall> for user_ledgerCalls {
        fn from(value: MarketManagerCall) -> Self {
            Self::MarketManager(value)
        }
    }
    impl ::core::convert::From<OperatorManagerAddressCall> for user_ledgerCalls {
        fn from(value: OperatorManagerAddressCall) -> Self {
            Self::OperatorManagerAddress(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for user_ledgerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RebalanceBurnFinishCall> for user_ledgerCalls {
        fn from(value: RebalanceBurnFinishCall) -> Self {
            Self::RebalanceBurnFinish(value)
        }
    }
    impl ::core::convert::From<RebalanceMintFinishCall> for user_ledgerCalls {
        fn from(value: RebalanceMintFinishCall) -> Self {
            Self::RebalanceMintFinish(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for user_ledgerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetCrossChainManagerCall> for user_ledgerCalls {
        fn from(value: SetCrossChainManagerCall) -> Self {
            Self::SetCrossChainManager(value)
        }
    }
    impl ::core::convert::From<SetFeeManagerCall> for user_ledgerCalls {
        fn from(value: SetFeeManagerCall) -> Self {
            Self::SetFeeManager(value)
        }
    }
    impl ::core::convert::From<SetMarketManagerCall> for user_ledgerCalls {
        fn from(value: SetMarketManagerCall) -> Self {
            Self::SetMarketManager(value)
        }
    }
    impl ::core::convert::From<SetOperatorManagerAddressCall> for user_ledgerCalls {
        fn from(value: SetOperatorManagerAddressCall) -> Self {
            Self::SetOperatorManagerAddress(value)
        }
    }
    impl ::core::convert::From<SetVaultManagerCall> for user_ledgerCalls {
        fn from(value: SetVaultManagerCall) -> Self {
            Self::SetVaultManager(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for user_ledgerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<VaultManagerCall> for user_ledgerCalls {
        fn from(value: VaultManagerCall) -> Self {
            Self::VaultManager(value)
        }
    }
    ///Container type for all return fields from the `batchGetUserLedger` function with signature `batchGetUserLedger(bytes32[])` and selector `0x1757cb37`
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
    pub struct BatchGetUserLedgerReturn(pub ::std::vec::Vec<AccountSnapshot>);
    ///Container type for all return fields from the `batchGetUserLedger` function with signature `batchGetUserLedger(bytes32[],bytes32[],bytes32[])` and selector `0x5f225799`
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
    pub struct BatchGetUserLedgerWithTokensReturn {
        pub account_snapshots: ::std::vec::Vec<AccountSnapshot>,
    }
    ///Container type for all return fields from the `crossChainManagerAddress` function with signature `crossChainManagerAddress()` and selector `0xb182dc69`
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
    pub struct CrossChainManagerAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `feeManager` function with signature `feeManager()` and selector `0xd0fb0203`
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
    pub struct FeeManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getFrozenWithdrawNonce` function with signature `getFrozenWithdrawNonce(bytes32,uint64,bytes32)` and selector `0x782e97e3`
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
    pub struct GetFrozenWithdrawNonceReturn(pub u128);
    ///Container type for all return fields from the `globalDepositId` function with signature `globalDepositId()` and selector `0xaae2844b`
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
    pub struct GlobalDepositIdReturn(pub u64);
    ///Container type for all return fields from the `globalEventId` function with signature `globalEventId()` and selector `0x08b85a26`
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
    pub struct GlobalEventIdReturn(pub u64);
    ///Container type for all return fields from the `marketManager` function with signature `marketManager()` and selector `0x41ed2c12`
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
    pub struct MarketManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `operatorManagerAddress` function with signature `operatorManagerAddress()` and selector `0x75bf9f6d`
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
    pub struct OperatorManagerAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `vaultManager` function with signature `vaultManager()` and selector `0x8a4adf24`
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
    pub struct VaultManagerReturn(pub ::ethers::core::types::Address);
    ///`AccountDeposit(bytes32,bytes32,address,bytes32,uint256,uint128,uint64)`
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
    pub struct AccountDeposit {
        pub account_id: [u8; 32],
        pub broker_hash: [u8; 32],
        pub user_address: ::ethers::core::types::Address,
        pub token_hash: [u8; 32],
        pub src_chain_id: ::ethers::core::types::U256,
        pub token_amount: u128,
        pub src_chain_deposit_nonce: u64,
    }
    ///`AccountPerpPositions(bytes32,int128,int128,int128,uint128,uint128,uint128,int128,uint128)`
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
    pub struct AccountPerpPositions {
        pub symbol_hash: [u8; 32],
        pub position_qty: i128,
        pub cost_position: i128,
        pub last_sum_unitary_fundings: i128,
        pub last_executed_price: u128,
        pub last_settled_price: u128,
        pub average_entry_price: u128,
        pub opening_cost: i128,
        pub last_adl_price: u128,
    }
    ///`AccountSnapshot(bytes32,bytes32,address,uint64,uint64,uint64,uint64,(bytes32,uint128,uint128)[],(bytes32,int128,int128,int128,uint128,uint128,uint128,int128,uint128)[])`
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
    pub struct AccountSnapshot {
        pub account_id: [u8; 32],
        pub broker_hash: [u8; 32],
        pub user_address: ::ethers::core::types::Address,
        pub last_withdraw_nonce: u64,
        pub last_perp_trade_id: u64,
        pub last_engine_event_id: u64,
        pub last_deposit_event_id: u64,
        pub token_balances: ::std::vec::Vec<AccountTokenBalances>,
        pub perp_positions: ::std::vec::Vec<AccountPerpPositions>,
    }
    ///`AccountTokenBalances(bytes32,uint128,uint128)`
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
    pub struct AccountTokenBalances {
        pub token_hash: [u8; 32],
        pub balance: u128,
        pub frozen_balance: u128,
    }
    ///`AccountWithdraw(bytes32,address,address,bytes32,bytes32,uint128,uint128,uint256,uint64)`
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
    pub struct AccountWithdraw {
        pub account_id: [u8; 32],
        pub sender: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
        pub broker_hash: [u8; 32],
        pub token_hash: [u8; 32],
        pub token_amount: u128,
        pub fee: u128,
        pub chain_id: ::ethers::core::types::U256,
        pub withdraw_nonce: u64,
    }
    ///`Adl(bytes32,bytes32,bytes32,int128,int128,uint128,int128,uint64)`
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
    pub struct Adl {
        pub account_id: [u8; 32],
        pub insurance_account_id: [u8; 32],
        pub symbol_hash: [u8; 32],
        pub position_qty_transfer: i128,
        pub cost_position_transfer: i128,
        pub adl_price: u128,
        pub sum_unitary_fundings: i128,
        pub timestamp: u64,
    }
    ///`Liquidation(bytes32,bytes32,bytes32,uint128,uint64,(bytes32,bytes32,int128,int128,int128,int128,int128,uint128,int128,uint64)[])`
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
    pub struct Liquidation {
        pub liquidated_account_id: [u8; 32],
        pub insurance_account_id: [u8; 32],
        pub liquidated_asset_hash: [u8; 32],
        pub insurance_transfer_amount: u128,
        pub timestamp: u64,
        pub liquidation_transfers: ::std::vec::Vec<LiquidationTransfer>,
    }
    ///`LiquidationTransfer(bytes32,bytes32,int128,int128,int128,int128,int128,uint128,int128,uint64)`
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
    pub struct LiquidationTransfer {
        pub liquidator_account_id: [u8; 32],
        pub symbol_hash: [u8; 32],
        pub position_qty_transfer: i128,
        pub cost_position_transfer: i128,
        pub liquidator_fee: i128,
        pub insurance_fee: i128,
        pub liquidation_fee: i128,
        pub mark_price: u128,
        pub sum_unitary_fundings: i128,
        pub liquidation_transfer_id: u64,
    }
    ///`Settlement(bytes32,bytes32,bytes32,int128,uint128,uint64,(bytes32,uint128,int128,int128)[])`
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
    pub struct Settlement {
        pub account_id: [u8; 32],
        pub settled_asset_hash: [u8; 32],
        pub insurance_account_id: [u8; 32],
        pub settled_amount: i128,
        pub insurance_transfer_amount: u128,
        pub timestamp: u64,
        pub settlement_executions: ::std::vec::Vec<SettlementExecution>,
    }
    ///`SettlementExecution(bytes32,uint128,int128,int128)`
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
    pub struct SettlementExecution {
        pub symbol_hash: [u8; 32],
        pub mark_price: u128,
        pub sum_unitary_fundings: i128,
        pub settled_amount: i128,
    }
    ///`WithdrawData(uint128,uint128,uint256,bytes32,bytes32,bytes32,uint8,address,uint64,address,uint64,string,string)`
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
    pub struct WithdrawData {
        pub token_amount: u128,
        pub fee: u128,
        pub chain_id: ::ethers::core::types::U256,
        pub account_id: [u8; 32],
        pub r: [u8; 32],
        pub s: [u8; 32],
        pub v: u8,
        pub sender: ::ethers::core::types::Address,
        pub withdraw_nonce: u64,
        pub receiver: ::ethers::core::types::Address,
        pub timestamp: u64,
        pub broker_id: ::std::string::String,
        pub token_symbol: ::std::string::String,
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
    ///`RebalanceBurnCCFinishData(bool,uint64,uint128,bytes32,uint256,uint256)`
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
    pub struct RebalanceBurnCCFinishData {
        pub success: bool,
        pub rebalance_id: u64,
        pub amount: u128,
        pub token_hash: [u8; 32],
        pub burn_chain_id: ::ethers::core::types::U256,
        pub mint_chain_id: ::ethers::core::types::U256,
    }
    ///`RebalanceBurnUploadData(bytes32,bytes32,uint8,uint64,uint128,bytes32,uint256,uint256)`
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
    pub struct RebalanceBurnUploadData {
        pub r: [u8; 32],
        pub s: [u8; 32],
        pub v: u8,
        pub rebalance_id: u64,
        pub amount: u128,
        pub token_hash: [u8; 32],
        pub burn_chain_id: ::ethers::core::types::U256,
        pub mint_chain_id: ::ethers::core::types::U256,
    }
    ///`RebalanceMintCCFinishData(bool,uint64,uint128,bytes32,uint256,uint256)`
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
    pub struct RebalanceMintCCFinishData {
        pub success: bool,
        pub rebalance_id: u64,
        pub amount: u128,
        pub token_hash: [u8; 32],
        pub burn_chain_id: ::ethers::core::types::U256,
        pub mint_chain_id: ::ethers::core::types::U256,
    }
    ///`RebalanceMintUploadData(bytes32,bytes32,uint8,uint64,uint128,bytes32,uint256,uint256,bytes,bytes)`
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
    pub struct RebalanceMintUploadData {
        pub r: [u8; 32],
        pub s: [u8; 32],
        pub v: u8,
        pub rebalance_id: u64,
        pub amount: u128,
        pub token_hash: [u8; 32],
        pub burn_chain_id: ::ethers::core::types::U256,
        pub mint_chain_id: ::ethers::core::types::U256,
        pub message_bytes: ::ethers::core::types::Bytes,
        pub message_signature: ::ethers::core::types::Bytes,
    }
}

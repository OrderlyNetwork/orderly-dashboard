pub use user_ledger::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod user_ledger {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
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
                    ::std::borrow::ToOwned::to_owned("accountDepositSol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("accountDepositSol"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct AccountTypes.AccountDepositSol",
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
                    ::std::borrow::ToOwned::to_owned("accountWithdrawFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "accountWithdrawFail",
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
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
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
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
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
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
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
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
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
                    ::std::borrow::ToOwned::to_owned("executeAdlV2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeAdlV2"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("adl"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct EventTypes.AdlV2"),
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
                    ::std::borrow::ToOwned::to_owned("executeBalanceTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeBalanceTransfer",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balanceTransfer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct EventTypes.BalanceTransfer",
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
                    ::std::borrow::ToOwned::to_owned("executeDelegateSigner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeDelegateSigner",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delegateSigner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct EventTypes.DelegateSigner",
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
                    ::std::borrow::ToOwned::to_owned("executeDelegateWithdrawAction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeDelegateWithdrawAction",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delegateWithdraw"),
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
                    ::std::borrow::ToOwned::to_owned("executeFeeDistribution"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeFeeDistribution",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeDistribution"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct EventTypes.FeeDistribution",
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
                    ::std::borrow::ToOwned::to_owned("executeLiquidationV2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeLiquidationV2",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct EventTypes.LiquidationV2",
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
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
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
                    ::std::borrow::ToOwned::to_owned(
                        "executeProcessValidatedFuturesBatch",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeProcessValidatedFuturesBatch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("trades"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct PerpTypes.FuturesTradeUpload[]",
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
                                    name: ::std::borrow::ToOwned::to_owned("ledger"),
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
                    ::std::borrow::ToOwned::to_owned("executeSwapResultUpload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeSwapResultUpload",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapResultUpload"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct EventTypes.SwapResult",
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
                    ::std::borrow::ToOwned::to_owned("executeWithdraw2Contract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeWithdraw2Contract",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct EventTypes.Withdraw2Contract",
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
                    ::std::borrow::ToOwned::to_owned("executeWithdrawSolAction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeWithdrawSolAction",
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
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct EventTypes.WithdrawDataSol",
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
                    ::std::borrow::ToOwned::to_owned("getBalanceTransferState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getBalanceTransferState",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("transferId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct EventTypes.InternalTransferTrack",
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
                    ::std::borrow::ToOwned::to_owned("getUserEscrowBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getUserEscrowBalance",
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
                    ::std::borrow::ToOwned::to_owned("getUserTokenBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getUserTokenBalance",
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getUserTotalFrozenBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getUserTotalFrozenBalance",
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
                    ::std::borrow::ToOwned::to_owned("setCrossChainManagerV2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setCrossChainManagerV2",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_crossChainManagerV2Address",
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
                    ::std::borrow::ToOwned::to_owned("setLedgerImplA"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setLedgerImplA"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ledgerImplA"),
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
                    ::std::borrow::ToOwned::to_owned("setLedgerImplB"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setLedgerImplB"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ledgerImplB"),
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
                    ::std::borrow::ToOwned::to_owned("setLedgerImplC"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setLedgerImplC"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ledgerImplC"),
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
                    ::std::borrow::ToOwned::to_owned("setLedgerImplD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setLedgerImplD"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ledgerImplD"),
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
                    ::std::borrow::ToOwned::to_owned("AccountDepositSol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AccountDepositSol"),
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
                                    name: ::std::borrow::ToOwned::to_owned("pubkey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
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
                                    name: ::std::borrow::ToOwned::to_owned("pubkey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("AccountWithdraw2Contract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccountWithdraw2Contract",
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
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                                    name: ::std::borrow::ToOwned::to_owned("failReason"),
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
                                    name: ::std::borrow::ToOwned::to_owned("failReason"),
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
                    ::std::borrow::ToOwned::to_owned("AccountWithdrawSolApprove"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccountWithdrawSolApprove",
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AccountWithdrawSolFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccountWithdrawSolFail",
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned("failReason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
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
                    ::std::borrow::ToOwned::to_owned("AdlResultV2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AdlResultV2"),
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
                    ::std::borrow::ToOwned::to_owned("BalanceTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("BalanceTransfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("transferId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fromAccountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("toAccountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
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
                                    name: ::std::borrow::ToOwned::to_owned("isFromAccountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("transferType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
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
                    ::std::borrow::ToOwned::to_owned("ChangeCrossChainManagerV2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ChangeCrossChainManagerV2",
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
                    ::std::borrow::ToOwned::to_owned("ChangeLedgerImplA"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChangeLedgerImplA"),
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
                    ::std::borrow::ToOwned::to_owned("ChangeLedgerImplB"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChangeLedgerImplB"),
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
                    ::std::borrow::ToOwned::to_owned("ChangeLedgerImplC"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChangeLedgerImplC"),
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
                    ::std::borrow::ToOwned::to_owned("ChangeLedgerImplD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChangeLedgerImplD"),
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
                    ::std::borrow::ToOwned::to_owned("DelegateSigner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DelegateSigner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned("delegateContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                                    name: ::std::borrow::ToOwned::to_owned("delegateSigner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeeDistribution"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FeeDistribution"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fromAccountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("toAccountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
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
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InternalTransferFinalised"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InternalTransferFinalised",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("transferId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("toAccountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
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
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("LiquidationResultV2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LiquidationResultV2",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("LiquidationTransferV2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LiquidationTransferV2",
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
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
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
                    ::std::borrow::ToOwned::to_owned("PrimeWalletSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PrimeWalletSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("primeWallet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
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
                (
                    ::std::borrow::ToOwned::to_owned("SwapResultUploaded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SwapResultUploaded"),
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
                                    name: ::std::borrow::ToOwned::to_owned("buyTokenHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sellTokenHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("buyQuantity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sellQuantity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
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
                                    name: ::std::borrow::ToOwned::to_owned("swapStatus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
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
                    ::std::borrow::ToOwned::to_owned("Bytes32Zero"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Bytes32Zero"),
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
                    ::std::borrow::ToOwned::to_owned("DelegateChainIdNotMatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DelegateChainIdNotMatch",
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
                                    name: ::std::borrow::ToOwned::to_owned("savedChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("givenChainId"),
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
                    ::std::borrow::ToOwned::to_owned("DelegateReceiverNotMatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DelegateReceiverNotMatch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delegateContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DelegateSignerNotMatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DelegateSignerNotMatch",
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
                                    name: ::std::borrow::ToOwned::to_owned("savedSginer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("givenSigner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DelegatecallFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("DelegatecallFail"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnumerableSetError"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EnumerableSetError"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidFeeCollectorType"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidFeeCollectorType",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidPrimeWallet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidPrimeWallet"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LedgerAddressZero"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("LedgerAddressZero"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotImplemented"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotImplemented"),
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
                    ::std::borrow::ToOwned::to_owned("OnlyCrossChainManagerV2CanCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OnlyCrossChainManagerV2CanCall",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyLedgerCanCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OnlyLedgerCanCall"),
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
                    ::std::borrow::ToOwned::to_owned("OnlyOperatorManagerCanCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OnlyOperatorManagerCanCall",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlySymbolManagerOrOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OnlySymbolManagerOrOwner",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorManagerAddressZero"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OperatorManagerAddressZero",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProtocolVaultAddressMismatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ProtocolVaultAddressMismatch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("want"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("got"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RebalanceAlreadySucc"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RebalanceAlreadySucc",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RebalanceChainIdInvalid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RebalanceChainIdInvalid",
                            ),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("RebalanceIdNotMatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RebalanceIdNotMatch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("givenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wantId"),
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
                    ::std::borrow::ToOwned::to_owned("RebalanceMintUnexpected"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RebalanceMintUnexpected",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RebalanceStillPending"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RebalanceStillPending",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RebalanceTokenNotSupported"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RebalanceTokenNotSupported",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("SymbolNotRegister"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SymbolNotRegister"),
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
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawBalanceNotEnough"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WithdrawBalanceNotEnough",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawEscrowBalanceNotEnough"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WithdrawEscrowBalanceNotEnough",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("availableBalance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawFeeTooLarge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WithdrawFeeTooLarge",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawToAddressZero"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WithdrawToAddressZero",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawVaultBalanceNotEnough"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WithdrawVaultBalanceNotEnough",
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
                                    name: ::std::borrow::ToOwned::to_owned("withdrawAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroChainId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ZeroChainId"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroDelegateContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ZeroDelegateContract",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroDelegateSigner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ZeroDelegateSigner"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static USER_LEDGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
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
            Self(::ethers::contract::Contract::new(
                address.into(),
                USER_LEDGER_ABI.clone(),
                client,
            ))
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
        ///Calls the contract's `accountDepositSol` (0xe4ea46aa) function
        pub fn account_deposit_sol(
            &self,
            data: AccountDepositSol,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 234, 70, 170], (data,))
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
        ///Calls the contract's `accountWithdrawFail` (0x26acf6e1) function
        pub fn account_withdraw_fail(
            &self,
            withdraw: AccountWithdraw,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 172, 246, 225], (withdraw,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `batchGetUserLedger` (0x1757cb37) function
        pub fn batch_get_user_ledger(
            &self,
            account_ids: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<AccountSnapshot>>
        {
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<AccountSnapshot>>
        {
            self.0
                .method_hash([95, 34, 87, 153], (account_ids, tokens, symbols))
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
        ///Calls the contract's `executeAdlV2` (0xf97a259c) function
        pub fn execute_adl_v2(
            &self,
            adl: AdlV2,
            event_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 122, 37, 156], (adl, event_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeBalanceTransfer` (0xf83bd887) function
        pub fn execute_balance_transfer(
            &self,
            balance_transfer: BalanceTransfer,
            event_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 59, 216, 135], (balance_transfer, event_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeDelegateSigner` (0x0997c228) function
        pub fn execute_delegate_signer(
            &self,
            delegate_signer: DelegateSigner,
            event_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 151, 194, 40], (delegate_signer, event_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeDelegateWithdrawAction` (0xec0a14aa) function
        pub fn execute_delegate_withdraw_action(
            &self,
            delegate_withdraw: WithdrawData,
            event_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([236, 10, 20, 170], (delegate_withdraw, event_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeFeeDistribution` (0x9078ffd8) function
        pub fn execute_fee_distribution(
            &self,
            fee_distribution: FeeDistribution,
            event_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([144, 120, 255, 216], (fee_distribution, event_id))
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
        ///Calls the contract's `executeLiquidationV2` (0xb8375d1f) function
        pub fn execute_liquidation_v2(
            &self,
            liquidation: LiquidationV2,
            event_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 55, 93, 31], (liquidation, event_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeProcessValidatedFutures` (0x0b16ebe4) function
        pub fn execute_process_validated_futures(
            &self,
            trade: FuturesTradeUpload,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 22, 235, 228], (trade,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeProcessValidatedFuturesBatch` (0xf8a1018f) function
        pub fn execute_process_validated_futures_batch(
            &self,
            trades: ::std::vec::Vec<FuturesTradeUpload>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 161, 1, 143], trades)
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
            ledger: Settlement,
            event_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 108, 59, 213], (ledger, event_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeSwapResultUpload` (0xae5f766e) function
        pub fn execute_swap_result_upload(
            &self,
            swap_result_upload: SwapResult,
            event_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 95, 118, 110], (swap_result_upload, event_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeWithdraw2Contract` (0xa71e351f) function
        pub fn execute_withdraw_2_contract(
            &self,
            data: Withdraw2Contract,
            event_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 30, 53, 31], (data, event_id))
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
        ///Calls the contract's `executeWithdrawSolAction` (0xd2050cb5) function
        pub fn execute_withdraw_sol_action(
            &self,
            withdraw: WithdrawDataSol,
            event_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 5, 12, 181], (withdraw, event_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBalanceTransferState` (0x55b39141) function
        pub fn get_balance_transfer_state(
            &self,
            transfer_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, InternalTransferTrack> {
            self.0
                .method_hash([85, 179, 145, 65], transfer_id)
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
        ///Calls the contract's `getUserEscrowBalance` (0xa9d31363) function
        pub fn get_user_escrow_balance(
            &self,
            account_id: [u8; 32],
            token_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([169, 211, 19, 99], (account_id, token_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUserTokenBalance` (0x69732a98) function
        pub fn get_user_token_balance(
            &self,
            account_id: [u8; 32],
            token_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([105, 115, 42, 152], (account_id, token_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUserTotalFrozenBalance` (0x9d2aff5f) function
        pub fn get_user_total_frozen_balance(
            &self,
            account_id: [u8; 32],
            token_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([157, 42, 255, 95], (account_id, token_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x8129fc1c) function
        pub fn initialize(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
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
        ///Calls the contract's `setCrossChainManager` (0x5e1eb4ce) function
        pub fn set_cross_chain_manager(
            &self,
            cross_chain_manager_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 30, 180, 206], cross_chain_manager_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setCrossChainManagerV2` (0xdb0033ae) function
        pub fn set_cross_chain_manager_v2(
            &self,
            cross_chain_manager_v2_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 0, 51, 174], cross_chain_manager_v2_address)
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
        ///Calls the contract's `setLedgerImplA` (0x25746836) function
        pub fn set_ledger_impl_a(
            &self,
            ledger_impl_a: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 116, 104, 54], ledger_impl_a)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLedgerImplB` (0xf10fbcb4) function
        pub fn set_ledger_impl_b(
            &self,
            ledger_impl_b: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 15, 188, 180], ledger_impl_b)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLedgerImplC` (0x0f8024a8) function
        pub fn set_ledger_impl_c(
            &self,
            ledger_impl_c: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 128, 36, 168], ledger_impl_c)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLedgerImplD` (0xf5ae6138) function
        pub fn set_ledger_impl_d(
            &self,
            ledger_impl_d: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 174, 97, 56], ledger_impl_d)
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
        ///Gets the contract's `AccountDeposit` event
        pub fn account_deposit_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AccountDeposit1Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `AccountDeposit` event
        pub fn account_deposit_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AccountDeposit2Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `AccountDepositSol` event
        pub fn account_deposit_sol_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AccountDepositSolFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `AccountRegister` event
        pub fn account_register_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AccountRegister1Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `AccountRegister` event
        pub fn account_register_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AccountRegister2Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `AccountRegister` event
        pub fn account_register_3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AccountRegister3Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `AccountWithdraw2Contract` event
        pub fn account_withdraw_2_contract_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountWithdraw2ContractFilter,
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AccountWithdrawFail1Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `AccountWithdrawFail` event
        pub fn account_withdraw_fail_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AccountWithdrawFail2Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `AccountWithdrawFinish` event
        pub fn account_withdraw_finish_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AccountWithdrawFinish1Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `AccountWithdrawFinish` event
        pub fn account_withdraw_finish_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AccountWithdrawFinish2Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `AccountWithdrawSolApprove` event
        pub fn account_withdraw_sol_approve_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountWithdrawSolApproveFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AccountWithdrawSolFail` event
        pub fn account_withdraw_sol_fail_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AccountWithdrawSolFailFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `AdlResult` event
        pub fn adl_result_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AdlResultFilter> {
            self.0.event()
        }
        ///Gets the contract's `AdlResultV2` event
        pub fn adl_result_v2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AdlResultV2Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `BalanceTransfer` event
        pub fn balance_transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BalanceTransferFilter>
        {
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
        ///Gets the contract's `ChangeCrossChainManagerV2` event
        pub fn change_cross_chain_manager_v2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangeCrossChainManagerV2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangeFeeManager` event
        pub fn change_fee_manager_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ChangeFeeManagerFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ChangeLedgerImplA` event
        pub fn change_ledger_impl_a_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ChangeLedgerImplAFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ChangeLedgerImplB` event
        pub fn change_ledger_impl_b_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ChangeLedgerImplBFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ChangeLedgerImplC` event
        pub fn change_ledger_impl_c_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ChangeLedgerImplCFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ChangeLedgerImplD` event
        pub fn change_ledger_impl_d_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ChangeLedgerImplDFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ChangeMarketManager` event
        pub fn change_market_manager_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ChangeMarketManagerFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ChangeOperatorManager` event
        pub fn change_operator_manager_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ChangeOperatorManagerFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ChangeVaultManager` event
        pub fn change_vault_manager_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ChangeVaultManagerFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `DelegateSigner` event
        pub fn delegate_signer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DelegateSignerFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `FeeDistribution` event
        pub fn fee_distribution_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FeeDistributionFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `InternalTransferFinalised` event
        pub fn internal_transfer_finalised_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InternalTransferFinalisedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LiquidationResult` event
        pub fn liquidation_result_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LiquidationResultFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `LiquidationResultV2` event
        pub fn liquidation_result_v2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LiquidationResultV2Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `LiquidationTransfer` event
        pub fn liquidation_transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LiquidationTransferFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `LiquidationTransferV2` event
        pub fn liquidation_transfer_v2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LiquidationTransferV2Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `PrimeWalletSet` event
        pub fn prime_wallet_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PrimeWalletSetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ProcessValidatedFutures` event
        pub fn process_validated_futures_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProcessValidatedFutures1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProcessValidatedFutures` event
        pub fn process_validated_futures_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProcessValidatedFutures2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SettlementExecution` event
        pub fn settlement_execution_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SettlementExecutionFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SettlementResult` event
        pub fn settlement_result_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SettlementResultFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SwapResultUploaded` event
        pub fn swap_result_uploaded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapResultUploadedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, user_ledgerEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for user_ledger<M> {
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[etherror(name = "BrokerNotAllowed", abi = "BrokerNotAllowed()")]
    pub struct BrokerNotAllowed;
    ///Custom Error type `Bytes32Zero` with signature `Bytes32Zero()` and selector `0x96b1f227`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Bytes32Zero", abi = "Bytes32Zero()")]
    pub struct Bytes32Zero;
    ///Custom Error type `CountNotMatch` with signature `CountNotMatch(uint256,uint256)` and selector `0x75ec4356`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "CountNotMatch", abi = "CountNotMatch(uint256,uint256)")]
    pub struct CountNotMatch {
        pub length: ::ethers::core::types::U256,
        pub count: ::ethers::core::types::U256,
    }
    ///Custom Error type `DelegateChainIdNotMatch` with signature `DelegateChainIdNotMatch(bytes32,uint256,uint256)` and selector `0x769e69bd`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "DelegateChainIdNotMatch",
        abi = "DelegateChainIdNotMatch(bytes32,uint256,uint256)"
    )]
    pub struct DelegateChainIdNotMatch {
        pub account_id: [u8; 32],
        pub saved_chain_id: ::ethers::core::types::U256,
        pub given_chain_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `DelegateReceiverNotMatch` with signature `DelegateReceiverNotMatch(address,address)` and selector `0xe7e2ed65`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "DelegateReceiverNotMatch",
        abi = "DelegateReceiverNotMatch(address,address)"
    )]
    pub struct DelegateReceiverNotMatch {
        pub receiver: ::ethers::core::types::Address,
        pub delegate_contract: ::ethers::core::types::Address,
    }
    ///Custom Error type `DelegateSignerNotMatch` with signature `DelegateSignerNotMatch(bytes32,address,address)` and selector `0xd9b8908e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "DelegateSignerNotMatch",
        abi = "DelegateSignerNotMatch(bytes32,address,address)"
    )]
    pub struct DelegateSignerNotMatch {
        pub account_id: [u8; 32],
        pub saved_sginer: ::ethers::core::types::Address,
        pub given_signer: ::ethers::core::types::Address,
    }
    ///Custom Error type `DelegatecallFail` with signature `DelegatecallFail()` and selector `0x68c40820`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "DelegatecallFail", abi = "DelegatecallFail()")]
    pub struct DelegatecallFail;
    ///Custom Error type `EnumerableSetError` with signature `EnumerableSetError()` and selector `0xa65b249b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "EnumerableSetError", abi = "EnumerableSetError()")]
    pub struct EnumerableSetError;
    ///Custom Error type `FrozenBalanceInconsistent` with signature `FrozenBalanceInconsistent()` and selector `0xdc6db21a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "FrozenBalanceInconsistent",
        abi = "FrozenBalanceInconsistent()"
    )]
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
        Hash,
    )]
    #[etherror(
        name = "InsurancePositionQtyInvalid",
        abi = "InsurancePositionQtyInvalid(int128,int128)"
    )]
    pub struct InsurancePositionQtyInvalid {
        pub adl_position_qty_transfer: i128,
        pub user_position_qty: i128,
    }
    ///Custom Error type `InsuranceTransferAmountInvalid` with signature `InsuranceTransferAmountInvalid(int128,uint128,int128)` and selector `0x13705c39`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "InsuranceTransferAmountInvalid",
        abi = "InsuranceTransferAmountInvalid(int128,uint128,int128)"
    )]
    pub struct InsuranceTransferAmountInvalid {
        pub balance: i128,
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
        Hash,
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
        Hash,
    )]
    #[etherror(name = "InvalidBizType", abi = "InvalidBizType(uint8)")]
    pub struct InvalidBizType {
        pub biz_type: u8,
    }
    ///Custom Error type `InvalidFeeCollectorType` with signature `InvalidFeeCollectorType()` and selector `0xb6bd80ad`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidFeeCollectorType", abi = "InvalidFeeCollectorType()")]
    pub struct InvalidFeeCollectorType;
    ///Custom Error type `InvalidPrimeWallet` with signature `InvalidPrimeWallet()` and selector `0xa6d1520a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidPrimeWallet", abi = "InvalidPrimeWallet()")]
    pub struct InvalidPrimeWallet;
    ///Custom Error type `LedgerAddressZero` with signature `LedgerAddressZero()` and selector `0x26ef1e88`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "LedgerAddressZero", abi = "LedgerAddressZero()")]
    pub struct LedgerAddressZero;
    ///Custom Error type `NotImplemented` with signature `NotImplemented()` and selector `0xd6234725`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NotImplemented", abi = "NotImplemented()")]
    pub struct NotImplemented;
    ///Custom Error type `OnlyCrossChainManagerCanCall` with signature `OnlyCrossChainManagerCanCall()` and selector `0x833d33e7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "OnlyCrossChainManagerCanCall",
        abi = "OnlyCrossChainManagerCanCall()"
    )]
    pub struct OnlyCrossChainManagerCanCall;
    ///Custom Error type `OnlyCrossChainManagerV2CanCall` with signature `OnlyCrossChainManagerV2CanCall()` and selector `0x44167c74`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "OnlyCrossChainManagerV2CanCall",
        abi = "OnlyCrossChainManagerV2CanCall()"
    )]
    pub struct OnlyCrossChainManagerV2CanCall;
    ///Custom Error type `OnlyLedgerCanCall` with signature `OnlyLedgerCanCall()` and selector `0x4842bd3f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "OnlyLedgerCanCall", abi = "OnlyLedgerCanCall()")]
    pub struct OnlyLedgerCanCall;
    ///Custom Error type `OnlyOperatorCanCall` with signature `OnlyOperatorCanCall()` and selector `0x515b6e4b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "OnlyOperatorCanCall", abi = "OnlyOperatorCanCall()")]
    pub struct OnlyOperatorCanCall;
    ///Custom Error type `OnlyOperatorManagerCanCall` with signature `OnlyOperatorManagerCanCall()` and selector `0xf03c884e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "OnlyOperatorManagerCanCall",
        abi = "OnlyOperatorManagerCanCall()"
    )]
    pub struct OnlyOperatorManagerCanCall;
    ///Custom Error type `OnlySymbolManagerOrOwner` with signature `OnlySymbolManagerOrOwner()` and selector `0x7293634c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "OnlySymbolManagerOrOwner", abi = "OnlySymbolManagerOrOwner()")]
    pub struct OnlySymbolManagerOrOwner;
    ///Custom Error type `OperatorManagerAddressZero` with signature `OperatorManagerAddressZero()` and selector `0x0886f3e1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "OperatorManagerAddressZero",
        abi = "OperatorManagerAddressZero()"
    )]
    pub struct OperatorManagerAddressZero;
    ///Custom Error type `ProtocolVaultAddressMismatch` with signature `ProtocolVaultAddressMismatch(address,address)` and selector `0x721c48f2`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "ProtocolVaultAddressMismatch",
        abi = "ProtocolVaultAddressMismatch(address,address)"
    )]
    pub struct ProtocolVaultAddressMismatch {
        pub want: ::ethers::core::types::Address,
        pub got: ::ethers::core::types::Address,
    }
    ///Custom Error type `RebalanceAlreadySucc` with signature `RebalanceAlreadySucc()` and selector `0x93a12797`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "RebalanceAlreadySucc", abi = "RebalanceAlreadySucc()")]
    pub struct RebalanceAlreadySucc;
    ///Custom Error type `RebalanceChainIdInvalid` with signature `RebalanceChainIdInvalid(uint256)` and selector `0x94f34d5d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "RebalanceChainIdInvalid",
        abi = "RebalanceChainIdInvalid(uint256)"
    )]
    pub struct RebalanceChainIdInvalid {
        pub chain_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `RebalanceIdNotMatch` with signature `RebalanceIdNotMatch(uint64,uint64)` and selector `0xa2204a01`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "RebalanceIdNotMatch",
        abi = "RebalanceIdNotMatch(uint64,uint64)"
    )]
    pub struct RebalanceIdNotMatch {
        pub given_id: u64,
        pub want_id: u64,
    }
    ///Custom Error type `RebalanceMintUnexpected` with signature `RebalanceMintUnexpected()` and selector `0x88bf1f03`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "RebalanceMintUnexpected", abi = "RebalanceMintUnexpected()")]
    pub struct RebalanceMintUnexpected;
    ///Custom Error type `RebalanceStillPending` with signature `RebalanceStillPending()` and selector `0x0fecfbdc`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "RebalanceStillPending", abi = "RebalanceStillPending()")]
    pub struct RebalanceStillPending;
    ///Custom Error type `RebalanceTokenNotSupported` with signature `RebalanceTokenNotSupported(bytes32,uint256)` and selector `0x1fca913c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "RebalanceTokenNotSupported",
        abi = "RebalanceTokenNotSupported(bytes32,uint256)"
    )]
    pub struct RebalanceTokenNotSupported {
        pub token_hash: [u8; 32],
        pub chain_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `SafeCastOverflow` with signature `SafeCastOverflow()` and selector `0x93dafdf1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Hash,
    )]
    #[etherror(name = "SafeCastUnderflow", abi = "SafeCastUnderflow()")]
    pub struct SafeCastUnderflow;
    ///Custom Error type `SignatureNotMatch` with signature `SignatureNotMatch()` and selector `0x0510cc45`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Hash,
    )]
    #[etherror(name = "SymbolNotAllowed", abi = "SymbolNotAllowed()")]
    pub struct SymbolNotAllowed;
    ///Custom Error type `SymbolNotRegister` with signature `SymbolNotRegister()` and selector `0xea160191`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "SymbolNotRegister", abi = "SymbolNotRegister()")]
    pub struct SymbolNotRegister;
    ///Custom Error type `TokenNotAllowed` with signature `TokenNotAllowed(bytes32,uint256)` and selector `0x7c334f8b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[etherror(
        name = "UserPerpPositionQtyZero",
        abi = "UserPerpPositionQtyZero(bytes32,bytes32)"
    )]
    pub struct UserPerpPositionQtyZero {
        pub account_id: [u8; 32],
        pub symbol_hash: [u8; 32],
    }
    ///Custom Error type `WithdrawBalanceNotEnough` with signature `WithdrawBalanceNotEnough(int128,uint128)` and selector `0xd8c06800`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "WithdrawBalanceNotEnough",
        abi = "WithdrawBalanceNotEnough(int128,uint128)"
    )]
    pub struct WithdrawBalanceNotEnough {
        pub balance: i128,
        pub withdraw_amount: u128,
    }
    ///Custom Error type `WithdrawEscrowBalanceNotEnough` with signature `WithdrawEscrowBalanceNotEnough(int128,uint128)` and selector `0x700f5d9f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "WithdrawEscrowBalanceNotEnough",
        abi = "WithdrawEscrowBalanceNotEnough(int128,uint128)"
    )]
    pub struct WithdrawEscrowBalanceNotEnough {
        pub available_balance: i128,
        pub withdraw_amount: u128,
    }
    ///Custom Error type `WithdrawFeeTooLarge` with signature `WithdrawFeeTooLarge(uint128,uint128)` and selector `0x2595b4f7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "WithdrawFeeTooLarge",
        abi = "WithdrawFeeTooLarge(uint128,uint128)"
    )]
    pub struct WithdrawFeeTooLarge {
        pub max_fee: u128,
        pub withdraw_fee: u128,
    }
    ///Custom Error type `WithdrawToAddressZero` with signature `WithdrawToAddressZero()` and selector `0x3f9474eb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "WithdrawToAddressZero", abi = "WithdrawToAddressZero()")]
    pub struct WithdrawToAddressZero;
    ///Custom Error type `WithdrawVaultBalanceNotEnough` with signature `WithdrawVaultBalanceNotEnough(uint128,uint128)` and selector `0x7911c14a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "WithdrawVaultBalanceNotEnough",
        abi = "WithdrawVaultBalanceNotEnough(uint128,uint128)"
    )]
    pub struct WithdrawVaultBalanceNotEnough {
        pub balance: u128,
        pub withdraw_amount: u128,
    }
    ///Custom Error type `ZeroChainId` with signature `ZeroChainId()` and selector `0xc84885d4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ZeroChainId", abi = "ZeroChainId()")]
    pub struct ZeroChainId;
    ///Custom Error type `ZeroDelegateContract` with signature `ZeroDelegateContract()` and selector `0xee9f3445`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ZeroDelegateContract", abi = "ZeroDelegateContract()")]
    pub struct ZeroDelegateContract;
    ///Custom Error type `ZeroDelegateSigner` with signature `ZeroDelegateSigner()` and selector `0xe06c2a14`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ZeroDelegateSigner", abi = "ZeroDelegateSigner()")]
    pub struct ZeroDelegateSigner;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum user_ledgerErrors {
        AccountIdInvalid(AccountIdInvalid),
        AddressZero(AddressZero),
        BalanceNotEnough(BalanceNotEnough),
        BatchIdNotMatch(BatchIdNotMatch),
        BrokerNotAllowed(BrokerNotAllowed),
        Bytes32Zero(Bytes32Zero),
        CountNotMatch(CountNotMatch),
        DelegateChainIdNotMatch(DelegateChainIdNotMatch),
        DelegateReceiverNotMatch(DelegateReceiverNotMatch),
        DelegateSignerNotMatch(DelegateSignerNotMatch),
        DelegatecallFail(DelegatecallFail),
        EnumerableSetError(EnumerableSetError),
        FrozenBalanceInconsistent(FrozenBalanceInconsistent),
        InsurancePositionQtyInvalid(InsurancePositionQtyInvalid),
        InsuranceTransferAmountInvalid(InsuranceTransferAmountInvalid),
        InsuranceTransferToSelf(InsuranceTransferToSelf),
        InvalidBizType(InvalidBizType),
        InvalidFeeCollectorType(InvalidFeeCollectorType),
        InvalidPrimeWallet(InvalidPrimeWallet),
        LedgerAddressZero(LedgerAddressZero),
        NotImplemented(NotImplemented),
        OnlyCrossChainManagerCanCall(OnlyCrossChainManagerCanCall),
        OnlyCrossChainManagerV2CanCall(OnlyCrossChainManagerV2CanCall),
        OnlyLedgerCanCall(OnlyLedgerCanCall),
        OnlyOperatorCanCall(OnlyOperatorCanCall),
        OnlyOperatorManagerCanCall(OnlyOperatorManagerCanCall),
        OnlySymbolManagerOrOwner(OnlySymbolManagerOrOwner),
        OperatorManagerAddressZero(OperatorManagerAddressZero),
        ProtocolVaultAddressMismatch(ProtocolVaultAddressMismatch),
        RebalanceAlreadySucc(RebalanceAlreadySucc),
        RebalanceChainIdInvalid(RebalanceChainIdInvalid),
        RebalanceIdNotMatch(RebalanceIdNotMatch),
        RebalanceMintUnexpected(RebalanceMintUnexpected),
        RebalanceStillPending(RebalanceStillPending),
        RebalanceTokenNotSupported(RebalanceTokenNotSupported),
        SafeCastOverflow(SafeCastOverflow),
        SafeCastUnderflow(SafeCastUnderflow),
        SignatureNotMatch(SignatureNotMatch),
        SymbolNotAllowed(SymbolNotAllowed),
        SymbolNotRegister(SymbolNotRegister),
        TokenNotAllowed(TokenNotAllowed),
        TotalSettleAmountNotMatch(TotalSettleAmountNotMatch),
        UserPerpPositionQtyZero(UserPerpPositionQtyZero),
        WithdrawBalanceNotEnough(WithdrawBalanceNotEnough),
        WithdrawEscrowBalanceNotEnough(WithdrawEscrowBalanceNotEnough),
        WithdrawFeeTooLarge(WithdrawFeeTooLarge),
        WithdrawToAddressZero(WithdrawToAddressZero),
        WithdrawVaultBalanceNotEnough(WithdrawVaultBalanceNotEnough),
        ZeroChainId(ZeroChainId),
        ZeroDelegateContract(ZeroDelegateContract),
        ZeroDelegateSigner(ZeroDelegateSigner),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for user_ledgerErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AccountIdInvalid as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AccountIdInvalid(decoded));
            }
            if let Ok(decoded) = <AddressZero as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddressZero(decoded));
            }
            if let Ok(decoded) = <BalanceNotEnough as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BalanceNotEnough(decoded));
            }
            if let Ok(decoded) = <BatchIdNotMatch as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BatchIdNotMatch(decoded));
            }
            if let Ok(decoded) = <BrokerNotAllowed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BrokerNotAllowed(decoded));
            }
            if let Ok(decoded) = <Bytes32Zero as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Bytes32Zero(decoded));
            }
            if let Ok(decoded) = <CountNotMatch as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CountNotMatch(decoded));
            }
            if let Ok(decoded) =
                <DelegateChainIdNotMatch as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DelegateChainIdNotMatch(decoded));
            }
            if let Ok(decoded) =
                <DelegateReceiverNotMatch as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DelegateReceiverNotMatch(decoded));
            }
            if let Ok(decoded) =
                <DelegateSignerNotMatch as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DelegateSignerNotMatch(decoded));
            }
            if let Ok(decoded) = <DelegatecallFail as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DelegatecallFail(decoded));
            }
            if let Ok(decoded) =
                <EnumerableSetError as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EnumerableSetError(decoded));
            }
            if let Ok(decoded) =
                <FrozenBalanceInconsistent as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FrozenBalanceInconsistent(decoded));
            }
            if let Ok(decoded) =
                <InsurancePositionQtyInvalid as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsurancePositionQtyInvalid(decoded));
            }
            if let Ok(decoded) =
                <InsuranceTransferAmountInvalid as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsuranceTransferAmountInvalid(decoded));
            }
            if let Ok(decoded) =
                <InsuranceTransferToSelf as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsuranceTransferToSelf(decoded));
            }
            if let Ok(decoded) = <InvalidBizType as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidBizType(decoded));
            }
            if let Ok(decoded) =
                <InvalidFeeCollectorType as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidFeeCollectorType(decoded));
            }
            if let Ok(decoded) =
                <InvalidPrimeWallet as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidPrimeWallet(decoded));
            }
            if let Ok(decoded) = <LedgerAddressZero as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LedgerAddressZero(decoded));
            }
            if let Ok(decoded) = <NotImplemented as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotImplemented(decoded));
            }
            if let Ok(decoded) =
                <OnlyCrossChainManagerCanCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyCrossChainManagerCanCall(decoded));
            }
            if let Ok(decoded) =
                <OnlyCrossChainManagerV2CanCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyCrossChainManagerV2CanCall(decoded));
            }
            if let Ok(decoded) = <OnlyLedgerCanCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyLedgerCanCall(decoded));
            }
            if let Ok(decoded) =
                <OnlyOperatorCanCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyOperatorCanCall(decoded));
            }
            if let Ok(decoded) =
                <OnlyOperatorManagerCanCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyOperatorManagerCanCall(decoded));
            }
            if let Ok(decoded) =
                <OnlySymbolManagerOrOwner as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlySymbolManagerOrOwner(decoded));
            }
            if let Ok(decoded) =
                <OperatorManagerAddressZero as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorManagerAddressZero(decoded));
            }
            if let Ok(decoded) =
                <ProtocolVaultAddressMismatch as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProtocolVaultAddressMismatch(decoded));
            }
            if let Ok(decoded) =
                <RebalanceAlreadySucc as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RebalanceAlreadySucc(decoded));
            }
            if let Ok(decoded) =
                <RebalanceChainIdInvalid as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RebalanceChainIdInvalid(decoded));
            }
            if let Ok(decoded) =
                <RebalanceIdNotMatch as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RebalanceIdNotMatch(decoded));
            }
            if let Ok(decoded) =
                <RebalanceMintUnexpected as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RebalanceMintUnexpected(decoded));
            }
            if let Ok(decoded) =
                <RebalanceStillPending as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RebalanceStillPending(decoded));
            }
            if let Ok(decoded) =
                <RebalanceTokenNotSupported as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RebalanceTokenNotSupported(decoded));
            }
            if let Ok(decoded) = <SafeCastOverflow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SafeCastOverflow(decoded));
            }
            if let Ok(decoded) = <SafeCastUnderflow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SafeCastUnderflow(decoded));
            }
            if let Ok(decoded) = <SignatureNotMatch as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignatureNotMatch(decoded));
            }
            if let Ok(decoded) = <SymbolNotAllowed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SymbolNotAllowed(decoded));
            }
            if let Ok(decoded) = <SymbolNotRegister as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SymbolNotRegister(decoded));
            }
            if let Ok(decoded) = <TokenNotAllowed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenNotAllowed(decoded));
            }
            if let Ok(decoded) =
                <TotalSettleAmountNotMatch as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TotalSettleAmountNotMatch(decoded));
            }
            if let Ok(decoded) =
                <UserPerpPositionQtyZero as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UserPerpPositionQtyZero(decoded));
            }
            if let Ok(decoded) =
                <WithdrawBalanceNotEnough as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawBalanceNotEnough(decoded));
            }
            if let Ok(decoded) =
                <WithdrawEscrowBalanceNotEnough as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawEscrowBalanceNotEnough(decoded));
            }
            if let Ok(decoded) =
                <WithdrawFeeTooLarge as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawFeeTooLarge(decoded));
            }
            if let Ok(decoded) =
                <WithdrawToAddressZero as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawToAddressZero(decoded));
            }
            if let Ok(decoded) =
                <WithdrawVaultBalanceNotEnough as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawVaultBalanceNotEnough(decoded));
            }
            if let Ok(decoded) = <ZeroChainId as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ZeroChainId(decoded));
            }
            if let Ok(decoded) =
                <ZeroDelegateContract as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ZeroDelegateContract(decoded));
            }
            if let Ok(decoded) =
                <ZeroDelegateSigner as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ZeroDelegateSigner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for user_ledgerErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AccountIdInvalid(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddressZero(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceNotEnough(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BatchIdNotMatch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BrokerNotAllowed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Bytes32Zero(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CountNotMatch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DelegateChainIdNotMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegateReceiverNotMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegateSignerNotMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegatecallFail(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnumerableSetError(element) => {
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
                Self::InvalidBizType(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidFeeCollectorType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidPrimeWallet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LedgerAddressZero(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotImplemented(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyCrossChainManagerCanCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyCrossChainManagerV2CanCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyLedgerCanCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyOperatorCanCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyOperatorManagerCanCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlySymbolManagerOrOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorManagerAddressZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProtocolVaultAddressMismatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RebalanceAlreadySucc(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RebalanceChainIdInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RebalanceIdNotMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RebalanceMintUnexpected(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RebalanceStillPending(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RebalanceTokenNotSupported(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeCastOverflow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeCastUnderflow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SignatureNotMatch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SymbolNotAllowed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SymbolNotRegister(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenNotAllowed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSettleAmountNotMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UserPerpPositionQtyZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawBalanceNotEnough(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawEscrowBalanceNotEnough(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawFeeTooLarge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawToAddressZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawVaultBalanceNotEnough(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroChainId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ZeroDelegateContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroDelegateSigner(element) => {
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
                    == <BatchIdNotMatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BrokerNotAllowed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Bytes32Zero as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <CountNotMatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DelegateChainIdNotMatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DelegateReceiverNotMatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DelegateSignerNotMatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DelegatecallFail as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EnumerableSetError as ::ethers::contract::EthError>::selector() => {
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
                    == <InvalidBizType as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidFeeCollectorType as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidPrimeWallet as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <LedgerAddressZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotImplemented as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyCrossChainManagerCanCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyCrossChainManagerV2CanCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyLedgerCanCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyOperatorCanCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyOperatorManagerCanCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlySymbolManagerOrOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OperatorManagerAddressZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ProtocolVaultAddressMismatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RebalanceAlreadySucc as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RebalanceChainIdInvalid as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RebalanceIdNotMatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RebalanceMintUnexpected as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RebalanceStillPending as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RebalanceTokenNotSupported as ::ethers::contract::EthError>::selector() => {
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
                    == <SignatureNotMatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SymbolNotAllowed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SymbolNotRegister as ::ethers::contract::EthError>::selector() => {
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
                _ if selector
                    == <WithdrawBalanceNotEnough as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <WithdrawEscrowBalanceNotEnough as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <WithdrawFeeTooLarge as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <WithdrawToAddressZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <WithdrawVaultBalanceNotEnough as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ZeroChainId as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ZeroDelegateContract as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ZeroDelegateSigner as ::ethers::contract::EthError>::selector() => {
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
                Self::BatchIdNotMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::BrokerNotAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::Bytes32Zero(element) => ::core::fmt::Display::fmt(element, f),
                Self::CountNotMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegateChainIdNotMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegateReceiverNotMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegateSignerNotMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegatecallFail(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnumerableSetError(element) => ::core::fmt::Display::fmt(element, f),
                Self::FrozenBalanceInconsistent(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsurancePositionQtyInvalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsuranceTransferAmountInvalid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsuranceTransferToSelf(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidBizType(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidFeeCollectorType(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidPrimeWallet(element) => ::core::fmt::Display::fmt(element, f),
                Self::LedgerAddressZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotImplemented(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyCrossChainManagerCanCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnlyCrossChainManagerV2CanCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnlyLedgerCanCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyOperatorCanCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyOperatorManagerCanCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlySymbolManagerOrOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorManagerAddressZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolVaultAddressMismatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RebalanceAlreadySucc(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceChainIdInvalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceIdNotMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceMintUnexpected(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceStillPending(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceTokenNotSupported(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeCastOverflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeCastUnderflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignatureNotMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::SymbolNotAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::SymbolNotRegister(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenNotAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSettleAmountNotMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserPerpPositionQtyZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawBalanceNotEnough(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawEscrowBalanceNotEnough(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawFeeTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawToAddressZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawVaultBalanceNotEnough(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ZeroChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroDelegateContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroDelegateSigner(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<BatchIdNotMatch> for user_ledgerErrors {
        fn from(value: BatchIdNotMatch) -> Self {
            Self::BatchIdNotMatch(value)
        }
    }
    impl ::core::convert::From<BrokerNotAllowed> for user_ledgerErrors {
        fn from(value: BrokerNotAllowed) -> Self {
            Self::BrokerNotAllowed(value)
        }
    }
    impl ::core::convert::From<Bytes32Zero> for user_ledgerErrors {
        fn from(value: Bytes32Zero) -> Self {
            Self::Bytes32Zero(value)
        }
    }
    impl ::core::convert::From<CountNotMatch> for user_ledgerErrors {
        fn from(value: CountNotMatch) -> Self {
            Self::CountNotMatch(value)
        }
    }
    impl ::core::convert::From<DelegateChainIdNotMatch> for user_ledgerErrors {
        fn from(value: DelegateChainIdNotMatch) -> Self {
            Self::DelegateChainIdNotMatch(value)
        }
    }
    impl ::core::convert::From<DelegateReceiverNotMatch> for user_ledgerErrors {
        fn from(value: DelegateReceiverNotMatch) -> Self {
            Self::DelegateReceiverNotMatch(value)
        }
    }
    impl ::core::convert::From<DelegateSignerNotMatch> for user_ledgerErrors {
        fn from(value: DelegateSignerNotMatch) -> Self {
            Self::DelegateSignerNotMatch(value)
        }
    }
    impl ::core::convert::From<DelegatecallFail> for user_ledgerErrors {
        fn from(value: DelegatecallFail) -> Self {
            Self::DelegatecallFail(value)
        }
    }
    impl ::core::convert::From<EnumerableSetError> for user_ledgerErrors {
        fn from(value: EnumerableSetError) -> Self {
            Self::EnumerableSetError(value)
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
    impl ::core::convert::From<InvalidBizType> for user_ledgerErrors {
        fn from(value: InvalidBizType) -> Self {
            Self::InvalidBizType(value)
        }
    }
    impl ::core::convert::From<InvalidFeeCollectorType> for user_ledgerErrors {
        fn from(value: InvalidFeeCollectorType) -> Self {
            Self::InvalidFeeCollectorType(value)
        }
    }
    impl ::core::convert::From<InvalidPrimeWallet> for user_ledgerErrors {
        fn from(value: InvalidPrimeWallet) -> Self {
            Self::InvalidPrimeWallet(value)
        }
    }
    impl ::core::convert::From<LedgerAddressZero> for user_ledgerErrors {
        fn from(value: LedgerAddressZero) -> Self {
            Self::LedgerAddressZero(value)
        }
    }
    impl ::core::convert::From<NotImplemented> for user_ledgerErrors {
        fn from(value: NotImplemented) -> Self {
            Self::NotImplemented(value)
        }
    }
    impl ::core::convert::From<OnlyCrossChainManagerCanCall> for user_ledgerErrors {
        fn from(value: OnlyCrossChainManagerCanCall) -> Self {
            Self::OnlyCrossChainManagerCanCall(value)
        }
    }
    impl ::core::convert::From<OnlyCrossChainManagerV2CanCall> for user_ledgerErrors {
        fn from(value: OnlyCrossChainManagerV2CanCall) -> Self {
            Self::OnlyCrossChainManagerV2CanCall(value)
        }
    }
    impl ::core::convert::From<OnlyLedgerCanCall> for user_ledgerErrors {
        fn from(value: OnlyLedgerCanCall) -> Self {
            Self::OnlyLedgerCanCall(value)
        }
    }
    impl ::core::convert::From<OnlyOperatorCanCall> for user_ledgerErrors {
        fn from(value: OnlyOperatorCanCall) -> Self {
            Self::OnlyOperatorCanCall(value)
        }
    }
    impl ::core::convert::From<OnlyOperatorManagerCanCall> for user_ledgerErrors {
        fn from(value: OnlyOperatorManagerCanCall) -> Self {
            Self::OnlyOperatorManagerCanCall(value)
        }
    }
    impl ::core::convert::From<OnlySymbolManagerOrOwner> for user_ledgerErrors {
        fn from(value: OnlySymbolManagerOrOwner) -> Self {
            Self::OnlySymbolManagerOrOwner(value)
        }
    }
    impl ::core::convert::From<OperatorManagerAddressZero> for user_ledgerErrors {
        fn from(value: OperatorManagerAddressZero) -> Self {
            Self::OperatorManagerAddressZero(value)
        }
    }
    impl ::core::convert::From<ProtocolVaultAddressMismatch> for user_ledgerErrors {
        fn from(value: ProtocolVaultAddressMismatch) -> Self {
            Self::ProtocolVaultAddressMismatch(value)
        }
    }
    impl ::core::convert::From<RebalanceAlreadySucc> for user_ledgerErrors {
        fn from(value: RebalanceAlreadySucc) -> Self {
            Self::RebalanceAlreadySucc(value)
        }
    }
    impl ::core::convert::From<RebalanceChainIdInvalid> for user_ledgerErrors {
        fn from(value: RebalanceChainIdInvalid) -> Self {
            Self::RebalanceChainIdInvalid(value)
        }
    }
    impl ::core::convert::From<RebalanceIdNotMatch> for user_ledgerErrors {
        fn from(value: RebalanceIdNotMatch) -> Self {
            Self::RebalanceIdNotMatch(value)
        }
    }
    impl ::core::convert::From<RebalanceMintUnexpected> for user_ledgerErrors {
        fn from(value: RebalanceMintUnexpected) -> Self {
            Self::RebalanceMintUnexpected(value)
        }
    }
    impl ::core::convert::From<RebalanceStillPending> for user_ledgerErrors {
        fn from(value: RebalanceStillPending) -> Self {
            Self::RebalanceStillPending(value)
        }
    }
    impl ::core::convert::From<RebalanceTokenNotSupported> for user_ledgerErrors {
        fn from(value: RebalanceTokenNotSupported) -> Self {
            Self::RebalanceTokenNotSupported(value)
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
    impl ::core::convert::From<SignatureNotMatch> for user_ledgerErrors {
        fn from(value: SignatureNotMatch) -> Self {
            Self::SignatureNotMatch(value)
        }
    }
    impl ::core::convert::From<SymbolNotAllowed> for user_ledgerErrors {
        fn from(value: SymbolNotAllowed) -> Self {
            Self::SymbolNotAllowed(value)
        }
    }
    impl ::core::convert::From<SymbolNotRegister> for user_ledgerErrors {
        fn from(value: SymbolNotRegister) -> Self {
            Self::SymbolNotRegister(value)
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
    impl ::core::convert::From<WithdrawBalanceNotEnough> for user_ledgerErrors {
        fn from(value: WithdrawBalanceNotEnough) -> Self {
            Self::WithdrawBalanceNotEnough(value)
        }
    }
    impl ::core::convert::From<WithdrawEscrowBalanceNotEnough> for user_ledgerErrors {
        fn from(value: WithdrawEscrowBalanceNotEnough) -> Self {
            Self::WithdrawEscrowBalanceNotEnough(value)
        }
    }
    impl ::core::convert::From<WithdrawFeeTooLarge> for user_ledgerErrors {
        fn from(value: WithdrawFeeTooLarge) -> Self {
            Self::WithdrawFeeTooLarge(value)
        }
    }
    impl ::core::convert::From<WithdrawToAddressZero> for user_ledgerErrors {
        fn from(value: WithdrawToAddressZero) -> Self {
            Self::WithdrawToAddressZero(value)
        }
    }
    impl ::core::convert::From<WithdrawVaultBalanceNotEnough> for user_ledgerErrors {
        fn from(value: WithdrawVaultBalanceNotEnough) -> Self {
            Self::WithdrawVaultBalanceNotEnough(value)
        }
    }
    impl ::core::convert::From<ZeroChainId> for user_ledgerErrors {
        fn from(value: ZeroChainId) -> Self {
            Self::ZeroChainId(value)
        }
    }
    impl ::core::convert::From<ZeroDelegateContract> for user_ledgerErrors {
        fn from(value: ZeroDelegateContract) -> Self {
            Self::ZeroDelegateContract(value)
        }
    }
    impl ::core::convert::From<ZeroDelegateSigner> for user_ledgerErrors {
        fn from(value: ZeroDelegateSigner) -> Self {
            Self::ZeroDelegateSigner(value)
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethevent(
        name = "AccountDepositSol",
        abi = "AccountDepositSol(bytes32,uint64,uint64,bytes32,bytes32,uint128,uint256,uint64,bytes32)"
    )]
    pub struct AccountDepositSolFilter {
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        #[ethevent(indexed)]
        pub deposit_nonce: u64,
        #[ethevent(indexed)]
        pub event_id: u64,
        pub pubkey: [u8; 32],
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
        Hash,
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
        Hash,
    )]
    #[ethevent(
        name = "AccountRegister",
        abi = "AccountRegister(bytes32,bytes32,bytes32)"
    )]
    pub struct AccountRegister2Filter {
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        #[ethevent(indexed)]
        pub broker_id: [u8; 32],
        #[ethevent(indexed)]
        pub pubkey: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "AccountRegister",
        abi = "AccountRegister(bytes32,bytes32,address,uint256)"
    )]
    pub struct AccountRegister3Filter {
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
        Hash,
    )]
    #[ethevent(
        name = "AccountWithdraw2Contract",
        abi = "AccountWithdraw2Contract(bytes32,uint64,uint64,uint256,bytes32,uint128,uint128,address)"
    )]
    pub struct AccountWithdraw2ContractFilter {
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        #[ethevent(indexed)]
        pub withdraw_nonce: u64,
        #[ethevent(indexed)]
        pub event_id: u64,
        pub chain_id: ::ethers::core::types::U256,
        pub token_hash: [u8; 32],
        pub token_amount: u128,
        pub fee: u128,
        pub receiver: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Hash,
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
        Hash,
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
        pub fail_reason: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        pub fail_reason: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethevent(
        name = "AccountWithdrawSolApprove",
        abi = "AccountWithdrawSolApprove(bytes32,uint64,uint64,bytes32,bytes32,bytes32,uint256,bytes32,uint128,uint128)"
    )]
    pub struct AccountWithdrawSolApproveFilter {
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        #[ethevent(indexed)]
        pub withdraw_nonce: u64,
        #[ethevent(indexed)]
        pub event_id: u64,
        pub broker_hash: [u8; 32],
        pub sender: [u8; 32],
        pub receiver: [u8; 32],
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
        Hash,
    )]
    #[ethevent(
        name = "AccountWithdrawSolFail",
        abi = "AccountWithdrawSolFail(bytes32,uint64,uint64,bytes32,bytes32,bytes32,uint256,bytes32,uint128,uint128,uint8)"
    )]
    pub struct AccountWithdrawSolFailFilter {
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        #[ethevent(indexed)]
        pub withdraw_nonce: u64,
        #[ethevent(indexed)]
        pub event_id: u64,
        pub broker_hash: [u8; 32],
        pub sender: [u8; 32],
        pub receiver: [u8; 32],
        pub chain_id: ::ethers::core::types::U256,
        pub token_hash: [u8; 32],
        pub token_amount: u128,
        pub fee: u128,
        pub fail_reason: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Hash,
    )]
    #[ethevent(
        name = "AdlResultV2",
        abi = "AdlResultV2(uint64,bytes32,bytes32,int128,int128,uint128,int128,uint64)"
    )]
    pub struct AdlResultV2Filter {
        #[ethevent(indexed)]
        pub event_id: u64,
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
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
        Hash,
    )]
    #[ethevent(
        name = "BalanceTransfer",
        abi = "BalanceTransfer(uint64,uint256,bytes32,bytes32,uint128,bytes32,bool,uint8)"
    )]
    pub struct BalanceTransferFilter {
        #[ethevent(indexed)]
        pub event_id: u64,
        #[ethevent(indexed)]
        pub transfer_id: ::ethers::core::types::U256,
        pub from_account_id: [u8; 32],
        pub to_account_id: [u8; 32],
        pub amount: u128,
        pub token_hash: [u8; 32],
        pub is_from_account_id: bool,
        pub transfer_type: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Hash,
    )]
    #[ethevent(
        name = "ChangeCrossChainManagerV2",
        abi = "ChangeCrossChainManagerV2(address,address)"
    )]
    pub struct ChangeCrossChainManagerV2Filter {
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
        Hash,
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
        Hash,
    )]
    #[ethevent(name = "ChangeLedgerImplA", abi = "ChangeLedgerImplA(address,address)")]
    pub struct ChangeLedgerImplAFilter {
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
        Hash,
    )]
    #[ethevent(name = "ChangeLedgerImplB", abi = "ChangeLedgerImplB(address,address)")]
    pub struct ChangeLedgerImplBFilter {
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
        Hash,
    )]
    #[ethevent(name = "ChangeLedgerImplC", abi = "ChangeLedgerImplC(address,address)")]
    pub struct ChangeLedgerImplCFilter {
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
        Hash,
    )]
    #[ethevent(name = "ChangeLedgerImplD", abi = "ChangeLedgerImplD(address,address)")]
    pub struct ChangeLedgerImplDFilter {
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethevent(
        name = "ChangeVaultManager",
        abi = "ChangeVaultManager(address,address)"
    )]
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
        Hash,
    )]
    #[ethevent(
        name = "DelegateSigner",
        abi = "DelegateSigner(uint64,uint256,bytes32,address,bytes32,address)"
    )]
    pub struct DelegateSignerFilter {
        #[ethevent(indexed)]
        pub event_id: u64,
        #[ethevent(indexed)]
        pub chain_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        pub delegate_contract: ::ethers::core::types::Address,
        pub broker_hash: [u8; 32],
        pub delegate_signer: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "FeeDistribution",
        abi = "FeeDistribution(uint64,bytes32,bytes32,uint128,bytes32)"
    )]
    pub struct FeeDistributionFilter {
        #[ethevent(indexed)]
        pub event_id: u64,
        #[ethevent(indexed)]
        pub from_account_id: [u8; 32],
        #[ethevent(indexed)]
        pub to_account_id: [u8; 32],
        pub amount: u128,
        pub token_hash: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "InternalTransferFinalised",
        abi = "InternalTransferFinalised(uint64,uint256,bytes32,bytes32,uint128)"
    )]
    pub struct InternalTransferFinalisedFilter {
        #[ethevent(indexed)]
        pub event_id: u64,
        #[ethevent(indexed)]
        pub transfer_id: ::ethers::core::types::U256,
        pub to_account_id: [u8; 32],
        pub token_hash: [u8; 32],
        pub amount: u128,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Hash,
    )]
    #[ethevent(
        name = "LiquidationResultV2",
        abi = "LiquidationResultV2(uint64,bytes32,bytes32,int128,uint64)"
    )]
    pub struct LiquidationResultV2Filter {
        #[ethevent(indexed)]
        pub event_id: u64,
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        pub liquidated_asset_hash: [u8; 32],
        pub insurance_transfer_amount: i128,
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
        Hash,
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
        Hash,
    )]
    #[ethevent(
        name = "LiquidationTransferV2",
        abi = "LiquidationTransferV2(bytes32,bytes32,int128,int128,int128,uint128,int128)"
    )]
    pub struct LiquidationTransferV2Filter {
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        #[ethevent(indexed)]
        pub symbol_hash: [u8; 32],
        pub position_qty_transfer: i128,
        pub cost_position_transfer: i128,
        pub fee: i128,
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
        Hash,
    )]
    #[ethevent(name = "PrimeWalletSet", abi = "PrimeWalletSet(bytes32,address)")]
    pub struct PrimeWalletSetFilter {
        pub id: [u8; 32],
        pub prime_wallet: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "ProcessValidatedFutures",
        abi = "ProcessValidatedFutures(bytes32,bytes32,bytes32,int128,int128,uint128,int128,int128,uint64,uint64,uint64,bool)"
    )]
    pub struct ProcessValidatedFutures1Filter {
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        #[ethevent(indexed)]
        pub symbol_hash: [u8; 32],
        pub fee_asset_hash: [u8; 32],
        pub trade_qty: i128,
        pub notional: i128,
        pub executed_price: u128,
        pub fee: i128,
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
        Hash,
    )]
    #[ethevent(
        name = "ProcessValidatedFutures",
        abi = "ProcessValidatedFutures(bytes32,bytes32,bytes32,int128,int128,uint128,uint128,int128,uint64,uint64,uint64,bool)"
    )]
    pub struct ProcessValidatedFutures2Filter {
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
        Hash,
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
        Hash,
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
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "SwapResultUploaded",
        abi = "SwapResultUploaded(uint64,bytes32,bytes32,bytes32,int128,int128,uint256,uint8)"
    )]
    pub struct SwapResultUploadedFilter {
        #[ethevent(indexed)]
        pub event_id: u64,
        #[ethevent(indexed)]
        pub account_id: [u8; 32],
        pub buy_token_hash: [u8; 32],
        pub sell_token_hash: [u8; 32],
        pub buy_quantity: i128,
        pub sell_quantity: i128,
        pub chain_id: ::ethers::core::types::U256,
        pub swap_status: u8,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum user_ledgerEvents {
        AccountDeposit1Filter(AccountDeposit1Filter),
        AccountDeposit2Filter(AccountDeposit2Filter),
        AccountDepositSolFilter(AccountDepositSolFilter),
        AccountRegister1Filter(AccountRegister1Filter),
        AccountRegister2Filter(AccountRegister2Filter),
        AccountRegister3Filter(AccountRegister3Filter),
        AccountWithdraw2ContractFilter(AccountWithdraw2ContractFilter),
        AccountWithdrawApprove1Filter(AccountWithdrawApprove1Filter),
        AccountWithdrawApprove2Filter(AccountWithdrawApprove2Filter),
        AccountWithdrawFail1Filter(AccountWithdrawFail1Filter),
        AccountWithdrawFail2Filter(AccountWithdrawFail2Filter),
        AccountWithdrawFinish1Filter(AccountWithdrawFinish1Filter),
        AccountWithdrawFinish2Filter(AccountWithdrawFinish2Filter),
        AccountWithdrawSolApproveFilter(AccountWithdrawSolApproveFilter),
        AccountWithdrawSolFailFilter(AccountWithdrawSolFailFilter),
        AdlResultFilter(AdlResultFilter),
        AdlResultV2Filter(AdlResultV2Filter),
        BalanceTransferFilter(BalanceTransferFilter),
        ChangeCrossChainManagerFilter(ChangeCrossChainManagerFilter),
        ChangeCrossChainManagerV2Filter(ChangeCrossChainManagerV2Filter),
        ChangeFeeManagerFilter(ChangeFeeManagerFilter),
        ChangeLedgerImplAFilter(ChangeLedgerImplAFilter),
        ChangeLedgerImplBFilter(ChangeLedgerImplBFilter),
        ChangeLedgerImplCFilter(ChangeLedgerImplCFilter),
        ChangeLedgerImplDFilter(ChangeLedgerImplDFilter),
        ChangeMarketManagerFilter(ChangeMarketManagerFilter),
        ChangeOperatorManagerFilter(ChangeOperatorManagerFilter),
        ChangeVaultManagerFilter(ChangeVaultManagerFilter),
        DelegateSignerFilter(DelegateSignerFilter),
        FeeDistributionFilter(FeeDistributionFilter),
        InternalTransferFinalisedFilter(InternalTransferFinalisedFilter),
        LiquidationResultFilter(LiquidationResultFilter),
        LiquidationResultV2Filter(LiquidationResultV2Filter),
        LiquidationTransferFilter(LiquidationTransferFilter),
        LiquidationTransferV2Filter(LiquidationTransferV2Filter),
        PrimeWalletSetFilter(PrimeWalletSetFilter),
        ProcessValidatedFutures1Filter(ProcessValidatedFutures1Filter),
        ProcessValidatedFutures2Filter(ProcessValidatedFutures2Filter),
        SettlementExecutionFilter(SettlementExecutionFilter),
        SettlementResultFilter(SettlementResultFilter),
        SwapResultUploadedFilter(SwapResultUploadedFilter),
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
            if let Ok(decoded) = AccountDepositSolFilter::decode_log(log) {
                return Ok(user_ledgerEvents::AccountDepositSolFilter(decoded));
            }
            if let Ok(decoded) = AccountRegister1Filter::decode_log(log) {
                return Ok(user_ledgerEvents::AccountRegister1Filter(decoded));
            }
            if let Ok(decoded) = AccountRegister2Filter::decode_log(log) {
                return Ok(user_ledgerEvents::AccountRegister2Filter(decoded));
            }
            if let Ok(decoded) = AccountRegister3Filter::decode_log(log) {
                return Ok(user_ledgerEvents::AccountRegister3Filter(decoded));
            }
            if let Ok(decoded) = AccountWithdraw2ContractFilter::decode_log(log) {
                return Ok(user_ledgerEvents::AccountWithdraw2ContractFilter(decoded));
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
            if let Ok(decoded) = AccountWithdrawSolApproveFilter::decode_log(log) {
                return Ok(user_ledgerEvents::AccountWithdrawSolApproveFilter(decoded));
            }
            if let Ok(decoded) = AccountWithdrawSolFailFilter::decode_log(log) {
                return Ok(user_ledgerEvents::AccountWithdrawSolFailFilter(decoded));
            }
            if let Ok(decoded) = AdlResultFilter::decode_log(log) {
                return Ok(user_ledgerEvents::AdlResultFilter(decoded));
            }
            if let Ok(decoded) = AdlResultV2Filter::decode_log(log) {
                return Ok(user_ledgerEvents::AdlResultV2Filter(decoded));
            }
            if let Ok(decoded) = BalanceTransferFilter::decode_log(log) {
                return Ok(user_ledgerEvents::BalanceTransferFilter(decoded));
            }
            if let Ok(decoded) = ChangeCrossChainManagerFilter::decode_log(log) {
                return Ok(user_ledgerEvents::ChangeCrossChainManagerFilter(decoded));
            }
            if let Ok(decoded) = ChangeCrossChainManagerV2Filter::decode_log(log) {
                return Ok(user_ledgerEvents::ChangeCrossChainManagerV2Filter(decoded));
            }
            if let Ok(decoded) = ChangeFeeManagerFilter::decode_log(log) {
                return Ok(user_ledgerEvents::ChangeFeeManagerFilter(decoded));
            }
            if let Ok(decoded) = ChangeLedgerImplAFilter::decode_log(log) {
                return Ok(user_ledgerEvents::ChangeLedgerImplAFilter(decoded));
            }
            if let Ok(decoded) = ChangeLedgerImplBFilter::decode_log(log) {
                return Ok(user_ledgerEvents::ChangeLedgerImplBFilter(decoded));
            }
            if let Ok(decoded) = ChangeLedgerImplCFilter::decode_log(log) {
                return Ok(user_ledgerEvents::ChangeLedgerImplCFilter(decoded));
            }
            if let Ok(decoded) = ChangeLedgerImplDFilter::decode_log(log) {
                return Ok(user_ledgerEvents::ChangeLedgerImplDFilter(decoded));
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
            if let Ok(decoded) = DelegateSignerFilter::decode_log(log) {
                return Ok(user_ledgerEvents::DelegateSignerFilter(decoded));
            }
            if let Ok(decoded) = FeeDistributionFilter::decode_log(log) {
                return Ok(user_ledgerEvents::FeeDistributionFilter(decoded));
            }
            if let Ok(decoded) = InternalTransferFinalisedFilter::decode_log(log) {
                return Ok(user_ledgerEvents::InternalTransferFinalisedFilter(decoded));
            }
            if let Ok(decoded) = LiquidationResultFilter::decode_log(log) {
                return Ok(user_ledgerEvents::LiquidationResultFilter(decoded));
            }
            if let Ok(decoded) = LiquidationResultV2Filter::decode_log(log) {
                return Ok(user_ledgerEvents::LiquidationResultV2Filter(decoded));
            }
            if let Ok(decoded) = LiquidationTransferFilter::decode_log(log) {
                return Ok(user_ledgerEvents::LiquidationTransferFilter(decoded));
            }
            if let Ok(decoded) = LiquidationTransferV2Filter::decode_log(log) {
                return Ok(user_ledgerEvents::LiquidationTransferV2Filter(decoded));
            }
            if let Ok(decoded) = PrimeWalletSetFilter::decode_log(log) {
                return Ok(user_ledgerEvents::PrimeWalletSetFilter(decoded));
            }
            if let Ok(decoded) = ProcessValidatedFutures1Filter::decode_log(log) {
                return Ok(user_ledgerEvents::ProcessValidatedFutures1Filter(decoded));
            }
            if let Ok(decoded) = ProcessValidatedFutures2Filter::decode_log(log) {
                return Ok(user_ledgerEvents::ProcessValidatedFutures2Filter(decoded));
            }
            if let Ok(decoded) = SettlementExecutionFilter::decode_log(log) {
                return Ok(user_ledgerEvents::SettlementExecutionFilter(decoded));
            }
            if let Ok(decoded) = SettlementResultFilter::decode_log(log) {
                return Ok(user_ledgerEvents::SettlementResultFilter(decoded));
            }
            if let Ok(decoded) = SwapResultUploadedFilter::decode_log(log) {
                return Ok(user_ledgerEvents::SwapResultUploadedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for user_ledgerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccountDeposit1Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccountDeposit2Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccountDepositSolFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccountRegister1Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccountRegister2Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccountRegister3Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccountWithdraw2ContractFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccountWithdrawApprove1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccountWithdrawApprove2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccountWithdrawFail1Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccountWithdrawFail2Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccountWithdrawFinish1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccountWithdrawFinish2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccountWithdrawSolApproveFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccountWithdrawSolFailFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AdlResultFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdlResultV2Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceTransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeCrossChainManagerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeCrossChainManagerV2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeFeeManagerFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeLedgerImplAFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeLedgerImplBFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeLedgerImplCFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeLedgerImplDFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeMarketManagerFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeOperatorManagerFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeVaultManagerFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegateSignerFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeDistributionFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InternalTransferFinalisedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidationResultFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidationResultV2Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidationTransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidationTransferV2Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrimeWalletSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessValidatedFutures1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProcessValidatedFutures2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SettlementExecutionFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SettlementResultFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapResultUploadedFilter(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<AccountDepositSolFilter> for user_ledgerEvents {
        fn from(value: AccountDepositSolFilter) -> Self {
            Self::AccountDepositSolFilter(value)
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
    impl ::core::convert::From<AccountRegister3Filter> for user_ledgerEvents {
        fn from(value: AccountRegister3Filter) -> Self {
            Self::AccountRegister3Filter(value)
        }
    }
    impl ::core::convert::From<AccountWithdraw2ContractFilter> for user_ledgerEvents {
        fn from(value: AccountWithdraw2ContractFilter) -> Self {
            Self::AccountWithdraw2ContractFilter(value)
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
    impl ::core::convert::From<AccountWithdrawSolApproveFilter> for user_ledgerEvents {
        fn from(value: AccountWithdrawSolApproveFilter) -> Self {
            Self::AccountWithdrawSolApproveFilter(value)
        }
    }
    impl ::core::convert::From<AccountWithdrawSolFailFilter> for user_ledgerEvents {
        fn from(value: AccountWithdrawSolFailFilter) -> Self {
            Self::AccountWithdrawSolFailFilter(value)
        }
    }
    impl ::core::convert::From<AdlResultFilter> for user_ledgerEvents {
        fn from(value: AdlResultFilter) -> Self {
            Self::AdlResultFilter(value)
        }
    }
    impl ::core::convert::From<AdlResultV2Filter> for user_ledgerEvents {
        fn from(value: AdlResultV2Filter) -> Self {
            Self::AdlResultV2Filter(value)
        }
    }
    impl ::core::convert::From<BalanceTransferFilter> for user_ledgerEvents {
        fn from(value: BalanceTransferFilter) -> Self {
            Self::BalanceTransferFilter(value)
        }
    }
    impl ::core::convert::From<ChangeCrossChainManagerFilter> for user_ledgerEvents {
        fn from(value: ChangeCrossChainManagerFilter) -> Self {
            Self::ChangeCrossChainManagerFilter(value)
        }
    }
    impl ::core::convert::From<ChangeCrossChainManagerV2Filter> for user_ledgerEvents {
        fn from(value: ChangeCrossChainManagerV2Filter) -> Self {
            Self::ChangeCrossChainManagerV2Filter(value)
        }
    }
    impl ::core::convert::From<ChangeFeeManagerFilter> for user_ledgerEvents {
        fn from(value: ChangeFeeManagerFilter) -> Self {
            Self::ChangeFeeManagerFilter(value)
        }
    }
    impl ::core::convert::From<ChangeLedgerImplAFilter> for user_ledgerEvents {
        fn from(value: ChangeLedgerImplAFilter) -> Self {
            Self::ChangeLedgerImplAFilter(value)
        }
    }
    impl ::core::convert::From<ChangeLedgerImplBFilter> for user_ledgerEvents {
        fn from(value: ChangeLedgerImplBFilter) -> Self {
            Self::ChangeLedgerImplBFilter(value)
        }
    }
    impl ::core::convert::From<ChangeLedgerImplCFilter> for user_ledgerEvents {
        fn from(value: ChangeLedgerImplCFilter) -> Self {
            Self::ChangeLedgerImplCFilter(value)
        }
    }
    impl ::core::convert::From<ChangeLedgerImplDFilter> for user_ledgerEvents {
        fn from(value: ChangeLedgerImplDFilter) -> Self {
            Self::ChangeLedgerImplDFilter(value)
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
    impl ::core::convert::From<DelegateSignerFilter> for user_ledgerEvents {
        fn from(value: DelegateSignerFilter) -> Self {
            Self::DelegateSignerFilter(value)
        }
    }
    impl ::core::convert::From<FeeDistributionFilter> for user_ledgerEvents {
        fn from(value: FeeDistributionFilter) -> Self {
            Self::FeeDistributionFilter(value)
        }
    }
    impl ::core::convert::From<InternalTransferFinalisedFilter> for user_ledgerEvents {
        fn from(value: InternalTransferFinalisedFilter) -> Self {
            Self::InternalTransferFinalisedFilter(value)
        }
    }
    impl ::core::convert::From<LiquidationResultFilter> for user_ledgerEvents {
        fn from(value: LiquidationResultFilter) -> Self {
            Self::LiquidationResultFilter(value)
        }
    }
    impl ::core::convert::From<LiquidationResultV2Filter> for user_ledgerEvents {
        fn from(value: LiquidationResultV2Filter) -> Self {
            Self::LiquidationResultV2Filter(value)
        }
    }
    impl ::core::convert::From<LiquidationTransferFilter> for user_ledgerEvents {
        fn from(value: LiquidationTransferFilter) -> Self {
            Self::LiquidationTransferFilter(value)
        }
    }
    impl ::core::convert::From<LiquidationTransferV2Filter> for user_ledgerEvents {
        fn from(value: LiquidationTransferV2Filter) -> Self {
            Self::LiquidationTransferV2Filter(value)
        }
    }
    impl ::core::convert::From<PrimeWalletSetFilter> for user_ledgerEvents {
        fn from(value: PrimeWalletSetFilter) -> Self {
            Self::PrimeWalletSetFilter(value)
        }
    }
    impl ::core::convert::From<ProcessValidatedFutures1Filter> for user_ledgerEvents {
        fn from(value: ProcessValidatedFutures1Filter) -> Self {
            Self::ProcessValidatedFutures1Filter(value)
        }
    }
    impl ::core::convert::From<ProcessValidatedFutures2Filter> for user_ledgerEvents {
        fn from(value: ProcessValidatedFutures2Filter) -> Self {
            Self::ProcessValidatedFutures2Filter(value)
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
    impl ::core::convert::From<SwapResultUploadedFilter> for user_ledgerEvents {
        fn from(value: SwapResultUploadedFilter) -> Self {
            Self::SwapResultUploadedFilter(value)
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
        Hash,
    )]
    #[ethcall(
        name = "accountDeposit",
        abi = "accountDeposit((bytes32,bytes32,address,bytes32,uint256,uint128,uint64))"
    )]
    pub struct AccountDepositCall {
        pub data: AccountDeposit,
    }
    ///Container type for all input parameters for the `accountDepositSol` function with signature `accountDepositSol((bytes32,bytes32,bytes32,bytes32,uint256,uint128,uint64))` and selector `0xe4ea46aa`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "accountDepositSol",
        abi = "accountDepositSol((bytes32,bytes32,bytes32,bytes32,uint256,uint128,uint64))"
    )]
    pub struct AccountDepositSolCall {
        pub data: AccountDepositSol,
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
        Hash,
    )]
    #[ethcall(
        name = "accountWithDrawFinish",
        abi = "accountWithDrawFinish((bytes32,address,address,bytes32,bytes32,uint128,uint128,uint256,uint64))"
    )]
    pub struct AccountWithDrawFinishCall {
        pub withdraw: AccountWithdraw,
    }
    ///Container type for all input parameters for the `accountWithdrawFail` function with signature `accountWithdrawFail((bytes32,address,address,bytes32,bytes32,uint128,uint128,uint256,uint64))` and selector `0x26acf6e1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "accountWithdrawFail",
        abi = "accountWithdrawFail((bytes32,address,address,bytes32,bytes32,uint128,uint128,uint256,uint64))"
    )]
    pub struct AccountWithdrawFailCall {
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
        Hash,
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
        Hash,
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
    ///Container type for all input parameters for the `executeAdl` function with signature `executeAdl((bytes32,bytes32,bytes32,int128,int128,uint128,int128,uint64),uint64)` and selector `0xc61ca104`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "executeAdl",
        abi = "executeAdl((bytes32,bytes32,bytes32,int128,int128,uint128,int128,uint64),uint64)"
    )]
    pub struct ExecuteAdlCall {
        pub adl: Adl,
        pub event_id: u64,
    }
    ///Container type for all input parameters for the `executeAdlV2` function with signature `executeAdlV2((bytes32,bytes32,int128,int128,uint128,int128,uint64,bool),uint64)` and selector `0xf97a259c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "executeAdlV2",
        abi = "executeAdlV2((bytes32,bytes32,int128,int128,uint128,int128,uint64,bool),uint64)"
    )]
    pub struct ExecuteAdlV2Call {
        pub adl: AdlV2,
        pub event_id: u64,
    }
    ///Container type for all input parameters for the `executeBalanceTransfer` function with signature `executeBalanceTransfer((bytes32,bytes32,uint128,bytes32,bool,uint8,uint256),uint64)` and selector `0xf83bd887`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "executeBalanceTransfer",
        abi = "executeBalanceTransfer((bytes32,bytes32,uint128,bytes32,bool,uint8,uint256),uint64)"
    )]
    pub struct ExecuteBalanceTransferCall {
        pub balance_transfer: BalanceTransfer,
        pub event_id: u64,
    }
    ///Container type for all input parameters for the `executeDelegateSigner` function with signature `executeDelegateSigner((address,address,bytes32,uint256),uint64)` and selector `0x0997c228`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "executeDelegateSigner",
        abi = "executeDelegateSigner((address,address,bytes32,uint256),uint64)"
    )]
    pub struct ExecuteDelegateSignerCall {
        pub delegate_signer: DelegateSigner,
        pub event_id: u64,
    }
    ///Container type for all input parameters for the `executeDelegateWithdrawAction` function with signature `executeDelegateWithdrawAction((uint128,uint128,uint256,bytes32,bytes32,bytes32,uint8,address,uint64,address,uint64,string,string),uint64)` and selector `0xec0a14aa`
    #[derive(Clone, ::ethers::contract::EthCall, ::ethers::contract::EthDisplay)]
    #[ethcall(
        name = "executeDelegateWithdrawAction",
        abi = "executeDelegateWithdrawAction((uint128,uint128,uint256,bytes32,bytes32,bytes32,uint8,address,uint64,address,uint64,string,string),uint64)"
    )]
    pub struct ExecuteDelegateWithdrawActionCall {
        pub delegate_withdraw: WithdrawData,
        pub event_id: u64,
    }
    ///Container type for all input parameters for the `executeFeeDistribution` function with signature `executeFeeDistribution((bytes32,bytes32,uint128,bytes32),uint64)` and selector `0x9078ffd8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "executeFeeDistribution",
        abi = "executeFeeDistribution((bytes32,bytes32,uint128,bytes32),uint64)"
    )]
    pub struct ExecuteFeeDistributionCall {
        pub fee_distribution: FeeDistribution,
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
        Hash,
    )]
    #[ethcall(
        name = "executeLiquidation",
        abi = "executeLiquidation((bytes32,bytes32,bytes32,uint128,uint64,(bytes32,bytes32,int128,int128,int128,int128,int128,uint128,int128,uint64)[]),uint64)"
    )]
    pub struct ExecuteLiquidationCall {
        pub liquidation: Liquidation,
        pub event_id: u64,
    }
    ///Container type for all input parameters for the `executeLiquidationV2` function with signature `executeLiquidationV2((bytes32,bytes32,int128,uint64,bool,(bytes32,int128,int128,int128,uint128,int128)[]),uint64)` and selector `0xb8375d1f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "executeLiquidationV2",
        abi = "executeLiquidationV2((bytes32,bytes32,int128,uint64,bool,(bytes32,int128,int128,int128,uint128,int128)[]),uint64)"
    )]
    pub struct ExecuteLiquidationV2Call {
        pub liquidation: LiquidationV2,
        pub event_id: u64,
    }
    ///Container type for all input parameters for the `executeProcessValidatedFutures` function with signature `executeProcessValidatedFutures((bytes32,bytes32,bytes32,int128,int128,uint128,int128,int128,uint64,uint64,uint64,bool))` and selector `0x0b16ebe4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "executeProcessValidatedFutures",
        abi = "executeProcessValidatedFutures((bytes32,bytes32,bytes32,int128,int128,uint128,int128,int128,uint64,uint64,uint64,bool))"
    )]
    pub struct ExecuteProcessValidatedFuturesCall {
        pub trade: FuturesTradeUpload,
    }
    ///Container type for all input parameters for the `executeProcessValidatedFuturesBatch` function with signature `executeProcessValidatedFuturesBatch((bytes32,bytes32,bytes32,int128,int128,uint128,int128,int128,uint64,uint64,uint64,bool)[])` and selector `0xf8a1018f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "executeProcessValidatedFuturesBatch",
        abi = "executeProcessValidatedFuturesBatch((bytes32,bytes32,bytes32,int128,int128,uint128,int128,int128,uint64,uint64,uint64,bool)[])"
    )]
    pub struct ExecuteProcessValidatedFuturesBatchCall {
        pub trades: ::std::vec::Vec<FuturesTradeUpload>,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(
        name = "executeSettlement",
        abi = "executeSettlement((bytes32,bytes32,bytes32,int128,uint128,uint64,(bytes32,uint128,int128,int128)[]),uint64)"
    )]
    pub struct ExecuteSettlementCall {
        pub ledger: Settlement,
        pub event_id: u64,
    }
    ///Container type for all input parameters for the `executeSwapResultUpload` function with signature `executeSwapResultUpload((bytes32,bytes32,bytes32,int128,int128,uint256,uint8),uint64)` and selector `0xae5f766e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "executeSwapResultUpload",
        abi = "executeSwapResultUpload((bytes32,bytes32,bytes32,int128,int128,uint256,uint8),uint64)"
    )]
    pub struct ExecuteSwapResultUploadCall {
        pub swap_result_upload: SwapResult,
        pub event_id: u64,
    }
    ///Container type for all input parameters for the `executeWithdraw2Contract` function with signature `executeWithdraw2Contract((uint128,uint128,uint256,bytes32,uint8,address,uint64,address,uint64,bytes32,bytes32,uint256),uint64)` and selector `0xa71e351f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "executeWithdraw2Contract",
        abi = "executeWithdraw2Contract((uint128,uint128,uint256,bytes32,uint8,address,uint64,address,uint64,bytes32,bytes32,uint256),uint64)"
    )]
    pub struct ExecuteWithdraw2ContractCall {
        pub data: Withdraw2Contract,
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
    ///Container type for all input parameters for the `executeWithdrawSolAction` function with signature `executeWithdrawSolAction((uint128,uint128,uint256,bytes32,bytes32,bytes32,bytes32,bytes32,uint64,uint64,string,string),uint64)` and selector `0xd2050cb5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "executeWithdrawSolAction",
        abi = "executeWithdrawSolAction((uint128,uint128,uint256,bytes32,bytes32,bytes32,bytes32,bytes32,uint64,uint64,string,string),uint64)"
    )]
    pub struct ExecuteWithdrawSolActionCall {
        pub withdraw: WithdrawDataSol,
        pub event_id: u64,
    }
    ///Container type for all input parameters for the `getBalanceTransferState` function with signature `getBalanceTransferState(uint256)` and selector `0x55b39141`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getBalanceTransferState",
        abi = "getBalanceTransferState(uint256)"
    )]
    pub struct GetBalanceTransferStateCall {
        pub transfer_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getFrozenWithdrawNonce` function with signature `getFrozenWithdrawNonce(bytes32,uint64,bytes32)` and selector `0x782e97e3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
    ///Container type for all input parameters for the `getUserEscrowBalance` function with signature `getUserEscrowBalance(bytes32,bytes32)` and selector `0xa9d31363`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getUserEscrowBalance",
        abi = "getUserEscrowBalance(bytes32,bytes32)"
    )]
    pub struct GetUserEscrowBalanceCall {
        pub account_id: [u8; 32],
        pub token_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `getUserTokenBalance` function with signature `getUserTokenBalance(bytes32,bytes32)` and selector `0x69732a98`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getUserTokenBalance",
        abi = "getUserTokenBalance(bytes32,bytes32)"
    )]
    pub struct GetUserTokenBalanceCall {
        pub account_id: [u8; 32],
        pub token_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `getUserTotalFrozenBalance` function with signature `getUserTotalFrozenBalance(bytes32,bytes32)` and selector `0x9d2aff5f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getUserTotalFrozenBalance",
        abi = "getUserTotalFrozenBalance(bytes32,bytes32)"
    )]
    pub struct GetUserTotalFrozenBalanceCall {
        pub account_id: [u8; 32],
        pub token_hash: [u8; 32],
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
        Hash,
    )]
    #[ethcall(name = "initialize", abi = "initialize()")]
    pub struct InitializeCall;
    ///Container type for all input parameters for the `rebalanceBurnFinish` function with signature `rebalanceBurnFinish((bool,uint64,uint128,bytes32,uint256,uint256))` and selector `0x97f8903e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Hash,
    )]
    #[ethcall(
        name = "rebalanceMintFinish",
        abi = "rebalanceMintFinish((bool,uint64,uint128,bytes32,uint256,uint256))"
    )]
    pub struct RebalanceMintFinishCall {
        pub data: RebalanceMintCCFinishData,
    }
    ///Container type for all input parameters for the `setCrossChainManager` function with signature `setCrossChainManager(address)` and selector `0x5e1eb4ce`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setCrossChainManager", abi = "setCrossChainManager(address)")]
    pub struct SetCrossChainManagerCall {
        pub cross_chain_manager_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setCrossChainManagerV2` function with signature `setCrossChainManagerV2(address)` and selector `0xdb0033ae`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "setCrossChainManagerV2",
        abi = "setCrossChainManagerV2(address)"
    )]
    pub struct SetCrossChainManagerV2Call {
        pub cross_chain_manager_v2_address: ::ethers::core::types::Address,
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
        Hash,
    )]
    #[ethcall(name = "setFeeManager", abi = "setFeeManager(address)")]
    pub struct SetFeeManagerCall {
        pub fee_manager_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setLedgerImplA` function with signature `setLedgerImplA(address)` and selector `0x25746836`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setLedgerImplA", abi = "setLedgerImplA(address)")]
    pub struct SetLedgerImplACall {
        pub ledger_impl_a: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setLedgerImplB` function with signature `setLedgerImplB(address)` and selector `0xf10fbcb4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setLedgerImplB", abi = "setLedgerImplB(address)")]
    pub struct SetLedgerImplBCall {
        pub ledger_impl_b: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setLedgerImplC` function with signature `setLedgerImplC(address)` and selector `0x0f8024a8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setLedgerImplC", abi = "setLedgerImplC(address)")]
    pub struct SetLedgerImplCCall {
        pub ledger_impl_c: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setLedgerImplD` function with signature `setLedgerImplD(address)` and selector `0xf5ae6138`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setLedgerImplD", abi = "setLedgerImplD(address)")]
    pub struct SetLedgerImplDCall {
        pub ledger_impl_d: ::ethers::core::types::Address,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "setVaultManager", abi = "setVaultManager(address)")]
    pub struct SetVaultManagerCall {
        pub vault_manager_address: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType)]
    pub enum user_ledgerCalls {
        AccountDeposit(AccountDepositCall),
        AccountDepositSol(AccountDepositSolCall),
        AccountWithDrawFinish(AccountWithDrawFinishCall),
        AccountWithdrawFail(AccountWithdrawFailCall),
        BatchGetUserLedger(BatchGetUserLedgerCall),
        BatchGetUserLedgerWithTokens(BatchGetUserLedgerWithTokensCall),
        ExecuteAdl(ExecuteAdlCall),
        ExecuteAdlV2(ExecuteAdlV2Call),
        ExecuteBalanceTransfer(ExecuteBalanceTransferCall),
        ExecuteDelegateSigner(ExecuteDelegateSignerCall),
        ExecuteDelegateWithdrawAction(ExecuteDelegateWithdrawActionCall),
        ExecuteFeeDistribution(ExecuteFeeDistributionCall),
        ExecuteLiquidation(ExecuteLiquidationCall),
        ExecuteLiquidationV2(ExecuteLiquidationV2Call),
        ExecuteProcessValidatedFutures(ExecuteProcessValidatedFuturesCall),
        ExecuteProcessValidatedFuturesBatch(ExecuteProcessValidatedFuturesBatchCall),
        ExecuteRebalanceBurn(ExecuteRebalanceBurnCall),
        ExecuteRebalanceMint(ExecuteRebalanceMintCall),
        ExecuteSettlement(ExecuteSettlementCall),
        ExecuteSwapResultUpload(ExecuteSwapResultUploadCall),
        ExecuteWithdraw2Contract(ExecuteWithdraw2ContractCall),
        ExecuteWithdrawAction(ExecuteWithdrawActionCall),
        ExecuteWithdrawSolAction(ExecuteWithdrawSolActionCall),
        GetBalanceTransferState(GetBalanceTransferStateCall),
        GetFrozenWithdrawNonce(GetFrozenWithdrawNonceCall),
        GetUserEscrowBalance(GetUserEscrowBalanceCall),
        GetUserTokenBalance(GetUserTokenBalanceCall),
        GetUserTotalFrozenBalance(GetUserTotalFrozenBalanceCall),
        Initialize(InitializeCall),
        RebalanceBurnFinish(RebalanceBurnFinishCall),
        RebalanceMintFinish(RebalanceMintFinishCall),
        SetCrossChainManager(SetCrossChainManagerCall),
        SetCrossChainManagerV2(SetCrossChainManagerV2Call),
        SetFeeManager(SetFeeManagerCall),
        SetLedgerImplA(SetLedgerImplACall),
        SetLedgerImplB(SetLedgerImplBCall),
        SetLedgerImplC(SetLedgerImplCCall),
        SetLedgerImplD(SetLedgerImplDCall),
        SetMarketManager(SetMarketManagerCall),
        SetOperatorManagerAddress(SetOperatorManagerAddressCall),
        SetVaultManager(SetVaultManagerCall),
    }
    impl ::ethers::core::abi::AbiDecode for user_ledgerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <AccountDepositCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AccountDeposit(decoded));
            }
            if let Ok(decoded) =
                <AccountDepositSolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AccountDepositSol(decoded));
            }
            if let Ok(decoded) =
                <AccountWithDrawFinishCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AccountWithDrawFinish(decoded));
            }
            if let Ok(decoded) =
                <AccountWithdrawFailCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AccountWithdrawFail(decoded));
            }
            if let Ok(decoded) =
                <BatchGetUserLedgerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BatchGetUserLedger(decoded));
            }
            if let Ok(decoded) =
                <BatchGetUserLedgerWithTokensCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BatchGetUserLedgerWithTokens(decoded));
            }
            if let Ok(decoded) = <ExecuteAdlCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecuteAdl(decoded));
            }
            if let Ok(decoded) = <ExecuteAdlV2Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteAdlV2(decoded));
            }
            if let Ok(decoded) =
                <ExecuteBalanceTransferCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteBalanceTransfer(decoded));
            }
            if let Ok(decoded) =
                <ExecuteDelegateSignerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteDelegateSigner(decoded));
            }
            if let Ok(decoded) =
                <ExecuteDelegateWithdrawActionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteDelegateWithdrawAction(decoded));
            }
            if let Ok(decoded) =
                <ExecuteFeeDistributionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteFeeDistribution(decoded));
            }
            if let Ok(decoded) =
                <ExecuteLiquidationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteLiquidation(decoded));
            }
            if let Ok(decoded) =
                <ExecuteLiquidationV2Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteLiquidationV2(decoded));
            }
            if let Ok(decoded) =
                <ExecuteProcessValidatedFuturesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteProcessValidatedFutures(decoded));
            }
            if let Ok(decoded) =
                <ExecuteProcessValidatedFuturesBatchCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ExecuteProcessValidatedFuturesBatch(decoded));
            }
            if let Ok(decoded) =
                <ExecuteRebalanceBurnCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteRebalanceBurn(decoded));
            }
            if let Ok(decoded) =
                <ExecuteRebalanceMintCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteRebalanceMint(decoded));
            }
            if let Ok(decoded) =
                <ExecuteSettlementCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteSettlement(decoded));
            }
            if let Ok(decoded) =
                <ExecuteSwapResultUploadCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteSwapResultUpload(decoded));
            }
            if let Ok(decoded) =
                <ExecuteWithdraw2ContractCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteWithdraw2Contract(decoded));
            }
            if let Ok(decoded) =
                <ExecuteWithdrawActionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteWithdrawAction(decoded));
            }
            if let Ok(decoded) =
                <ExecuteWithdrawSolActionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteWithdrawSolAction(decoded));
            }
            if let Ok(decoded) =
                <GetBalanceTransferStateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetBalanceTransferState(decoded));
            }
            if let Ok(decoded) =
                <GetFrozenWithdrawNonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetFrozenWithdrawNonce(decoded));
            }
            if let Ok(decoded) =
                <GetUserEscrowBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetUserEscrowBalance(decoded));
            }
            if let Ok(decoded) =
                <GetUserTokenBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetUserTokenBalance(decoded));
            }
            if let Ok(decoded) =
                <GetUserTotalFrozenBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetUserTotalFrozenBalance(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <RebalanceBurnFinishCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RebalanceBurnFinish(decoded));
            }
            if let Ok(decoded) =
                <RebalanceMintFinishCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RebalanceMintFinish(decoded));
            }
            if let Ok(decoded) =
                <SetCrossChainManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetCrossChainManager(decoded));
            }
            if let Ok(decoded) =
                <SetCrossChainManagerV2Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetCrossChainManagerV2(decoded));
            }
            if let Ok(decoded) = <SetFeeManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetFeeManager(decoded));
            }
            if let Ok(decoded) =
                <SetLedgerImplACall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetLedgerImplA(decoded));
            }
            if let Ok(decoded) =
                <SetLedgerImplBCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetLedgerImplB(decoded));
            }
            if let Ok(decoded) =
                <SetLedgerImplCCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetLedgerImplC(decoded));
            }
            if let Ok(decoded) =
                <SetLedgerImplDCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetLedgerImplD(decoded));
            }
            if let Ok(decoded) =
                <SetMarketManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetMarketManager(decoded));
            }
            if let Ok(decoded) =
                <SetOperatorManagerAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetOperatorManagerAddress(decoded));
            }
            if let Ok(decoded) =
                <SetVaultManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetVaultManager(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for user_ledgerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AccountDeposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AccountDepositSol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AccountWithDrawFinish(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccountWithdrawFail(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BatchGetUserLedger(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BatchGetUserLedgerWithTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteAdl(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteAdlV2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteBalanceTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteDelegateSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteDelegateWithdrawAction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteFeeDistribution(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteLiquidation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteLiquidationV2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteProcessValidatedFutures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteProcessValidatedFuturesBatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteRebalanceBurn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteRebalanceMint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteSettlement(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteSwapResultUpload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteWithdraw2Contract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteWithdrawAction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteWithdrawSolAction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBalanceTransferState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFrozenWithdrawNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUserEscrowBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUserTokenBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUserTotalFrozenBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RebalanceBurnFinish(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RebalanceMintFinish(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetCrossChainManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetCrossChainManagerV2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFeeManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetLedgerImplA(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetLedgerImplB(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetLedgerImplC(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetLedgerImplD(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetMarketManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetOperatorManagerAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetVaultManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for user_ledgerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccountDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccountDepositSol(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccountWithDrawFinish(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccountWithdrawFail(element) => ::core::fmt::Display::fmt(element, f),
                Self::BatchGetUserLedger(element) => ::core::fmt::Display::fmt(element, f),
                Self::BatchGetUserLedgerWithTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteAdl(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteAdlV2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteBalanceTransfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteDelegateSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteDelegateWithdrawAction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteFeeDistribution(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteLiquidation(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteLiquidationV2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteProcessValidatedFutures(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteProcessValidatedFuturesBatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteRebalanceBurn(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteRebalanceMint(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteSettlement(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteSwapResultUpload(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteWithdraw2Contract(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteWithdrawAction(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteWithdrawSolAction(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBalanceTransferState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFrozenWithdrawNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUserEscrowBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUserTokenBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUserTotalFrozenBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceBurnFinish(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceMintFinish(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCrossChainManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCrossChainManagerV2(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLedgerImplA(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLedgerImplB(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLedgerImplC(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLedgerImplD(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMarketManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOperatorManagerAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetVaultManager(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AccountDepositCall> for user_ledgerCalls {
        fn from(value: AccountDepositCall) -> Self {
            Self::AccountDeposit(value)
        }
    }
    impl ::core::convert::From<AccountDepositSolCall> for user_ledgerCalls {
        fn from(value: AccountDepositSolCall) -> Self {
            Self::AccountDepositSol(value)
        }
    }
    impl ::core::convert::From<AccountWithDrawFinishCall> for user_ledgerCalls {
        fn from(value: AccountWithDrawFinishCall) -> Self {
            Self::AccountWithDrawFinish(value)
        }
    }
    impl ::core::convert::From<AccountWithdrawFailCall> for user_ledgerCalls {
        fn from(value: AccountWithdrawFailCall) -> Self {
            Self::AccountWithdrawFail(value)
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
    impl ::core::convert::From<ExecuteAdlCall> for user_ledgerCalls {
        fn from(value: ExecuteAdlCall) -> Self {
            Self::ExecuteAdl(value)
        }
    }
    impl ::core::convert::From<ExecuteAdlV2Call> for user_ledgerCalls {
        fn from(value: ExecuteAdlV2Call) -> Self {
            Self::ExecuteAdlV2(value)
        }
    }
    impl ::core::convert::From<ExecuteBalanceTransferCall> for user_ledgerCalls {
        fn from(value: ExecuteBalanceTransferCall) -> Self {
            Self::ExecuteBalanceTransfer(value)
        }
    }
    impl ::core::convert::From<ExecuteDelegateSignerCall> for user_ledgerCalls {
        fn from(value: ExecuteDelegateSignerCall) -> Self {
            Self::ExecuteDelegateSigner(value)
        }
    }
    impl ::core::convert::From<ExecuteDelegateWithdrawActionCall> for user_ledgerCalls {
        fn from(value: ExecuteDelegateWithdrawActionCall) -> Self {
            Self::ExecuteDelegateWithdrawAction(value)
        }
    }
    impl ::core::convert::From<ExecuteFeeDistributionCall> for user_ledgerCalls {
        fn from(value: ExecuteFeeDistributionCall) -> Self {
            Self::ExecuteFeeDistribution(value)
        }
    }
    impl ::core::convert::From<ExecuteLiquidationCall> for user_ledgerCalls {
        fn from(value: ExecuteLiquidationCall) -> Self {
            Self::ExecuteLiquidation(value)
        }
    }
    impl ::core::convert::From<ExecuteLiquidationV2Call> for user_ledgerCalls {
        fn from(value: ExecuteLiquidationV2Call) -> Self {
            Self::ExecuteLiquidationV2(value)
        }
    }
    impl ::core::convert::From<ExecuteProcessValidatedFuturesCall> for user_ledgerCalls {
        fn from(value: ExecuteProcessValidatedFuturesCall) -> Self {
            Self::ExecuteProcessValidatedFutures(value)
        }
    }
    impl ::core::convert::From<ExecuteProcessValidatedFuturesBatchCall> for user_ledgerCalls {
        fn from(value: ExecuteProcessValidatedFuturesBatchCall) -> Self {
            Self::ExecuteProcessValidatedFuturesBatch(value)
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
    impl ::core::convert::From<ExecuteSwapResultUploadCall> for user_ledgerCalls {
        fn from(value: ExecuteSwapResultUploadCall) -> Self {
            Self::ExecuteSwapResultUpload(value)
        }
    }
    impl ::core::convert::From<ExecuteWithdraw2ContractCall> for user_ledgerCalls {
        fn from(value: ExecuteWithdraw2ContractCall) -> Self {
            Self::ExecuteWithdraw2Contract(value)
        }
    }
    impl ::core::convert::From<ExecuteWithdrawActionCall> for user_ledgerCalls {
        fn from(value: ExecuteWithdrawActionCall) -> Self {
            Self::ExecuteWithdrawAction(value)
        }
    }
    impl ::core::convert::From<ExecuteWithdrawSolActionCall> for user_ledgerCalls {
        fn from(value: ExecuteWithdrawSolActionCall) -> Self {
            Self::ExecuteWithdrawSolAction(value)
        }
    }
    impl ::core::convert::From<GetBalanceTransferStateCall> for user_ledgerCalls {
        fn from(value: GetBalanceTransferStateCall) -> Self {
            Self::GetBalanceTransferState(value)
        }
    }
    impl ::core::convert::From<GetFrozenWithdrawNonceCall> for user_ledgerCalls {
        fn from(value: GetFrozenWithdrawNonceCall) -> Self {
            Self::GetFrozenWithdrawNonce(value)
        }
    }
    impl ::core::convert::From<GetUserEscrowBalanceCall> for user_ledgerCalls {
        fn from(value: GetUserEscrowBalanceCall) -> Self {
            Self::GetUserEscrowBalance(value)
        }
    }
    impl ::core::convert::From<GetUserTokenBalanceCall> for user_ledgerCalls {
        fn from(value: GetUserTokenBalanceCall) -> Self {
            Self::GetUserTokenBalance(value)
        }
    }
    impl ::core::convert::From<GetUserTotalFrozenBalanceCall> for user_ledgerCalls {
        fn from(value: GetUserTotalFrozenBalanceCall) -> Self {
            Self::GetUserTotalFrozenBalance(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for user_ledgerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
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
    impl ::core::convert::From<SetCrossChainManagerCall> for user_ledgerCalls {
        fn from(value: SetCrossChainManagerCall) -> Self {
            Self::SetCrossChainManager(value)
        }
    }
    impl ::core::convert::From<SetCrossChainManagerV2Call> for user_ledgerCalls {
        fn from(value: SetCrossChainManagerV2Call) -> Self {
            Self::SetCrossChainManagerV2(value)
        }
    }
    impl ::core::convert::From<SetFeeManagerCall> for user_ledgerCalls {
        fn from(value: SetFeeManagerCall) -> Self {
            Self::SetFeeManager(value)
        }
    }
    impl ::core::convert::From<SetLedgerImplACall> for user_ledgerCalls {
        fn from(value: SetLedgerImplACall) -> Self {
            Self::SetLedgerImplA(value)
        }
    }
    impl ::core::convert::From<SetLedgerImplBCall> for user_ledgerCalls {
        fn from(value: SetLedgerImplBCall) -> Self {
            Self::SetLedgerImplB(value)
        }
    }
    impl ::core::convert::From<SetLedgerImplCCall> for user_ledgerCalls {
        fn from(value: SetLedgerImplCCall) -> Self {
            Self::SetLedgerImplC(value)
        }
    }
    impl ::core::convert::From<SetLedgerImplDCall> for user_ledgerCalls {
        fn from(value: SetLedgerImplDCall) -> Self {
            Self::SetLedgerImplD(value)
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
    ///Container type for all return fields from the `batchGetUserLedger` function with signature `batchGetUserLedger(bytes32[])` and selector `0x1757cb37`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Hash,
    )]
    pub struct BatchGetUserLedgerWithTokensReturn(pub ::std::vec::Vec<AccountSnapshot>);
    ///Container type for all return fields from the `getBalanceTransferState` function with signature `getBalanceTransferState(uint256)` and selector `0x55b39141`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetBalanceTransferStateReturn(pub InternalTransferTrack);
    ///Container type for all return fields from the `getFrozenWithdrawNonce` function with signature `getFrozenWithdrawNonce(bytes32,uint64,bytes32)` and selector `0x782e97e3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetFrozenWithdrawNonceReturn(pub u128);
    ///Container type for all return fields from the `getUserEscrowBalance` function with signature `getUserEscrowBalance(bytes32,bytes32)` and selector `0xa9d31363`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetUserEscrowBalanceReturn(pub u128);
    ///Container type for all return fields from the `getUserTokenBalance` function with signature `getUserTokenBalance(bytes32,bytes32)` and selector `0x69732a98`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetUserTokenBalanceReturn(pub i128);
    ///Container type for all return fields from the `getUserTotalFrozenBalance` function with signature `getUserTotalFrozenBalance(bytes32,bytes32)` and selector `0x9d2aff5f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetUserTotalFrozenBalanceReturn(pub u128);
    ///`AccountDeposit(bytes32,bytes32,address,bytes32,uint256,uint128,uint64)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
    ///`AccountDepositSol(bytes32,bytes32,bytes32,bytes32,uint256,uint128,uint64)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct AccountDepositSol {
        pub account_id: [u8; 32],
        pub broker_hash: [u8; 32],
        pub pubkey: [u8; 32],
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
        Hash,
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
    ///`AccountSnapshot(bytes32,bytes32,address,uint64,uint64,uint64,uint64,(bytes32,int128,uint128)[],(bytes32,int128,int128,int128,uint128,uint128,uint128,int128,uint128)[],uint64,uint64)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        pub last_deposit_src_chain_id: u64,
        pub last_deposit_src_chain_nonce: u64,
    }
    ///`AccountTokenBalances(bytes32,int128,uint128)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct AccountTokenBalances {
        pub token_hash: [u8; 32],
        pub balance: i128,
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
        Hash,
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
        Hash,
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
    ///`AdlV2(bytes32,bytes32,int128,int128,uint128,int128,uint64,bool)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct AdlV2 {
        pub account_id: [u8; 32],
        pub symbol_hash: [u8; 32],
        pub position_qty_transfer: i128,
        pub cost_position_transfer: i128,
        pub adl_price: u128,
        pub sum_unitary_fundings: i128,
        pub timestamp: u64,
        pub is_insurance_account: bool,
    }
    ///`BalanceTransfer(bytes32,bytes32,uint128,bytes32,bool,uint8,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BalanceTransfer {
        pub from_account_id: [u8; 32],
        pub to_account_id: [u8; 32],
        pub amount: u128,
        pub token_hash: [u8; 32],
        pub is_from_account_id: bool,
        pub transfer_type: u8,
        pub transfer_id: ::ethers::core::types::U256,
    }
    ///`DelegateSigner(address,address,bytes32,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DelegateSigner {
        pub delegate_signer: ::ethers::core::types::Address,
        pub delegate_contract: ::ethers::core::types::Address,
        pub broker_hash: [u8; 32],
        pub chain_id: ::ethers::core::types::U256,
    }
    ///`FeeDistribution(bytes32,bytes32,uint128,bytes32)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FeeDistribution {
        pub from_account_id: [u8; 32],
        pub to_account_id: [u8; 32],
        pub amount: u128,
        pub token_hash: [u8; 32],
    }
    ///`InternalTransferTrack(uint8,bytes32,uint128)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct InternalTransferTrack {
        pub side: u8,
        pub token_hash: [u8; 32],
        pub amount: u128,
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
        Hash,
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
        Hash,
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
    ///`LiquidationTransferV2(bytes32,int128,int128,int128,uint128,int128)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LiquidationTransferV2 {
        pub symbol_hash: [u8; 32],
        pub position_qty_transfer: i128,
        pub cost_position_transfer: i128,
        pub fee: i128,
        pub mark_price: u128,
        pub sum_unitary_fundings: i128,
    }
    ///`LiquidationV2(bytes32,bytes32,int128,uint64,bool,(bytes32,int128,int128,int128,uint128,int128)[])`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LiquidationV2 {
        pub account_id: [u8; 32],
        pub liquidated_asset_hash: [u8; 32],
        pub insurance_transfer_amount: i128,
        pub timestamp: u64,
        pub is_insurance_account: bool,
        pub liquidation_transfers: ::std::vec::Vec<LiquidationTransferV2>,
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
        Hash,
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
        Hash,
    )]
    pub struct SettlementExecution {
        pub symbol_hash: [u8; 32],
        pub mark_price: u128,
        pub sum_unitary_fundings: i128,
        pub settled_amount: i128,
    }
    ///`SwapResult(bytes32,bytes32,bytes32,int128,int128,uint256,uint8)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SwapResult {
        pub account_id: [u8; 32],
        pub buy_token_hash: [u8; 32],
        pub sell_token_hash: [u8; 32],
        pub buy_quantity: i128,
        pub sell_quantity: i128,
        pub chain_id: ::ethers::core::types::U256,
        pub swap_status: u8,
    }
    ///`Withdraw2Contract(uint128,uint128,uint256,bytes32,uint8,address,uint64,address,uint64,bytes32,bytes32,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Withdraw2Contract {
        pub token_amount: u128,
        pub fee: u128,
        pub chain_id: ::ethers::core::types::U256,
        pub account_id: [u8; 32],
        pub vault_type: u8,
        pub sender: ::ethers::core::types::Address,
        pub withdraw_nonce: u64,
        pub receiver: ::ethers::core::types::Address,
        pub timestamp: u64,
        pub broker_hash: [u8; 32],
        pub token_hash: [u8; 32],
        pub client_id: ::ethers::core::types::U256,
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
        Hash,
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
    ///`WithdrawDataSol(uint128,uint128,uint256,bytes32,bytes32,bytes32,bytes32,bytes32,uint64,uint64,string,string)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct WithdrawDataSol {
        pub token_amount: u128,
        pub fee: u128,
        pub chain_id: ::ethers::core::types::U256,
        pub account_id: [u8; 32],
        pub r: [u8; 32],
        pub s: [u8; 32],
        pub sender: [u8; 32],
        pub receiver: [u8; 32],
        pub withdraw_nonce: u64,
        pub timestamp: u64,
        pub broker_id: ::std::string::String,
        pub token_symbol: ::std::string::String,
    }
    ///`FuturesTradeUpload(bytes32,bytes32,bytes32,int128,int128,uint128,int128,int128,uint64,uint64,uint64,bool)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FuturesTradeUpload {
        pub account_id: [u8; 32],
        pub symbol_hash: [u8; 32],
        pub fee_asset_hash: [u8; 32],
        pub trade_qty: i128,
        pub notional: i128,
        pub executed_price: u128,
        pub fee: i128,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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

pub use vault_manager::*;
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
pub mod vault_manager {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addBalance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_tokenHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_deltaBalance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeRebalanceBurn"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("executeRebalanceBurn",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("data"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct RebalanceTypes.RebalanceBurnUploadData",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeRebalanceMint"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("executeRebalanceMint",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("data"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
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
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct RebalanceTypes.RebalanceMintUploadData",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("finishFrozenBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("finishFrozenBalance",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_tokenHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_deltaBalance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("frozenBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("frozenBalance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_tokenHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_deltaBalance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAllAllowedBroker"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAllAllowedBroker",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAllAllowedSymbol"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAllAllowedSymbol",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAllAllowedToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAllAllowedToken"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAllowedBroker"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAllowedBroker"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_brokerHash"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAllowedChainToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAllowedChainToken",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_tokenHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAllowedSymbol"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAllowedSymbol"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_symbolHash"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAllowedToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAllowedToken"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_tokenHash"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getBalance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_tokenHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint128"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getChain2VaultAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getChain2VaultAddress",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("chainId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getChain2cctpDomain"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getChain2cctpDomain",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("chainId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getFrozenBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getFrozenBalance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_tokenHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint128"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMaxWithdrawFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getMaxWithdrawFee"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_tokenHash"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint128"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRebalanceStatus"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getRebalanceStatus"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("rebalanceId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct RebalanceTypes.RebalanceStatus",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ledgerAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("ledgerAddress"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rebalanceBurnFinish"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rebalanceBurnFinish",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("data"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct RebalanceTypes.RebalanceBurnCCFinishData",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rebalanceMintFinish"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rebalanceMintFinish",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("data"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct RebalanceTypes.RebalanceMintCCFinishData",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rebalanceStatus"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rebalanceStatus"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("rebalanceId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("burnStatus"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "enum RebalanceTypes.RebalanceStatusEnum",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("mintStatus"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "enum RebalanceTypes.RebalanceStatusEnum",
                                    ),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setAllowedBroker"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setAllowedBroker"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_brokerHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_allowed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setAllowedChainToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setAllowedChainToken",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_tokenHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_allowed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setAllowedSymbol"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setAllowedSymbol"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_symbolHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_allowed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setAllowedToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setAllowedToken"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_tokenHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_allowed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setChain2VaultAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setChain2VaultAddress",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("chainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("vaultAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setChain2cctpDomain"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setChain2cctpDomain",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("chainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("cctpDomain"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setLedgerAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setLedgerAddress"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_ledgerAddress"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setMaxWithdrawFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setMaxWithdrawFee"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_tokenHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_maxWithdrawFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("subBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("subBalance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_tokenHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_deltaBalance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newOwner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ChangeLedger"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Initialized"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferred",),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RebalanceBurn"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RebalanceBurn"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("rebalanceId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokenHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("srcChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("dstChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RebalanceBurnResult"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RebalanceBurnResult",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("rebalanceId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("success"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RebalanceMint"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RebalanceMint"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("rebalanceId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RebalanceMintResult"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RebalanceMintResult",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("rebalanceId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("success"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetAllowedBroker"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SetAllowedBroker"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_brokerHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_allowed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetAllowedChainToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SetAllowedChainToken",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_tokenHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_allowed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetAllowedSymbol"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SetAllowedSymbol"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_symbolHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_allowed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetAllowedToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SetAllowedToken"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_tokenHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_allowed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetMaxWithdrawFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SetMaxWithdrawFee"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_tokenHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_maxWithdrawFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EnumerableSetError"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("EnumerableSetError"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LedgerAddressZero"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("LedgerAddressZero"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyLedgerCanCall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyLedgerCanCall"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RebalanceAlreadySucc"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("RebalanceAlreadySucc",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RebalanceIdNotMatch"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("RebalanceIdNotMatch",),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RebalanceMintUnexpected"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("RebalanceMintUnexpected",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RebalanceStillPending"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("RebalanceStillPending",),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static VAULT_MANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct vault_manager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for vault_manager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for vault_manager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for vault_manager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for vault_manager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(vault_manager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> vault_manager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                VAULT_MANAGER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `addBalance` (0x566b0849) function
        pub fn add_balance(
            &self,
            token_hash: [u8; 32],
            chain_id: ::ethers::core::types::U256,
            delta_balance: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([86, 107, 8, 73], (token_hash, chain_id, delta_balance))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeRebalanceBurn` (0xb76c1210) function
        pub fn execute_rebalance_burn(
            &self,
            data: RebalanceBurnUploadData,
        ) -> ::ethers::contract::builders::ContractCall<M, (u32, ::ethers::core::types::Address)>
        {
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
        ///Calls the contract's `finishFrozenBalance` (0x46fbccaf) function
        pub fn finish_frozen_balance(
            &self,
            token_hash: [u8; 32],
            chain_id: ::ethers::core::types::U256,
            delta_balance: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 251, 204, 175], (token_hash, chain_id, delta_balance))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `frozenBalance` (0x7a93a06d) function
        pub fn frozen_balance(
            &self,
            token_hash: [u8; 32],
            chain_id: ::ethers::core::types::U256,
            delta_balance: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 147, 160, 109], (token_hash, chain_id, delta_balance))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllAllowedBroker` (0xd6aeb431) function
        pub fn get_all_allowed_broker(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([214, 174, 180, 49], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllAllowedSymbol` (0xb4ab0eca) function
        pub fn get_all_allowed_symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([180, 171, 14, 202], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllAllowedToken` (0x9305a91a) function
        pub fn get_all_allowed_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([147, 5, 169, 26], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllowedBroker` (0x258082f5) function
        pub fn get_allowed_broker(
            &self,
            broker_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([37, 128, 130, 245], broker_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllowedChainToken` (0xe71a2710) function
        pub fn get_allowed_chain_token(
            &self,
            token_hash: [u8; 32],
            chain_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([231, 26, 39, 16], (token_hash, chain_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllowedSymbol` (0x0587f413) function
        pub fn get_allowed_symbol(
            &self,
            symbol_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([5, 135, 244, 19], symbol_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllowedToken` (0xc7eeb9c2) function
        pub fn get_allowed_token(
            &self,
            token_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([199, 238, 185, 194], token_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBalance` (0x9b62377d) function
        pub fn get_balance(
            &self,
            token_hash: [u8; 32],
            chain_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([155, 98, 55, 125], (token_hash, chain_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChain2VaultAddress` (0xa546efff) function
        pub fn get_chain_2_vault_address(
            &self,
            chain_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([165, 70, 239, 255], chain_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChain2cctpDomain` (0xcddb58e6) function
        pub fn get_chain_2cctp_domain(
            &self,
            chain_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([205, 219, 88, 230], chain_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFrozenBalance` (0x0a1f5230) function
        pub fn get_frozen_balance(
            &self,
            token_hash: [u8; 32],
            chain_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([10, 31, 82, 48], (token_hash, chain_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMaxWithdrawFee` (0x4945fe16) function
        pub fn get_max_withdraw_fee(
            &self,
            token_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([73, 69, 254, 22], token_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRebalanceStatus` (0xde559de0) function
        pub fn get_rebalance_status(
            &self,
            rebalance_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, RebalanceStatus> {
            self.0
                .method_hash([222, 85, 157, 224], rebalance_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x8129fc1c) function
        pub fn initialize(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ledgerAddress` (0xd1d20056) function
        pub fn ledger_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([209, 210, 0, 86], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ///Calls the contract's `rebalanceStatus` (0x287ee68b) function
        pub fn rebalance_status(
            &self,
            p0: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, (u64, u8, u8)> {
            self.0
                .method_hash([40, 126, 230, 139], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAllowedBroker` (0xdf0f4ae7) function
        pub fn set_allowed_broker(
            &self,
            broker_hash: [u8; 32],
            allowed: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 15, 74, 231], (broker_hash, allowed))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAllowedChainToken` (0x0b19ccdf) function
        pub fn set_allowed_chain_token(
            &self,
            token_hash: [u8; 32],
            chain_id: ::ethers::core::types::U256,
            allowed: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 25, 204, 223], (token_hash, chain_id, allowed))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAllowedSymbol` (0x47b82bd1) function
        pub fn set_allowed_symbol(
            &self,
            symbol_hash: [u8; 32],
            allowed: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 184, 43, 209], (symbol_hash, allowed))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAllowedToken` (0xc9fc8797) function
        pub fn set_allowed_token(
            &self,
            token_hash: [u8; 32],
            allowed: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 252, 135, 151], (token_hash, allowed))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setChain2VaultAddress` (0xd6d186a1) function
        pub fn set_chain_2_vault_address(
            &self,
            chain_id: ::ethers::core::types::U256,
            vault_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([214, 209, 134, 161], (chain_id, vault_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setChain2cctpDomain` (0xa4564a24) function
        pub fn set_chain_2cctp_domain(
            &self,
            chain_id: ::ethers::core::types::U256,
            cctp_domain: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 86, 74, 36], (chain_id, cctp_domain))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLedgerAddress` (0xba891f5c) function
        pub fn set_ledger_address(
            &self,
            ledger_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 137, 31, 92], ledger_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMaxWithdrawFee` (0xcdf27803) function
        pub fn set_max_withdraw_fee(
            &self,
            token_hash: [u8; 32],
            max_withdraw_fee: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([205, 242, 120, 3], (token_hash, max_withdraw_fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `subBalance` (0xa4f76f2a) function
        pub fn sub_balance(
            &self,
            token_hash: [u8; 32],
            chain_id: ::ethers::core::types::U256,
            delta_balance: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 247, 111, 42], (token_hash, chain_id, delta_balance))
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
        ///Gets the contract's `ChangeLedger` event
        pub fn change_ledger_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ChangeLedgerFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RebalanceBurn` event
        pub fn rebalance_burn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RebalanceBurnFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RebalanceBurnResult` event
        pub fn rebalance_burn_result_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RebalanceBurnResultFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RebalanceMint` event
        pub fn rebalance_mint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RebalanceMintFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RebalanceMintResult` event
        pub fn rebalance_mint_result_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RebalanceMintResultFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SetAllowedBroker` event
        pub fn set_allowed_broker_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetAllowedBrokerFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SetAllowedChainToken` event
        pub fn set_allowed_chain_token_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetAllowedChainTokenFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SetAllowedSymbol` event
        pub fn set_allowed_symbol_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetAllowedSymbolFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SetAllowedToken` event
        pub fn set_allowed_token_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetAllowedTokenFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SetMaxWithdrawFee` event
        pub fn set_max_withdraw_fee_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetMaxWithdrawFeeFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, vault_managerEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for vault_manager<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum vault_managerErrors {
        EnumerableSetError(EnumerableSetError),
        LedgerAddressZero(LedgerAddressZero),
        OnlyLedgerCanCall(OnlyLedgerCanCall),
        RebalanceAlreadySucc(RebalanceAlreadySucc),
        RebalanceIdNotMatch(RebalanceIdNotMatch),
        RebalanceMintUnexpected(RebalanceMintUnexpected),
        RebalanceStillPending(RebalanceStillPending),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for vault_managerErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <EnumerableSetError as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EnumerableSetError(decoded));
            }
            if let Ok(decoded) = <LedgerAddressZero as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LedgerAddressZero(decoded));
            }
            if let Ok(decoded) = <OnlyLedgerCanCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyLedgerCanCall(decoded));
            }
            if let Ok(decoded) =
                <RebalanceAlreadySucc as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RebalanceAlreadySucc(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for vault_managerErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::EnumerableSetError(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LedgerAddressZero(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyLedgerCanCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RebalanceAlreadySucc(element) => {
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
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for vault_managerErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <EnumerableSetError as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <LedgerAddressZero as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <OnlyLedgerCanCall as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <RebalanceAlreadySucc as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <RebalanceIdNotMatch as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <RebalanceMintUnexpected as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <RebalanceStillPending as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for vault_managerErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EnumerableSetError(element) => ::core::fmt::Display::fmt(element, f),
                Self::LedgerAddressZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyLedgerCanCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceAlreadySucc(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceIdNotMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceMintUnexpected(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceStillPending(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for vault_managerErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<EnumerableSetError> for vault_managerErrors {
        fn from(value: EnumerableSetError) -> Self {
            Self::EnumerableSetError(value)
        }
    }
    impl ::core::convert::From<LedgerAddressZero> for vault_managerErrors {
        fn from(value: LedgerAddressZero) -> Self {
            Self::LedgerAddressZero(value)
        }
    }
    impl ::core::convert::From<OnlyLedgerCanCall> for vault_managerErrors {
        fn from(value: OnlyLedgerCanCall) -> Self {
            Self::OnlyLedgerCanCall(value)
        }
    }
    impl ::core::convert::From<RebalanceAlreadySucc> for vault_managerErrors {
        fn from(value: RebalanceAlreadySucc) -> Self {
            Self::RebalanceAlreadySucc(value)
        }
    }
    impl ::core::convert::From<RebalanceIdNotMatch> for vault_managerErrors {
        fn from(value: RebalanceIdNotMatch) -> Self {
            Self::RebalanceIdNotMatch(value)
        }
    }
    impl ::core::convert::From<RebalanceMintUnexpected> for vault_managerErrors {
        fn from(value: RebalanceMintUnexpected) -> Self {
            Self::RebalanceMintUnexpected(value)
        }
    }
    impl ::core::convert::From<RebalanceStillPending> for vault_managerErrors {
        fn from(value: RebalanceStillPending) -> Self {
            Self::RebalanceStillPending(value)
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethevent(
        name = "RebalanceBurn",
        abi = "RebalanceBurn(uint64,uint128,bytes32,uint256,uint256)"
    )]
    pub struct RebalanceBurnFilter {
        #[ethevent(indexed)]
        pub rebalance_id: u64,
        pub amount: u128,
        pub token_hash: [u8; 32],
        pub src_chain_id: ::ethers::core::types::U256,
        pub dst_chain_id: ::ethers::core::types::U256,
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
    #[ethevent(name = "RebalanceBurnResult", abi = "RebalanceBurnResult(uint64,bool)")]
    pub struct RebalanceBurnResultFilter {
        #[ethevent(indexed)]
        pub rebalance_id: u64,
        pub success: bool,
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
    #[ethevent(name = "RebalanceMint", abi = "RebalanceMint(uint64)")]
    pub struct RebalanceMintFilter {
        #[ethevent(indexed)]
        pub rebalance_id: u64,
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
    #[ethevent(name = "RebalanceMintResult", abi = "RebalanceMintResult(uint64,bool)")]
    pub struct RebalanceMintResultFilter {
        #[ethevent(indexed)]
        pub rebalance_id: u64,
        pub success: bool,
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
    #[ethevent(name = "SetAllowedBroker", abi = "SetAllowedBroker(bytes32,bool)")]
    pub struct SetAllowedBrokerFilter {
        #[ethevent(indexed)]
        pub broker_hash: [u8; 32],
        pub allowed: bool,
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
        name = "SetAllowedChainToken",
        abi = "SetAllowedChainToken(bytes32,uint256,bool)"
    )]
    pub struct SetAllowedChainTokenFilter {
        #[ethevent(indexed)]
        pub token_hash: [u8; 32],
        #[ethevent(indexed)]
        pub chain_id: ::ethers::core::types::U256,
        pub allowed: bool,
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
    #[ethevent(name = "SetAllowedSymbol", abi = "SetAllowedSymbol(bytes32,bool)")]
    pub struct SetAllowedSymbolFilter {
        #[ethevent(indexed)]
        pub symbol_hash: [u8; 32],
        pub allowed: bool,
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
    #[ethevent(name = "SetAllowedToken", abi = "SetAllowedToken(bytes32,bool)")]
    pub struct SetAllowedTokenFilter {
        #[ethevent(indexed)]
        pub token_hash: [u8; 32],
        pub allowed: bool,
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
    #[ethevent(name = "SetMaxWithdrawFee", abi = "SetMaxWithdrawFee(bytes32,uint128)")]
    pub struct SetMaxWithdrawFeeFilter {
        #[ethevent(indexed)]
        pub token_hash: [u8; 32],
        pub max_withdraw_fee: u128,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum vault_managerEvents {
        ChangeLedgerFilter(ChangeLedgerFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        RebalanceBurnFilter(RebalanceBurnFilter),
        RebalanceBurnResultFilter(RebalanceBurnResultFilter),
        RebalanceMintFilter(RebalanceMintFilter),
        RebalanceMintResultFilter(RebalanceMintResultFilter),
        SetAllowedBrokerFilter(SetAllowedBrokerFilter),
        SetAllowedChainTokenFilter(SetAllowedChainTokenFilter),
        SetAllowedSymbolFilter(SetAllowedSymbolFilter),
        SetAllowedTokenFilter(SetAllowedTokenFilter),
        SetMaxWithdrawFeeFilter(SetMaxWithdrawFeeFilter),
    }
    impl ::ethers::contract::EthLogDecode for vault_managerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ChangeLedgerFilter::decode_log(log) {
                return Ok(vault_managerEvents::ChangeLedgerFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(vault_managerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(vault_managerEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = RebalanceBurnFilter::decode_log(log) {
                return Ok(vault_managerEvents::RebalanceBurnFilter(decoded));
            }
            if let Ok(decoded) = RebalanceBurnResultFilter::decode_log(log) {
                return Ok(vault_managerEvents::RebalanceBurnResultFilter(decoded));
            }
            if let Ok(decoded) = RebalanceMintFilter::decode_log(log) {
                return Ok(vault_managerEvents::RebalanceMintFilter(decoded));
            }
            if let Ok(decoded) = RebalanceMintResultFilter::decode_log(log) {
                return Ok(vault_managerEvents::RebalanceMintResultFilter(decoded));
            }
            if let Ok(decoded) = SetAllowedBrokerFilter::decode_log(log) {
                return Ok(vault_managerEvents::SetAllowedBrokerFilter(decoded));
            }
            if let Ok(decoded) = SetAllowedChainTokenFilter::decode_log(log) {
                return Ok(vault_managerEvents::SetAllowedChainTokenFilter(decoded));
            }
            if let Ok(decoded) = SetAllowedSymbolFilter::decode_log(log) {
                return Ok(vault_managerEvents::SetAllowedSymbolFilter(decoded));
            }
            if let Ok(decoded) = SetAllowedTokenFilter::decode_log(log) {
                return Ok(vault_managerEvents::SetAllowedTokenFilter(decoded));
            }
            if let Ok(decoded) = SetMaxWithdrawFeeFilter::decode_log(log) {
                return Ok(vault_managerEvents::SetMaxWithdrawFeeFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for vault_managerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ChangeLedgerFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceBurnFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceBurnResultFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceMintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceMintResultFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAllowedBrokerFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAllowedChainTokenFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAllowedSymbolFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAllowedTokenFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMaxWithdrawFeeFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ChangeLedgerFilter> for vault_managerEvents {
        fn from(value: ChangeLedgerFilter) -> Self {
            Self::ChangeLedgerFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for vault_managerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for vault_managerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<RebalanceBurnFilter> for vault_managerEvents {
        fn from(value: RebalanceBurnFilter) -> Self {
            Self::RebalanceBurnFilter(value)
        }
    }
    impl ::core::convert::From<RebalanceBurnResultFilter> for vault_managerEvents {
        fn from(value: RebalanceBurnResultFilter) -> Self {
            Self::RebalanceBurnResultFilter(value)
        }
    }
    impl ::core::convert::From<RebalanceMintFilter> for vault_managerEvents {
        fn from(value: RebalanceMintFilter) -> Self {
            Self::RebalanceMintFilter(value)
        }
    }
    impl ::core::convert::From<RebalanceMintResultFilter> for vault_managerEvents {
        fn from(value: RebalanceMintResultFilter) -> Self {
            Self::RebalanceMintResultFilter(value)
        }
    }
    impl ::core::convert::From<SetAllowedBrokerFilter> for vault_managerEvents {
        fn from(value: SetAllowedBrokerFilter) -> Self {
            Self::SetAllowedBrokerFilter(value)
        }
    }
    impl ::core::convert::From<SetAllowedChainTokenFilter> for vault_managerEvents {
        fn from(value: SetAllowedChainTokenFilter) -> Self {
            Self::SetAllowedChainTokenFilter(value)
        }
    }
    impl ::core::convert::From<SetAllowedSymbolFilter> for vault_managerEvents {
        fn from(value: SetAllowedSymbolFilter) -> Self {
            Self::SetAllowedSymbolFilter(value)
        }
    }
    impl ::core::convert::From<SetAllowedTokenFilter> for vault_managerEvents {
        fn from(value: SetAllowedTokenFilter) -> Self {
            Self::SetAllowedTokenFilter(value)
        }
    }
    impl ::core::convert::From<SetMaxWithdrawFeeFilter> for vault_managerEvents {
        fn from(value: SetMaxWithdrawFeeFilter) -> Self {
            Self::SetMaxWithdrawFeeFilter(value)
        }
    }
    ///Container type for all input parameters for the `addBalance` function with signature `addBalance(bytes32,uint256,uint128)` and selector `0x566b0849`
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
    #[ethcall(name = "addBalance", abi = "addBalance(bytes32,uint256,uint128)")]
    pub struct AddBalanceCall {
        pub token_hash: [u8; 32],
        pub chain_id: ::ethers::core::types::U256,
        pub delta_balance: u128,
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
    ///Container type for all input parameters for the `finishFrozenBalance` function with signature `finishFrozenBalance(bytes32,uint256,uint128)` and selector `0x46fbccaf`
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
        name = "finishFrozenBalance",
        abi = "finishFrozenBalance(bytes32,uint256,uint128)"
    )]
    pub struct FinishFrozenBalanceCall {
        pub token_hash: [u8; 32],
        pub chain_id: ::ethers::core::types::U256,
        pub delta_balance: u128,
    }
    ///Container type for all input parameters for the `frozenBalance` function with signature `frozenBalance(bytes32,uint256,uint128)` and selector `0x7a93a06d`
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
    #[ethcall(name = "frozenBalance", abi = "frozenBalance(bytes32,uint256,uint128)")]
    pub struct FrozenBalanceCall {
        pub token_hash: [u8; 32],
        pub chain_id: ::ethers::core::types::U256,
        pub delta_balance: u128,
    }
    ///Container type for all input parameters for the `getAllAllowedBroker` function with signature `getAllAllowedBroker()` and selector `0xd6aeb431`
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
    #[ethcall(name = "getAllAllowedBroker", abi = "getAllAllowedBroker()")]
    pub struct GetAllAllowedBrokerCall;
    ///Container type for all input parameters for the `getAllAllowedSymbol` function with signature `getAllAllowedSymbol()` and selector `0xb4ab0eca`
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
    #[ethcall(name = "getAllAllowedSymbol", abi = "getAllAllowedSymbol()")]
    pub struct GetAllAllowedSymbolCall;
    ///Container type for all input parameters for the `getAllAllowedToken` function with signature `getAllAllowedToken()` and selector `0x9305a91a`
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
    #[ethcall(name = "getAllAllowedToken", abi = "getAllAllowedToken()")]
    pub struct GetAllAllowedTokenCall;
    ///Container type for all input parameters for the `getAllowedBroker` function with signature `getAllowedBroker(bytes32)` and selector `0x258082f5`
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
    #[ethcall(name = "getAllowedBroker", abi = "getAllowedBroker(bytes32)")]
    pub struct GetAllowedBrokerCall {
        pub broker_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `getAllowedChainToken` function with signature `getAllowedChainToken(bytes32,uint256)` and selector `0xe71a2710`
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
        name = "getAllowedChainToken",
        abi = "getAllowedChainToken(bytes32,uint256)"
    )]
    pub struct GetAllowedChainTokenCall {
        pub token_hash: [u8; 32],
        pub chain_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getAllowedSymbol` function with signature `getAllowedSymbol(bytes32)` and selector `0x0587f413`
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
    #[ethcall(name = "getAllowedSymbol", abi = "getAllowedSymbol(bytes32)")]
    pub struct GetAllowedSymbolCall {
        pub symbol_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `getAllowedToken` function with signature `getAllowedToken(bytes32)` and selector `0xc7eeb9c2`
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
    #[ethcall(name = "getAllowedToken", abi = "getAllowedToken(bytes32)")]
    pub struct GetAllowedTokenCall {
        pub token_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `getBalance` function with signature `getBalance(bytes32,uint256)` and selector `0x9b62377d`
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
    #[ethcall(name = "getBalance", abi = "getBalance(bytes32,uint256)")]
    pub struct GetBalanceCall {
        pub token_hash: [u8; 32],
        pub chain_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getChain2VaultAddress` function with signature `getChain2VaultAddress(uint256)` and selector `0xa546efff`
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
    #[ethcall(name = "getChain2VaultAddress", abi = "getChain2VaultAddress(uint256)")]
    pub struct GetChain2VaultAddressCall {
        pub chain_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getChain2cctpDomain` function with signature `getChain2cctpDomain(uint256)` and selector `0xcddb58e6`
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
    #[ethcall(name = "getChain2cctpDomain", abi = "getChain2cctpDomain(uint256)")]
    pub struct GetChain2CctpDomainCall {
        pub chain_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getFrozenBalance` function with signature `getFrozenBalance(bytes32,uint256)` and selector `0x0a1f5230`
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
    #[ethcall(name = "getFrozenBalance", abi = "getFrozenBalance(bytes32,uint256)")]
    pub struct GetFrozenBalanceCall {
        pub token_hash: [u8; 32],
        pub chain_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getMaxWithdrawFee` function with signature `getMaxWithdrawFee(bytes32)` and selector `0x4945fe16`
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
    #[ethcall(name = "getMaxWithdrawFee", abi = "getMaxWithdrawFee(bytes32)")]
    pub struct GetMaxWithdrawFeeCall {
        pub token_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `getRebalanceStatus` function with signature `getRebalanceStatus(uint64)` and selector `0xde559de0`
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
    #[ethcall(name = "getRebalanceStatus", abi = "getRebalanceStatus(uint64)")]
    pub struct GetRebalanceStatusCall {
        pub rebalance_id: u64,
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
    ///Container type for all input parameters for the `ledgerAddress` function with signature `ledgerAddress()` and selector `0xd1d20056`
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
    #[ethcall(name = "ledgerAddress", abi = "ledgerAddress()")]
    pub struct LedgerAddressCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    ///Container type for all input parameters for the `rebalanceStatus` function with signature `rebalanceStatus(uint64)` and selector `0x287ee68b`
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
    #[ethcall(name = "rebalanceStatus", abi = "rebalanceStatus(uint64)")]
    pub struct RebalanceStatusCall(pub u64);
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setAllowedBroker` function with signature `setAllowedBroker(bytes32,bool)` and selector `0xdf0f4ae7`
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
    #[ethcall(name = "setAllowedBroker", abi = "setAllowedBroker(bytes32,bool)")]
    pub struct SetAllowedBrokerCall {
        pub broker_hash: [u8; 32],
        pub allowed: bool,
    }
    ///Container type for all input parameters for the `setAllowedChainToken` function with signature `setAllowedChainToken(bytes32,uint256,bool)` and selector `0x0b19ccdf`
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
        name = "setAllowedChainToken",
        abi = "setAllowedChainToken(bytes32,uint256,bool)"
    )]
    pub struct SetAllowedChainTokenCall {
        pub token_hash: [u8; 32],
        pub chain_id: ::ethers::core::types::U256,
        pub allowed: bool,
    }
    ///Container type for all input parameters for the `setAllowedSymbol` function with signature `setAllowedSymbol(bytes32,bool)` and selector `0x47b82bd1`
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
    #[ethcall(name = "setAllowedSymbol", abi = "setAllowedSymbol(bytes32,bool)")]
    pub struct SetAllowedSymbolCall {
        pub symbol_hash: [u8; 32],
        pub allowed: bool,
    }
    ///Container type for all input parameters for the `setAllowedToken` function with signature `setAllowedToken(bytes32,bool)` and selector `0xc9fc8797`
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
    #[ethcall(name = "setAllowedToken", abi = "setAllowedToken(bytes32,bool)")]
    pub struct SetAllowedTokenCall {
        pub token_hash: [u8; 32],
        pub allowed: bool,
    }
    ///Container type for all input parameters for the `setChain2VaultAddress` function with signature `setChain2VaultAddress(uint256,address)` and selector `0xd6d186a1`
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
        name = "setChain2VaultAddress",
        abi = "setChain2VaultAddress(uint256,address)"
    )]
    pub struct SetChain2VaultAddressCall {
        pub chain_id: ::ethers::core::types::U256,
        pub vault_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setChain2cctpDomain` function with signature `setChain2cctpDomain(uint256,uint32)` and selector `0xa4564a24`
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
        name = "setChain2cctpDomain",
        abi = "setChain2cctpDomain(uint256,uint32)"
    )]
    pub struct SetChain2CctpDomainCall {
        pub chain_id: ::ethers::core::types::U256,
        pub cctp_domain: u32,
    }
    ///Container type for all input parameters for the `setLedgerAddress` function with signature `setLedgerAddress(address)` and selector `0xba891f5c`
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
    #[ethcall(name = "setLedgerAddress", abi = "setLedgerAddress(address)")]
    pub struct SetLedgerAddressCall {
        pub ledger_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setMaxWithdrawFee` function with signature `setMaxWithdrawFee(bytes32,uint128)` and selector `0xcdf27803`
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
    #[ethcall(name = "setMaxWithdrawFee", abi = "setMaxWithdrawFee(bytes32,uint128)")]
    pub struct SetMaxWithdrawFeeCall {
        pub token_hash: [u8; 32],
        pub max_withdraw_fee: u128,
    }
    ///Container type for all input parameters for the `subBalance` function with signature `subBalance(bytes32,uint256,uint128)` and selector `0xa4f76f2a`
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
    #[ethcall(name = "subBalance", abi = "subBalance(bytes32,uint256,uint128)")]
    pub struct SubBalanceCall {
        pub token_hash: [u8; 32],
        pub chain_id: ::ethers::core::types::U256,
        pub delta_balance: u128,
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
        Hash,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum vault_managerCalls {
        AddBalance(AddBalanceCall),
        ExecuteRebalanceBurn(ExecuteRebalanceBurnCall),
        ExecuteRebalanceMint(ExecuteRebalanceMintCall),
        FinishFrozenBalance(FinishFrozenBalanceCall),
        FrozenBalance(FrozenBalanceCall),
        GetAllAllowedBroker(GetAllAllowedBrokerCall),
        GetAllAllowedSymbol(GetAllAllowedSymbolCall),
        GetAllAllowedToken(GetAllAllowedTokenCall),
        GetAllowedBroker(GetAllowedBrokerCall),
        GetAllowedChainToken(GetAllowedChainTokenCall),
        GetAllowedSymbol(GetAllowedSymbolCall),
        GetAllowedToken(GetAllowedTokenCall),
        GetBalance(GetBalanceCall),
        GetChain2VaultAddress(GetChain2VaultAddressCall),
        GetChain2CctpDomain(GetChain2CctpDomainCall),
        GetFrozenBalance(GetFrozenBalanceCall),
        GetMaxWithdrawFee(GetMaxWithdrawFeeCall),
        GetRebalanceStatus(GetRebalanceStatusCall),
        Initialize(InitializeCall),
        LedgerAddress(LedgerAddressCall),
        Owner(OwnerCall),
        RebalanceBurnFinish(RebalanceBurnFinishCall),
        RebalanceMintFinish(RebalanceMintFinishCall),
        RebalanceStatus(RebalanceStatusCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetAllowedBroker(SetAllowedBrokerCall),
        SetAllowedChainToken(SetAllowedChainTokenCall),
        SetAllowedSymbol(SetAllowedSymbolCall),
        SetAllowedToken(SetAllowedTokenCall),
        SetChain2VaultAddress(SetChain2VaultAddressCall),
        SetChain2CctpDomain(SetChain2CctpDomainCall),
        SetLedgerAddress(SetLedgerAddressCall),
        SetMaxWithdrawFee(SetMaxWithdrawFeeCall),
        SubBalance(SubBalanceCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for vault_managerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddBalance(decoded));
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
                <FinishFrozenBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FinishFrozenBalance(decoded));
            }
            if let Ok(decoded) = <FrozenBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FrozenBalance(decoded));
            }
            if let Ok(decoded) =
                <GetAllAllowedBrokerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAllAllowedBroker(decoded));
            }
            if let Ok(decoded) =
                <GetAllAllowedSymbolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAllAllowedSymbol(decoded));
            }
            if let Ok(decoded) =
                <GetAllAllowedTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAllAllowedToken(decoded));
            }
            if let Ok(decoded) =
                <GetAllowedBrokerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAllowedBroker(decoded));
            }
            if let Ok(decoded) =
                <GetAllowedChainTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAllowedChainToken(decoded));
            }
            if let Ok(decoded) =
                <GetAllowedSymbolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAllowedSymbol(decoded));
            }
            if let Ok(decoded) =
                <GetAllowedTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAllowedToken(decoded));
            }
            if let Ok(decoded) = <GetBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetBalance(decoded));
            }
            if let Ok(decoded) =
                <GetChain2VaultAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetChain2VaultAddress(decoded));
            }
            if let Ok(decoded) =
                <GetChain2CctpDomainCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetChain2CctpDomain(decoded));
            }
            if let Ok(decoded) =
                <GetFrozenBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetFrozenBalance(decoded));
            }
            if let Ok(decoded) =
                <GetMaxWithdrawFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetMaxWithdrawFee(decoded));
            }
            if let Ok(decoded) =
                <GetRebalanceStatusCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRebalanceStatus(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <LedgerAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LedgerAddress(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
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
                <RebalanceStatusCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RebalanceStatus(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetAllowedBrokerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetAllowedBroker(decoded));
            }
            if let Ok(decoded) =
                <SetAllowedChainTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetAllowedChainToken(decoded));
            }
            if let Ok(decoded) =
                <SetAllowedSymbolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetAllowedSymbol(decoded));
            }
            if let Ok(decoded) =
                <SetAllowedTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetAllowedToken(decoded));
            }
            if let Ok(decoded) =
                <SetChain2VaultAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetChain2VaultAddress(decoded));
            }
            if let Ok(decoded) =
                <SetChain2CctpDomainCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetChain2CctpDomain(decoded));
            }
            if let Ok(decoded) =
                <SetLedgerAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetLedgerAddress(decoded));
            }
            if let Ok(decoded) =
                <SetMaxWithdrawFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetMaxWithdrawFee(decoded));
            }
            if let Ok(decoded) = <SubBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SubBalance(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for vault_managerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteRebalanceBurn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteRebalanceMint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FinishFrozenBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FrozenBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAllAllowedBroker(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAllAllowedSymbol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAllAllowedToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAllowedBroker(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAllowedChainToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAllowedSymbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAllowedToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetChain2VaultAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChain2CctpDomain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFrozenBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetMaxWithdrawFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRebalanceStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LedgerAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RebalanceBurnFinish(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RebalanceMintFinish(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RebalanceStatus(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetAllowedBroker(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetAllowedChainToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAllowedSymbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetAllowedToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetChain2VaultAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetChain2CctpDomain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetLedgerAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetMaxWithdrawFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SubBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for vault_managerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteRebalanceBurn(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteRebalanceMint(element) => ::core::fmt::Display::fmt(element, f),
                Self::FinishFrozenBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::FrozenBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllAllowedBroker(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllAllowedSymbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllAllowedToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllowedBroker(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllowedChainToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllowedSymbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllowedToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetChain2VaultAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetChain2CctpDomain(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFrozenBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMaxWithdrawFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRebalanceStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LedgerAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceBurnFinish(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceMintFinish(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAllowedBroker(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAllowedChainToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAllowedSymbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAllowedToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetChain2VaultAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetChain2CctpDomain(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLedgerAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMaxWithdrawFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddBalanceCall> for vault_managerCalls {
        fn from(value: AddBalanceCall) -> Self {
            Self::AddBalance(value)
        }
    }
    impl ::core::convert::From<ExecuteRebalanceBurnCall> for vault_managerCalls {
        fn from(value: ExecuteRebalanceBurnCall) -> Self {
            Self::ExecuteRebalanceBurn(value)
        }
    }
    impl ::core::convert::From<ExecuteRebalanceMintCall> for vault_managerCalls {
        fn from(value: ExecuteRebalanceMintCall) -> Self {
            Self::ExecuteRebalanceMint(value)
        }
    }
    impl ::core::convert::From<FinishFrozenBalanceCall> for vault_managerCalls {
        fn from(value: FinishFrozenBalanceCall) -> Self {
            Self::FinishFrozenBalance(value)
        }
    }
    impl ::core::convert::From<FrozenBalanceCall> for vault_managerCalls {
        fn from(value: FrozenBalanceCall) -> Self {
            Self::FrozenBalance(value)
        }
    }
    impl ::core::convert::From<GetAllAllowedBrokerCall> for vault_managerCalls {
        fn from(value: GetAllAllowedBrokerCall) -> Self {
            Self::GetAllAllowedBroker(value)
        }
    }
    impl ::core::convert::From<GetAllAllowedSymbolCall> for vault_managerCalls {
        fn from(value: GetAllAllowedSymbolCall) -> Self {
            Self::GetAllAllowedSymbol(value)
        }
    }
    impl ::core::convert::From<GetAllAllowedTokenCall> for vault_managerCalls {
        fn from(value: GetAllAllowedTokenCall) -> Self {
            Self::GetAllAllowedToken(value)
        }
    }
    impl ::core::convert::From<GetAllowedBrokerCall> for vault_managerCalls {
        fn from(value: GetAllowedBrokerCall) -> Self {
            Self::GetAllowedBroker(value)
        }
    }
    impl ::core::convert::From<GetAllowedChainTokenCall> for vault_managerCalls {
        fn from(value: GetAllowedChainTokenCall) -> Self {
            Self::GetAllowedChainToken(value)
        }
    }
    impl ::core::convert::From<GetAllowedSymbolCall> for vault_managerCalls {
        fn from(value: GetAllowedSymbolCall) -> Self {
            Self::GetAllowedSymbol(value)
        }
    }
    impl ::core::convert::From<GetAllowedTokenCall> for vault_managerCalls {
        fn from(value: GetAllowedTokenCall) -> Self {
            Self::GetAllowedToken(value)
        }
    }
    impl ::core::convert::From<GetBalanceCall> for vault_managerCalls {
        fn from(value: GetBalanceCall) -> Self {
            Self::GetBalance(value)
        }
    }
    impl ::core::convert::From<GetChain2VaultAddressCall> for vault_managerCalls {
        fn from(value: GetChain2VaultAddressCall) -> Self {
            Self::GetChain2VaultAddress(value)
        }
    }
    impl ::core::convert::From<GetChain2CctpDomainCall> for vault_managerCalls {
        fn from(value: GetChain2CctpDomainCall) -> Self {
            Self::GetChain2CctpDomain(value)
        }
    }
    impl ::core::convert::From<GetFrozenBalanceCall> for vault_managerCalls {
        fn from(value: GetFrozenBalanceCall) -> Self {
            Self::GetFrozenBalance(value)
        }
    }
    impl ::core::convert::From<GetMaxWithdrawFeeCall> for vault_managerCalls {
        fn from(value: GetMaxWithdrawFeeCall) -> Self {
            Self::GetMaxWithdrawFee(value)
        }
    }
    impl ::core::convert::From<GetRebalanceStatusCall> for vault_managerCalls {
        fn from(value: GetRebalanceStatusCall) -> Self {
            Self::GetRebalanceStatus(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for vault_managerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LedgerAddressCall> for vault_managerCalls {
        fn from(value: LedgerAddressCall) -> Self {
            Self::LedgerAddress(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for vault_managerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RebalanceBurnFinishCall> for vault_managerCalls {
        fn from(value: RebalanceBurnFinishCall) -> Self {
            Self::RebalanceBurnFinish(value)
        }
    }
    impl ::core::convert::From<RebalanceMintFinishCall> for vault_managerCalls {
        fn from(value: RebalanceMintFinishCall) -> Self {
            Self::RebalanceMintFinish(value)
        }
    }
    impl ::core::convert::From<RebalanceStatusCall> for vault_managerCalls {
        fn from(value: RebalanceStatusCall) -> Self {
            Self::RebalanceStatus(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for vault_managerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetAllowedBrokerCall> for vault_managerCalls {
        fn from(value: SetAllowedBrokerCall) -> Self {
            Self::SetAllowedBroker(value)
        }
    }
    impl ::core::convert::From<SetAllowedChainTokenCall> for vault_managerCalls {
        fn from(value: SetAllowedChainTokenCall) -> Self {
            Self::SetAllowedChainToken(value)
        }
    }
    impl ::core::convert::From<SetAllowedSymbolCall> for vault_managerCalls {
        fn from(value: SetAllowedSymbolCall) -> Self {
            Self::SetAllowedSymbol(value)
        }
    }
    impl ::core::convert::From<SetAllowedTokenCall> for vault_managerCalls {
        fn from(value: SetAllowedTokenCall) -> Self {
            Self::SetAllowedToken(value)
        }
    }
    impl ::core::convert::From<SetChain2VaultAddressCall> for vault_managerCalls {
        fn from(value: SetChain2VaultAddressCall) -> Self {
            Self::SetChain2VaultAddress(value)
        }
    }
    impl ::core::convert::From<SetChain2CctpDomainCall> for vault_managerCalls {
        fn from(value: SetChain2CctpDomainCall) -> Self {
            Self::SetChain2CctpDomain(value)
        }
    }
    impl ::core::convert::From<SetLedgerAddressCall> for vault_managerCalls {
        fn from(value: SetLedgerAddressCall) -> Self {
            Self::SetLedgerAddress(value)
        }
    }
    impl ::core::convert::From<SetMaxWithdrawFeeCall> for vault_managerCalls {
        fn from(value: SetMaxWithdrawFeeCall) -> Self {
            Self::SetMaxWithdrawFee(value)
        }
    }
    impl ::core::convert::From<SubBalanceCall> for vault_managerCalls {
        fn from(value: SubBalanceCall) -> Self {
            Self::SubBalance(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for vault_managerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `executeRebalanceBurn` function with signature `executeRebalanceBurn((bytes32,bytes32,uint8,uint64,uint128,bytes32,uint256,uint256))` and selector `0xb76c1210`
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
    pub struct ExecuteRebalanceBurnReturn(pub u32, pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getAllAllowedBroker` function with signature `getAllAllowedBroker()` and selector `0xd6aeb431`
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
    pub struct GetAllAllowedBrokerReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `getAllAllowedSymbol` function with signature `getAllAllowedSymbol()` and selector `0xb4ab0eca`
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
    pub struct GetAllAllowedSymbolReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `getAllAllowedToken` function with signature `getAllAllowedToken()` and selector `0x9305a91a`
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
    pub struct GetAllAllowedTokenReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `getAllowedBroker` function with signature `getAllowedBroker(bytes32)` and selector `0x258082f5`
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
    pub struct GetAllowedBrokerReturn(pub bool);
    ///Container type for all return fields from the `getAllowedChainToken` function with signature `getAllowedChainToken(bytes32,uint256)` and selector `0xe71a2710`
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
    pub struct GetAllowedChainTokenReturn(pub bool);
    ///Container type for all return fields from the `getAllowedSymbol` function with signature `getAllowedSymbol(bytes32)` and selector `0x0587f413`
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
    pub struct GetAllowedSymbolReturn(pub bool);
    ///Container type for all return fields from the `getAllowedToken` function with signature `getAllowedToken(bytes32)` and selector `0xc7eeb9c2`
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
    pub struct GetAllowedTokenReturn(pub bool);
    ///Container type for all return fields from the `getBalance` function with signature `getBalance(bytes32,uint256)` and selector `0x9b62377d`
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
    pub struct GetBalanceReturn(pub u128);
    ///Container type for all return fields from the `getChain2VaultAddress` function with signature `getChain2VaultAddress(uint256)` and selector `0xa546efff`
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
    pub struct GetChain2VaultAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getChain2cctpDomain` function with signature `getChain2cctpDomain(uint256)` and selector `0xcddb58e6`
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
    pub struct GetChain2CctpDomainReturn(pub u32);
    ///Container type for all return fields from the `getFrozenBalance` function with signature `getFrozenBalance(bytes32,uint256)` and selector `0x0a1f5230`
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
    pub struct GetFrozenBalanceReturn(pub u128);
    ///Container type for all return fields from the `getMaxWithdrawFee` function with signature `getMaxWithdrawFee(bytes32)` and selector `0x4945fe16`
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
    pub struct GetMaxWithdrawFeeReturn(pub u128);
    ///Container type for all return fields from the `getRebalanceStatus` function with signature `getRebalanceStatus(uint64)` and selector `0xde559de0`
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
    pub struct GetRebalanceStatusReturn(pub RebalanceStatus);
    ///Container type for all return fields from the `ledgerAddress` function with signature `ledgerAddress()` and selector `0xd1d20056`
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
    pub struct LedgerAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `rebalanceStatus` function with signature `rebalanceStatus(uint64)` and selector `0x287ee68b`
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
    pub struct RebalanceStatusReturn {
        pub rebalance_id: u64,
        pub burn_status: u8,
        pub mint_status: u8,
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
        pub src_chain_id: ::ethers::core::types::U256,
        pub dst_chain_id: ::ethers::core::types::U256,
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
        pub src_chain_id: ::ethers::core::types::U256,
        pub dst_chain_id: ::ethers::core::types::U256,
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
        pub src_chain_id: ::ethers::core::types::U256,
        pub dst_chain_id: ::ethers::core::types::U256,
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
        pub src_chain_id: ::ethers::core::types::U256,
        pub dst_chain_id: ::ethers::core::types::U256,
        pub message_bytes: ::ethers::core::types::Bytes,
        pub message_signature: ::ethers::core::types::Bytes,
    }
    ///`RebalanceStatus(uint64,uint8,uint8)`
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
    pub struct RebalanceStatus {
        pub rebalance_id: u64,
        pub burn_status: u8,
        pub mint_status: u8,
    }
}

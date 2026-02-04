pub use market_manager::*;
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
pub mod market_manager {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getPerpMarketCfg"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPerpMarketCfg"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pairSymbol"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct MarketTypes.PerpMarketCfg",
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
                    ::std::borrow::ToOwned::to_owned("ledgerAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ledgerAddress"),
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
                    ::std::borrow::ToOwned::to_owned("perpMarketCfg"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("perpMarketCfg"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "baseMaintenanceMargin",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("baseInitialMargin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidationFeeMax"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("markPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("indexPriceOrderly"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "sumUnitaryFundings",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "lastMarkPriceUpdated",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "lastFundingUpdated",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("setLastFundingUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setLastFundingUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pairSymbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_lastFundingUpdated",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("setLastMarkPriceUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setLastMarkPriceUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pairSymbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_lastMarkPriceUpdated",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("setLedgerAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setLedgerAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ledgerAddress"),
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
                    ::std::borrow::ToOwned::to_owned("setPerpMarketCfg"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPerpMarketCfg"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_symbolHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_perpMarketCfg"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct MarketTypes.PerpMarketCfg",
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
                    ::std::borrow::ToOwned::to_owned("updateMarketUpload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateMarketUpload"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateMarketUpload"),
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
            ]),
            events: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("FundingData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FundingData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("maxTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
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
                    ::std::borrow::ToOwned::to_owned("MarketData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MarketData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("maxTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
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
            ]),
            errors: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("OnlyLedgerCanCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OnlyLedgerCanCall"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MARKET_MANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct market_manager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for market_manager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for market_manager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for market_manager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for market_manager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(market_manager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> market_manager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MARKET_MANAGER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `getPerpMarketCfg` (0x0ea7ba41) function
        pub fn get_perp_market_cfg(
            &self,
            pair_symbol: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, PerpMarketCfg> {
            self.0
                .method_hash([14, 167, 186, 65], pair_symbol)
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
        ///Calls the contract's `operatorManagerAddress` (0x75bf9f6d) function
        pub fn operator_manager_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([117, 191, 159, 109], ())
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
        ///Calls the contract's `perpMarketCfg` (0xe8ee090f) function
        pub fn perp_market_cfg(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u32,
                u32,
                u128,
                u128,
                u128,
                i128,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([232, 238, 9, 15], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLastFundingUpdated` (0xd599a893) function
        pub fn set_last_funding_updated(
            &self,
            pair_symbol: [u8; 32],
            last_funding_updated: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 153, 168, 147], (pair_symbol, last_funding_updated))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLastMarkPriceUpdated` (0x74468fd6) function
        pub fn set_last_mark_price_updated(
            &self,
            pair_symbol: [u8; 32],
            last_mark_price_updated: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 70, 143, 214], (pair_symbol, last_mark_price_updated))
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
        ///Calls the contract's `setOperatorManagerAddress` (0xde0c9c86) function
        pub fn set_operator_manager_address(
            &self,
            operator_manager_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 12, 156, 134], operator_manager_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPerpMarketCfg` (0x0d52b637) function
        pub fn set_perp_market_cfg(
            &self,
            symbol_hash: [u8; 32],
            perp_market_cfg: PerpMarketCfg,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 82, 182, 55], (symbol_hash, perp_market_cfg))
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
        ///Calls the contract's `updateMarketUpload` (0x121e5767) function
        pub fn update_market_upload(
            &self,
            data: UploadPerpPrice,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 30, 87, 103], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMarketUpload` (0x793aac69) function
        pub fn update_market_upload_with_data(
            &self,
            data: UploadPerpPrice,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 58, 172, 105], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ChangeLedger` event
        pub fn change_ledger_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ChangeLedgerFilter>
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
        ///Gets the contract's `FundingData` event
        pub fn funding_data_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FundingDataFilter>
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
        ///Gets the contract's `MarketData` event
        pub fn market_data_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MarketDataFilter> {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, market_managerEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for market_manager<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum market_managerErrors {
        LedgerAddressZero(LedgerAddressZero),
        OnlyLedgerCanCall(OnlyLedgerCanCall),
        OnlyOperatorManagerCanCall(OnlyOperatorManagerCanCall),
        OperatorManagerAddressZero(OperatorManagerAddressZero),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for market_managerErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
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
                <OnlyOperatorManagerCanCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyOperatorManagerCanCall(decoded));
            }
            if let Ok(decoded) =
                <OperatorManagerAddressZero as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorManagerAddressZero(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for market_managerErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::LedgerAddressZero(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyLedgerCanCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyOperatorManagerCanCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorManagerAddressZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for market_managerErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
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
                    == <OnlyOperatorManagerCanCall as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <OperatorManagerAddressZero as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for market_managerErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LedgerAddressZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyLedgerCanCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyOperatorManagerCanCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorManagerAddressZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for market_managerErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<LedgerAddressZero> for market_managerErrors {
        fn from(value: LedgerAddressZero) -> Self {
            Self::LedgerAddressZero(value)
        }
    }
    impl ::core::convert::From<OnlyLedgerCanCall> for market_managerErrors {
        fn from(value: OnlyLedgerCanCall) -> Self {
            Self::OnlyLedgerCanCall(value)
        }
    }
    impl ::core::convert::From<OnlyOperatorManagerCanCall> for market_managerErrors {
        fn from(value: OnlyOperatorManagerCanCall) -> Self {
            Self::OnlyOperatorManagerCanCall(value)
        }
    }
    impl ::core::convert::From<OperatorManagerAddressZero> for market_managerErrors {
        fn from(value: OperatorManagerAddressZero) -> Self {
            Self::OperatorManagerAddressZero(value)
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
    #[ethevent(name = "FundingData", abi = "FundingData(uint64)")]
    pub struct FundingDataFilter {
        pub max_timestamp: u64,
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
    #[ethevent(name = "MarketData", abi = "MarketData(uint64)")]
    pub struct MarketDataFilter {
        pub max_timestamp: u64,
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
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum market_managerEvents {
        ChangeLedgerFilter(ChangeLedgerFilter),
        ChangeOperatorManagerFilter(ChangeOperatorManagerFilter),
        FundingDataFilter(FundingDataFilter),
        InitializedFilter(InitializedFilter),
        MarketDataFilter(MarketDataFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for market_managerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ChangeLedgerFilter::decode_log(log) {
                return Ok(market_managerEvents::ChangeLedgerFilter(decoded));
            }
            if let Ok(decoded) = ChangeOperatorManagerFilter::decode_log(log) {
                return Ok(market_managerEvents::ChangeOperatorManagerFilter(decoded));
            }
            if let Ok(decoded) = FundingDataFilter::decode_log(log) {
                return Ok(market_managerEvents::FundingDataFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(market_managerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MarketDataFilter::decode_log(log) {
                return Ok(market_managerEvents::MarketDataFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(market_managerEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for market_managerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ChangeLedgerFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeOperatorManagerFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FundingDataFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MarketDataFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ChangeLedgerFilter> for market_managerEvents {
        fn from(value: ChangeLedgerFilter) -> Self {
            Self::ChangeLedgerFilter(value)
        }
    }
    impl ::core::convert::From<ChangeOperatorManagerFilter> for market_managerEvents {
        fn from(value: ChangeOperatorManagerFilter) -> Self {
            Self::ChangeOperatorManagerFilter(value)
        }
    }
    impl ::core::convert::From<FundingDataFilter> for market_managerEvents {
        fn from(value: FundingDataFilter) -> Self {
            Self::FundingDataFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for market_managerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<MarketDataFilter> for market_managerEvents {
        fn from(value: MarketDataFilter) -> Self {
            Self::MarketDataFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for market_managerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `getPerpMarketCfg` function with signature `getPerpMarketCfg(bytes32)` and selector `0x0ea7ba41`
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
    #[ethcall(name = "getPerpMarketCfg", abi = "getPerpMarketCfg(bytes32)")]
    pub struct GetPerpMarketCfgCall {
        pub pair_symbol: [u8; 32],
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
    ///Container type for all input parameters for the `operatorManagerAddress` function with signature `operatorManagerAddress()` and selector `0x75bf9f6d`
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
        Hash,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `perpMarketCfg` function with signature `perpMarketCfg(bytes32)` and selector `0xe8ee090f`
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
    #[ethcall(name = "perpMarketCfg", abi = "perpMarketCfg(bytes32)")]
    pub struct PerpMarketCfgCall(pub [u8; 32]);
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
    ///Container type for all input parameters for the `setLastFundingUpdated` function with signature `setLastFundingUpdated(bytes32,uint64)` and selector `0xd599a893`
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
        name = "setLastFundingUpdated",
        abi = "setLastFundingUpdated(bytes32,uint64)"
    )]
    pub struct SetLastFundingUpdatedCall {
        pub pair_symbol: [u8; 32],
        pub last_funding_updated: u64,
    }
    ///Container type for all input parameters for the `setLastMarkPriceUpdated` function with signature `setLastMarkPriceUpdated(bytes32,uint64)` and selector `0x74468fd6`
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
        name = "setLastMarkPriceUpdated",
        abi = "setLastMarkPriceUpdated(bytes32,uint64)"
    )]
    pub struct SetLastMarkPriceUpdatedCall {
        pub pair_symbol: [u8; 32],
        pub last_mark_price_updated: u64,
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
    ///Container type for all input parameters for the `setPerpMarketCfg` function with signature `setPerpMarketCfg(bytes32,(uint32,uint32,uint128,uint128,uint128,int128,uint256,uint256))` and selector `0x0d52b637`
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
        name = "setPerpMarketCfg",
        abi = "setPerpMarketCfg(bytes32,(uint32,uint32,uint128,uint128,uint128,int128,uint256,uint256))"
    )]
    pub struct SetPerpMarketCfgCall {
        pub symbol_hash: [u8; 32],
        pub perp_market_cfg: PerpMarketCfg,
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
    ///Container type for all input parameters for the `updateMarketUpload` function with signature `updateMarketUpload((bytes32,bytes32,uint8,uint64,(bytes32,int128,uint64)[]))` and selector `0x121e5767`
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
        name = "updateMarketUpload",
        abi = "updateMarketUpload((bytes32,bytes32,uint8,uint64,(bytes32,int128,uint64)[]))"
    )]
    pub struct UpdateMarketUploadCall {
        pub data: UploadPerpPrice,
    }
    ///Container type for all input parameters for the `updateMarketUpload` function with signature `updateMarketUpload((bytes32,bytes32,uint8,uint64,(bytes32,uint128,uint128,uint64)[]))` and selector `0x793aac69`
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
        name = "updateMarketUpload",
        abi = "updateMarketUpload((bytes32,bytes32,uint8,uint64,(bytes32,uint128,uint128,uint64)[]))"
    )]
    pub struct UpdateMarketUploadWithDataCall {
        pub data: UploadPerpPrice,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum market_managerCalls {
        GetPerpMarketCfg(GetPerpMarketCfgCall),
        Initialize(InitializeCall),
        LedgerAddress(LedgerAddressCall),
        OperatorManagerAddress(OperatorManagerAddressCall),
        Owner(OwnerCall),
        PerpMarketCfg(PerpMarketCfgCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetLastFundingUpdated(SetLastFundingUpdatedCall),
        SetLastMarkPriceUpdated(SetLastMarkPriceUpdatedCall),
        SetLedgerAddress(SetLedgerAddressCall),
        SetOperatorManagerAddress(SetOperatorManagerAddressCall),
        SetPerpMarketCfg(SetPerpMarketCfgCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateMarketUpload(UpdateMarketUploadCall),
        UpdateMarketUploadWithData(UpdateMarketUploadWithDataCall),
    }
    impl ::ethers::core::abi::AbiDecode for market_managerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <GetPerpMarketCfgCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPerpMarketCfg(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <LedgerAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LedgerAddress(decoded));
            }
            if let Ok(decoded) =
                <OperatorManagerAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorManagerAddress(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PerpMarketCfgCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PerpMarketCfg(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetLastFundingUpdatedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetLastFundingUpdated(decoded));
            }
            if let Ok(decoded) =
                <SetLastMarkPriceUpdatedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetLastMarkPriceUpdated(decoded));
            }
            if let Ok(decoded) =
                <SetLedgerAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetLedgerAddress(decoded));
            }
            if let Ok(decoded) =
                <SetOperatorManagerAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetOperatorManagerAddress(decoded));
            }
            if let Ok(decoded) =
                <SetPerpMarketCfgCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetPerpMarketCfg(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UpdateMarketUploadCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateMarketUpload(decoded));
            }
            if let Ok(decoded) =
                <UpdateMarketUploadWithDataCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateMarketUploadWithData(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for market_managerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetPerpMarketCfg(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LedgerAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OperatorManagerAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PerpMarketCfg(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetLastFundingUpdated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetLastMarkPriceUpdated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetLedgerAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetOperatorManagerAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPerpMarketCfg(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateMarketUpload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateMarketUploadWithData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for market_managerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetPerpMarketCfg(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LedgerAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorManagerAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PerpMarketCfg(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLastFundingUpdated(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLastMarkPriceUpdated(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLedgerAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOperatorManagerAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPerpMarketCfg(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateMarketUpload(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateMarketUploadWithData(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetPerpMarketCfgCall> for market_managerCalls {
        fn from(value: GetPerpMarketCfgCall) -> Self {
            Self::GetPerpMarketCfg(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for market_managerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LedgerAddressCall> for market_managerCalls {
        fn from(value: LedgerAddressCall) -> Self {
            Self::LedgerAddress(value)
        }
    }
    impl ::core::convert::From<OperatorManagerAddressCall> for market_managerCalls {
        fn from(value: OperatorManagerAddressCall) -> Self {
            Self::OperatorManagerAddress(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for market_managerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PerpMarketCfgCall> for market_managerCalls {
        fn from(value: PerpMarketCfgCall) -> Self {
            Self::PerpMarketCfg(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for market_managerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetLastFundingUpdatedCall> for market_managerCalls {
        fn from(value: SetLastFundingUpdatedCall) -> Self {
            Self::SetLastFundingUpdated(value)
        }
    }
    impl ::core::convert::From<SetLastMarkPriceUpdatedCall> for market_managerCalls {
        fn from(value: SetLastMarkPriceUpdatedCall) -> Self {
            Self::SetLastMarkPriceUpdated(value)
        }
    }
    impl ::core::convert::From<SetLedgerAddressCall> for market_managerCalls {
        fn from(value: SetLedgerAddressCall) -> Self {
            Self::SetLedgerAddress(value)
        }
    }
    impl ::core::convert::From<SetOperatorManagerAddressCall> for market_managerCalls {
        fn from(value: SetOperatorManagerAddressCall) -> Self {
            Self::SetOperatorManagerAddress(value)
        }
    }
    impl ::core::convert::From<SetPerpMarketCfgCall> for market_managerCalls {
        fn from(value: SetPerpMarketCfgCall) -> Self {
            Self::SetPerpMarketCfg(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for market_managerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpdateMarketUploadCall> for market_managerCalls {
        fn from(value: UpdateMarketUploadCall) -> Self {
            Self::UpdateMarketUpload(value)
        }
    }
    impl ::core::convert::From<UpdateMarketUploadWithDataCall> for market_managerCalls {
        fn from(value: UpdateMarketUploadWithDataCall) -> Self {
            Self::UpdateMarketUploadWithData(value)
        }
    }
    ///Container type for all return fields from the `getPerpMarketCfg` function with signature `getPerpMarketCfg(bytes32)` and selector `0x0ea7ba41`
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
    pub struct GetPerpMarketCfgReturn(pub PerpMarketCfg);
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
    ///Container type for all return fields from the `operatorManagerAddress` function with signature `operatorManagerAddress()` and selector `0x75bf9f6d`
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
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `perpMarketCfg` function with signature `perpMarketCfg(bytes32)` and selector `0xe8ee090f`
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
    pub struct PerpMarketCfgReturn {
        pub base_maintenance_margin: u32,
        pub base_initial_margin: u32,
        pub liquidation_fee_max: u128,
        pub mark_price: u128,
        pub index_price_orderly: u128,
        pub sum_unitary_fundings: i128,
        pub last_mark_price_updated: ::ethers::core::types::U256,
        pub last_funding_updated: ::ethers::core::types::U256,
    }
    ///`PerpMarketCfg(uint32,uint32,uint128,uint128,uint128,int128,uint256,uint256)`
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
    pub struct PerpMarketCfg {
        pub base_maintenance_margin: u32,
        pub base_initial_margin: u32,
        pub liquidation_fee_max: u128,
        pub mark_price: u128,
        pub index_price_orderly: u128,
        pub sum_unitary_fundings: i128,
        pub last_mark_price_updated: ::ethers::core::types::U256,
        pub last_funding_updated: ::ethers::core::types::U256,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct UploadSumUnitaryFundings {
        pub r: [u8; 32],
        pub s: [u8; 32],
        pub v: u8,
        pub max_timestamp: u64,
        pub sum_unitary_fundings: ::std::vec::Vec<SumUnitaryFunding>,
    }
}

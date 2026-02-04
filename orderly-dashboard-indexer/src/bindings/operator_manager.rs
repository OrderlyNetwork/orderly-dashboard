pub use operator_manager::*;
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
pub mod operator_manager {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("bizTypeToSelectors"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("bizTypeToSelectors"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes4"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkEngineDown"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("checkEngineDown"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("engineEventUploadAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("engineEventUploadAddress",),
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
                    ::std::borrow::ToOwned::to_owned("engineMarketUploadAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("engineMarketUploadAddress",),
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
                    ::std::borrow::ToOwned::to_owned("enginePerpTradeUploadAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("enginePerpTradeUploadAddress",),
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
                    ::std::borrow::ToOwned::to_owned("engineRebalanceUploadAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("engineRebalanceUploadAddress",),
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
                    ::std::borrow::ToOwned::to_owned("engineSpotTradeUploadAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("engineSpotTradeUploadAddress",),
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
                    ::std::borrow::ToOwned::to_owned("eventUpload"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("eventUpload"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("data"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct EventTypes.EventUpload",),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("eventUploadBatchId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("eventUploadBatchId"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("futuresTradeUpload"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("futuresTradeUpload"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("data"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct PerpTypes.FuturesTradeUploadData",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("futuresTradeUploadV3"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("futuresTradeUploadV3",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("data"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ],),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct PerpTypes.FuturesTradeUploadDataV3",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("futuresUploadBatchId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("futuresUploadBatchId",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOperatorManagerImpl"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOperatorManagerImpl",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initBizTypeToSelector"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initBizTypeToSelector",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("lastOperatorInteraction"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lastOperatorInteraction",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ledger"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("ledger"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract ILedger"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("marketManager"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("marketManager"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IMarketManager"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operatorAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("operatorAddress"),
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
                    ::std::borrow::ToOwned::to_owned("operatorManagerZipAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("operatorManagerZipAddress",),
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
                    ::std::borrow::ToOwned::to_owned("operatorPing"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("operatorPing"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("perpPriceUpload"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("perpPriceUpload"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("data"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct MarketTypes.UploadPerpPrice",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rebalanceBurnUpload"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rebalanceBurnUpload",),
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
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rebalanceMintUpload"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rebalanceMintUpload",),
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
                    ::std::borrow::ToOwned::to_owned("setEngineEventUploadAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setEngineEventUploadAddress",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_engineEventUploadAddress",),
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
                    ::std::borrow::ToOwned::to_owned("setEngineMarketUploadAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setEngineMarketUploadAddress",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_engineMarketUploadAddress",),
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
                    ::std::borrow::ToOwned::to_owned("setEnginePerpTradeUploadAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setEnginePerpTradeUploadAddress",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_enginePerpTradeUploadAddress",),
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
                    ::std::borrow::ToOwned::to_owned("setEngineRebalanceUploadAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setEngineRebalanceUploadAddress",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_engineRebalanceUploadAddress",),
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
                    ::std::borrow::ToOwned::to_owned("setEngineSpotTradeUploadAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setEngineSpotTradeUploadAddress",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_engineSpotTradeUploadAddress",),
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
                    ::std::borrow::ToOwned::to_owned("setLedger"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setLedger"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_ledger"),
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
                    ::std::borrow::ToOwned::to_owned("setMarketManager"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setMarketManager"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_marketManagerAddress",),
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
                    ::std::borrow::ToOwned::to_owned("setOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setOperator"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_operatorAddress"),
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
                    ::std::borrow::ToOwned::to_owned("setOperatorManagerImplA"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setOperatorManagerImplA",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_operatorManagerImplA",),
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
                    ::std::borrow::ToOwned::to_owned("setOperatorManagerImplB"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setOperatorManagerImplB",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_operatorManagerImplB",),
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
                    ::std::borrow::ToOwned::to_owned("setOperatorManagerZipAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setOperatorManagerZipAddress",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_operatorManagerZipAddress",),
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
                    ::std::borrow::ToOwned::to_owned("sumUnitaryFundingsUpload"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("sumUnitaryFundingsUpload",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("data"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct MarketTypes.UploadSumUnitaryFundings",
                                ),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("ChangeEngineUpload"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ChangeEngineUpload"),
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
                    },],
                ),
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
                    ::std::borrow::ToOwned::to_owned("ChangeMarketManager"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ChangeMarketManager",),
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
                    ::std::borrow::ToOwned::to_owned("ChangeOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ChangeOperatorImplA"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ChangeOperatorImplA",),
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
                    ::std::borrow::ToOwned::to_owned("ChangeOperatorImplB"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ChangeOperatorImplB",),
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
                    ::std::borrow::ToOwned::to_owned("EventUpload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EventUpload"),
                            inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("batchId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: true,
                            },],
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
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("blocktime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    indexed: false,
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
                            inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("batchId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: true,
                            },],
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
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("blocktime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FuturesTradeUploadV3"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("FuturesTradeUploadV3",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("batchId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            indexed: true,
                        },],
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
                    ::std::borrow::ToOwned::to_owned("RebalanceBurnUpload"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RebalanceBurnUpload",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("rebalanceId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RebalanceMintUpload"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RebalanceMintUpload",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("rebalanceId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AccountIdInvalid"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AccountIdInvalid"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressZero"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AddressZero"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BalanceNotEnough"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BalanceNotEnough"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BatchIdNotMatch"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
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
                                name: ::std::borrow::ToOwned::to_owned("futuresUploadBatchId",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BrokerNotAllowed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BrokerNotAllowed"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Bytes32Zero"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Bytes32Zero"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CountNotMatch"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CountNotMatch"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("length"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("count"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DelegateChainIdNotMatch"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DelegateChainIdNotMatch",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("accountId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("savedChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("givenChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DelegateReceiverNotMatch"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DelegateReceiverNotMatch",),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DelegateSignerNotMatch"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DelegateSignerNotMatch",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("accountId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DelegatecallFail"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DelegatecallFail"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnumerableSetError"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("EnumerableSetError"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FrozenBalanceInconsistent"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FrozenBalanceInconsistent",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsurancePositionQtyInvalid"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InsurancePositionQtyInvalid",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("adlPositionQtyTransfer",),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsuranceTransferAmountInvalid"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InsuranceTransferAmountInvalid",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("insuranceTransferAmount",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsuranceTransferToSelf"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InsuranceTransferToSelf",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidBizType"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidBizType"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("bizType"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidFeeCollectorType"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidFeeCollectorType",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMarginMode"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidMarginMode"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("marginMode"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidPrimeWallet"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidPrimeWallet"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidVault"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidVault"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IsoAdlMarginToCrossAmountInvalid"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("IsoAdlMarginToCrossAmountInvalid",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("isoMarginToCross"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int128"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("MarginTransferV3AmountInvalid"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("MarginTransferV3AmountInvalid",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("marginFromCross"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int128"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotImplemented"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotImplemented"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyCrossChainManagerCanCall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyCrossChainManagerCanCall",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyCrossChainManagerV2CanCall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyCrossChainManagerV2CanCall",),
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
                    ::std::borrow::ToOwned::to_owned("OnlyOperatorCanCall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyOperatorCanCall",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyOperatorManagerCanCall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyOperatorManagerCanCall",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlySymbolManagerOrOwner"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlySymbolManagerOrOwner",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorManagerAddressZero"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OperatorManagerAddressZero",),
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
                    ::std::borrow::ToOwned::to_owned("RebalanceChainIdInvalid"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("RebalanceChainIdInvalid",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("chainId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
                (
                    ::std::borrow::ToOwned::to_owned("RebalanceTokenNotSupported"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("RebalanceTokenNotSupported",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("chainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeCastOverflow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SafeCastOverflow"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeCastUnderflow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SafeCastUnderflow"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SignatureNotMatch"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SignatureNotMatch"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SymbolNotAllowed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SymbolNotAllowed"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SymbolNotRegister"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SymbolNotRegister"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenNotAllowed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TokenNotAllowed"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("chainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TotalSettleAmountNotMatch"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TotalSettleAmountNotMatch",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int128"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnsupportChainType"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("UnsupportChainType"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UserPerpPositionQtyZero"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("UserPerpPositionQtyZero",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("accountId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("symbolHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawBalanceNotEnough"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("WithdrawBalanceNotEnough",),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawFeeTooLarge"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("WithdrawFeeTooLarge",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("maxFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("withdrawFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawToAddressZero"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("WithdrawToAddressZero",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawVaultBalanceNotEnough"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("WithdrawVaultBalanceNotEnough",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("withdrawAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroChainId"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ZeroChainId"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroDelegateContract"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ZeroDelegateContract",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroDelegateSigner"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ZeroDelegateSigner"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static OPERATOR_MANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
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
            Self(::ethers::contract::Contract::new(
                address.into(),
                OPERATOR_MANAGER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `bizTypeToSelectors` (0xb9847b92) function
        pub fn biz_type_to_selectors(
            &self,
            p0: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([185, 132, 123, 146], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkEngineDown` (0x0bc70960) function
        pub fn check_engine_down(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([11, 199, 9, 96], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `engineEventUploadAddress` (0xc553cd84) function
        pub fn engine_event_upload_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([197, 83, 205, 132], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `engineMarketUploadAddress` (0x895f9313) function
        pub fn engine_market_upload_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([137, 95, 147, 19], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enginePerpTradeUploadAddress` (0xe02a7d8d) function
        pub fn engine_perp_trade_upload_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([224, 42, 125, 141], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `engineRebalanceUploadAddress` (0xc97d4f5a) function
        pub fn engine_rebalance_upload_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([201, 125, 79, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `engineSpotTradeUploadAddress` (0x407d3915) function
        pub fn engine_spot_trade_upload_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([64, 125, 57, 21], ())
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
        ///Calls the contract's `eventUploadBatchId` (0xd4b6de87) function
        pub fn event_upload_batch_id(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([212, 182, 222, 135], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `futuresTradeUpload` (0xdba9ab14) function
        pub fn futures_trade_upload(
            &self,
            data: FuturesTradeUploadData,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 169, 171, 20], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `futuresTradeUploadV3` (0x6d201adf) function
        pub fn futures_trade_upload_v3(
            &self,
            data: FuturesTradeUploadDataV3,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 32, 26, 223], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `futuresUploadBatchId` (0x89ac77c5) function
        pub fn futures_upload_batch_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([137, 172, 119, 197], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperatorManagerImpl` (0x70664d30) function
        pub fn get_operator_manager_impl(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([112, 102, 77, 48], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initBizTypeToSelector` (0x466cc876) function
        pub fn init_biz_type_to_selector(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 108, 200, 118], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x8129fc1c) function
        pub fn initialize(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastOperatorInteraction` (0xfa8123c0) function
        pub fn last_operator_interaction(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([250, 129, 35, 192], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ledger` (0x56397c35) function
        pub fn ledger(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([86, 57, 124, 53], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `marketManager` (0x41ed2c12) function
        pub fn market_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([65, 237, 44, 18], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorAddress` (0x127effb2) function
        pub fn operator_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([18, 126, 255, 178], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorManagerZipAddress` (0xf92b162b) function
        pub fn operator_manager_zip_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([249, 43, 22, 43], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorPing` (0x6ecb9886) function
        pub fn operator_ping(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 203, 152, 134], ())
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
        ///Calls the contract's `perpPriceUpload` (0x24ce6299) function
        pub fn perp_price_upload(
            &self,
            data: UploadPerpPrice,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 206, 98, 153], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rebalanceBurnUpload` (0x7071c2df) function
        pub fn rebalance_burn_upload(
            &self,
            data: RebalanceBurnUploadData,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 113, 194, 223], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rebalanceMintUpload` (0xc70a68ff) function
        pub fn rebalance_mint_upload(
            &self,
            data: RebalanceMintUploadData,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([199, 10, 104, 255], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEngineEventUploadAddress` (0x3e172194) function
        pub fn set_engine_event_upload_address(
            &self,
            engine_event_upload_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([62, 23, 33, 148], engine_event_upload_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEngineMarketUploadAddress` (0x681d4e34) function
        pub fn set_engine_market_upload_address(
            &self,
            engine_market_upload_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 29, 78, 52], engine_market_upload_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEnginePerpTradeUploadAddress` (0xb5f552a0) function
        pub fn set_engine_perp_trade_upload_address(
            &self,
            engine_perp_trade_upload_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 245, 82, 160], engine_perp_trade_upload_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEngineRebalanceUploadAddress` (0x314c3157) function
        pub fn set_engine_rebalance_upload_address(
            &self,
            engine_rebalance_upload_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 76, 49, 87], engine_rebalance_upload_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEngineSpotTradeUploadAddress` (0x7e2ae251) function
        pub fn set_engine_spot_trade_upload_address(
            &self,
            engine_spot_trade_upload_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([126, 42, 226, 81], engine_spot_trade_upload_address)
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
        ///Calls the contract's `setOperatorManagerImplA` (0xb314b59f) function
        pub fn set_operator_manager_impl_a(
            &self,
            operator_manager_impl_a: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([179, 20, 181, 159], operator_manager_impl_a)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOperatorManagerImplB` (0x68a1bab8) function
        pub fn set_operator_manager_impl_b(
            &self,
            operator_manager_impl_b: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 161, 186, 184], operator_manager_impl_b)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOperatorManagerZipAddress` (0x892021dc) function
        pub fn set_operator_manager_zip_address(
            &self,
            operator_manager_zip_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 32, 33, 220], operator_manager_zip_address)
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
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ChangeEngineUpload` event
        pub fn change_engine_upload_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ChangeEngineUploadFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ChangeLedger` event
        pub fn change_ledger_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ChangeLedgerFilter>
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
        ///Gets the contract's `ChangeOperator` event
        pub fn change_operator_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ChangeOperatorFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ChangeOperatorImplA` event
        pub fn change_operator_impl_a_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ChangeOperatorImplAFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ChangeOperatorImplB` event
        pub fn change_operator_impl_b_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ChangeOperatorImplBFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `EventUpload` event
        pub fn event_upload_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EventUpload1Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `EventUpload` event
        pub fn event_upload_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EventUpload2Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `FuturesTradeUpload` event
        pub fn futures_trade_upload_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FuturesTradeUpload1Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `FuturesTradeUpload` event
        pub fn futures_trade_upload_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FuturesTradeUpload2Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `FuturesTradeUploadV3` event
        pub fn futures_trade_upload_v3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FuturesTradeUploadV3Filter>
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
        ///Gets the contract's `RebalanceBurnUpload` event
        pub fn rebalance_burn_upload_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RebalanceBurnUploadFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RebalanceMintUpload` event
        pub fn rebalance_mint_upload_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RebalanceMintUploadFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, operator_managerEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for operator_manager<M>
    {
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
    ///Custom Error type `InvalidMarginMode` with signature `InvalidMarginMode(uint8)` and selector `0x64cdbd97`
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
    #[etherror(name = "InvalidMarginMode", abi = "InvalidMarginMode(uint8)")]
    pub struct InvalidMarginMode {
        pub margin_mode: u8,
    }
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
    ///Custom Error type `InvalidVault` with signature `InvalidVault()` and selector `0xd03a6320`
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
    #[etherror(name = "InvalidVault", abi = "InvalidVault()")]
    pub struct InvalidVault;
    ///Custom Error type `IsoAdlMarginToCrossAmountInvalid` with signature `IsoAdlMarginToCrossAmountInvalid(int128)` and selector `0xb0b72bc2`
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
        name = "IsoAdlMarginToCrossAmountInvalid",
        abi = "IsoAdlMarginToCrossAmountInvalid(int128)"
    )]
    pub struct IsoAdlMarginToCrossAmountInvalid {
        pub iso_margin_to_cross: i128,
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
    ///Custom Error type `MarginTransferV3AmountInvalid` with signature `MarginTransferV3AmountInvalid(int128)` and selector `0x43d0f72f`
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
        name = "MarginTransferV3AmountInvalid",
        abi = "MarginTransferV3AmountInvalid(int128)"
    )]
    pub struct MarginTransferV3AmountInvalid {
        pub margin_from_cross: i128,
    }
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
    ///Custom Error type `UnsupportChainType` with signature `UnsupportChainType()` and selector `0x5a8a5514`
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
    #[etherror(name = "UnsupportChainType", abi = "UnsupportChainType()")]
    pub struct UnsupportChainType;
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
    pub enum operator_managerErrors {
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
        InvalidMarginMode(InvalidMarginMode),
        InvalidPrimeWallet(InvalidPrimeWallet),
        InvalidVault(InvalidVault),
        IsoAdlMarginToCrossAmountInvalid(IsoAdlMarginToCrossAmountInvalid),
        LedgerAddressZero(LedgerAddressZero),
        MarginTransferV3AmountInvalid(MarginTransferV3AmountInvalid),
        NotImplemented(NotImplemented),
        OnlyCrossChainManagerCanCall(OnlyCrossChainManagerCanCall),
        OnlyCrossChainManagerV2CanCall(OnlyCrossChainManagerV2CanCall),
        OnlyLedgerCanCall(OnlyLedgerCanCall),
        OnlyOperatorCanCall(OnlyOperatorCanCall),
        OnlyOperatorManagerCanCall(OnlyOperatorManagerCanCall),
        OnlySymbolManagerOrOwner(OnlySymbolManagerOrOwner),
        OperatorManagerAddressZero(OperatorManagerAddressZero),
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
        UnsupportChainType(UnsupportChainType),
        UserPerpPositionQtyZero(UserPerpPositionQtyZero),
        WithdrawBalanceNotEnough(WithdrawBalanceNotEnough),
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
    impl ::ethers::core::abi::AbiDecode for operator_managerErrors {
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
            if let Ok(decoded) = <InvalidMarginMode as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidMarginMode(decoded));
            }
            if let Ok(decoded) =
                <InvalidPrimeWallet as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidPrimeWallet(decoded));
            }
            if let Ok(decoded) = <InvalidVault as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidVault(decoded));
            }
            if let Ok(decoded) =
                <IsoAdlMarginToCrossAmountInvalid as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsoAdlMarginToCrossAmountInvalid(decoded));
            }
            if let Ok(decoded) = <LedgerAddressZero as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LedgerAddressZero(decoded));
            }
            if let Ok(decoded) =
                <MarginTransferV3AmountInvalid as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MarginTransferV3AmountInvalid(decoded));
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
                <UnsupportChainType as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsupportChainType(decoded));
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
    impl ::ethers::core::abi::AbiEncode for operator_managerErrors {
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
                Self::InvalidMarginMode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidPrimeWallet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidVault(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsoAdlMarginToCrossAmountInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LedgerAddressZero(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MarginTransferV3AmountInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::UnsupportChainType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UserPerpPositionQtyZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawBalanceNotEnough(element) => {
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
                    == <InvalidMarginMode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidPrimeWallet as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidVault as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <IsoAdlMarginToCrossAmountInvalid as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <LedgerAddressZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MarginTransferV3AmountInvalid as ::ethers::contract::EthError>::selector() => {
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
                    == <UnsupportChainType as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for operator_managerErrors {
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
                Self::InvalidMarginMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidPrimeWallet(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidVault(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsoAdlMarginToCrossAmountInvalid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LedgerAddressZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::MarginTransferV3AmountInvalid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
                Self::UnsupportChainType(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserPerpPositionQtyZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawBalanceNotEnough(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<Bytes32Zero> for operator_managerErrors {
        fn from(value: Bytes32Zero) -> Self {
            Self::Bytes32Zero(value)
        }
    }
    impl ::core::convert::From<CountNotMatch> for operator_managerErrors {
        fn from(value: CountNotMatch) -> Self {
            Self::CountNotMatch(value)
        }
    }
    impl ::core::convert::From<DelegateChainIdNotMatch> for operator_managerErrors {
        fn from(value: DelegateChainIdNotMatch) -> Self {
            Self::DelegateChainIdNotMatch(value)
        }
    }
    impl ::core::convert::From<DelegateReceiverNotMatch> for operator_managerErrors {
        fn from(value: DelegateReceiverNotMatch) -> Self {
            Self::DelegateReceiverNotMatch(value)
        }
    }
    impl ::core::convert::From<DelegateSignerNotMatch> for operator_managerErrors {
        fn from(value: DelegateSignerNotMatch) -> Self {
            Self::DelegateSignerNotMatch(value)
        }
    }
    impl ::core::convert::From<DelegatecallFail> for operator_managerErrors {
        fn from(value: DelegatecallFail) -> Self {
            Self::DelegatecallFail(value)
        }
    }
    impl ::core::convert::From<EnumerableSetError> for operator_managerErrors {
        fn from(value: EnumerableSetError) -> Self {
            Self::EnumerableSetError(value)
        }
    }
    impl ::core::convert::From<FrozenBalanceInconsistent> for operator_managerErrors {
        fn from(value: FrozenBalanceInconsistent) -> Self {
            Self::FrozenBalanceInconsistent(value)
        }
    }
    impl ::core::convert::From<InsurancePositionQtyInvalid> for operator_managerErrors {
        fn from(value: InsurancePositionQtyInvalid) -> Self {
            Self::InsurancePositionQtyInvalid(value)
        }
    }
    impl ::core::convert::From<InsuranceTransferAmountInvalid> for operator_managerErrors {
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
    impl ::core::convert::From<InvalidFeeCollectorType> for operator_managerErrors {
        fn from(value: InvalidFeeCollectorType) -> Self {
            Self::InvalidFeeCollectorType(value)
        }
    }
    impl ::core::convert::From<InvalidMarginMode> for operator_managerErrors {
        fn from(value: InvalidMarginMode) -> Self {
            Self::InvalidMarginMode(value)
        }
    }
    impl ::core::convert::From<InvalidPrimeWallet> for operator_managerErrors {
        fn from(value: InvalidPrimeWallet) -> Self {
            Self::InvalidPrimeWallet(value)
        }
    }
    impl ::core::convert::From<InvalidVault> for operator_managerErrors {
        fn from(value: InvalidVault) -> Self {
            Self::InvalidVault(value)
        }
    }
    impl ::core::convert::From<IsoAdlMarginToCrossAmountInvalid> for operator_managerErrors {
        fn from(value: IsoAdlMarginToCrossAmountInvalid) -> Self {
            Self::IsoAdlMarginToCrossAmountInvalid(value)
        }
    }
    impl ::core::convert::From<LedgerAddressZero> for operator_managerErrors {
        fn from(value: LedgerAddressZero) -> Self {
            Self::LedgerAddressZero(value)
        }
    }
    impl ::core::convert::From<MarginTransferV3AmountInvalid> for operator_managerErrors {
        fn from(value: MarginTransferV3AmountInvalid) -> Self {
            Self::MarginTransferV3AmountInvalid(value)
        }
    }
    impl ::core::convert::From<NotImplemented> for operator_managerErrors {
        fn from(value: NotImplemented) -> Self {
            Self::NotImplemented(value)
        }
    }
    impl ::core::convert::From<OnlyCrossChainManagerCanCall> for operator_managerErrors {
        fn from(value: OnlyCrossChainManagerCanCall) -> Self {
            Self::OnlyCrossChainManagerCanCall(value)
        }
    }
    impl ::core::convert::From<OnlyCrossChainManagerV2CanCall> for operator_managerErrors {
        fn from(value: OnlyCrossChainManagerV2CanCall) -> Self {
            Self::OnlyCrossChainManagerV2CanCall(value)
        }
    }
    impl ::core::convert::From<OnlyLedgerCanCall> for operator_managerErrors {
        fn from(value: OnlyLedgerCanCall) -> Self {
            Self::OnlyLedgerCanCall(value)
        }
    }
    impl ::core::convert::From<OnlyOperatorCanCall> for operator_managerErrors {
        fn from(value: OnlyOperatorCanCall) -> Self {
            Self::OnlyOperatorCanCall(value)
        }
    }
    impl ::core::convert::From<OnlyOperatorManagerCanCall> for operator_managerErrors {
        fn from(value: OnlyOperatorManagerCanCall) -> Self {
            Self::OnlyOperatorManagerCanCall(value)
        }
    }
    impl ::core::convert::From<OnlySymbolManagerOrOwner> for operator_managerErrors {
        fn from(value: OnlySymbolManagerOrOwner) -> Self {
            Self::OnlySymbolManagerOrOwner(value)
        }
    }
    impl ::core::convert::From<OperatorManagerAddressZero> for operator_managerErrors {
        fn from(value: OperatorManagerAddressZero) -> Self {
            Self::OperatorManagerAddressZero(value)
        }
    }
    impl ::core::convert::From<RebalanceAlreadySucc> for operator_managerErrors {
        fn from(value: RebalanceAlreadySucc) -> Self {
            Self::RebalanceAlreadySucc(value)
        }
    }
    impl ::core::convert::From<RebalanceChainIdInvalid> for operator_managerErrors {
        fn from(value: RebalanceChainIdInvalid) -> Self {
            Self::RebalanceChainIdInvalid(value)
        }
    }
    impl ::core::convert::From<RebalanceIdNotMatch> for operator_managerErrors {
        fn from(value: RebalanceIdNotMatch) -> Self {
            Self::RebalanceIdNotMatch(value)
        }
    }
    impl ::core::convert::From<RebalanceMintUnexpected> for operator_managerErrors {
        fn from(value: RebalanceMintUnexpected) -> Self {
            Self::RebalanceMintUnexpected(value)
        }
    }
    impl ::core::convert::From<RebalanceStillPending> for operator_managerErrors {
        fn from(value: RebalanceStillPending) -> Self {
            Self::RebalanceStillPending(value)
        }
    }
    impl ::core::convert::From<RebalanceTokenNotSupported> for operator_managerErrors {
        fn from(value: RebalanceTokenNotSupported) -> Self {
            Self::RebalanceTokenNotSupported(value)
        }
    }
    impl ::core::convert::From<SafeCastOverflow> for operator_managerErrors {
        fn from(value: SafeCastOverflow) -> Self {
            Self::SafeCastOverflow(value)
        }
    }
    impl ::core::convert::From<SafeCastUnderflow> for operator_managerErrors {
        fn from(value: SafeCastUnderflow) -> Self {
            Self::SafeCastUnderflow(value)
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
    impl ::core::convert::From<SymbolNotRegister> for operator_managerErrors {
        fn from(value: SymbolNotRegister) -> Self {
            Self::SymbolNotRegister(value)
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
    impl ::core::convert::From<UnsupportChainType> for operator_managerErrors {
        fn from(value: UnsupportChainType) -> Self {
            Self::UnsupportChainType(value)
        }
    }
    impl ::core::convert::From<UserPerpPositionQtyZero> for operator_managerErrors {
        fn from(value: UserPerpPositionQtyZero) -> Self {
            Self::UserPerpPositionQtyZero(value)
        }
    }
    impl ::core::convert::From<WithdrawBalanceNotEnough> for operator_managerErrors {
        fn from(value: WithdrawBalanceNotEnough) -> Self {
            Self::WithdrawBalanceNotEnough(value)
        }
    }
    impl ::core::convert::From<WithdrawFeeTooLarge> for operator_managerErrors {
        fn from(value: WithdrawFeeTooLarge) -> Self {
            Self::WithdrawFeeTooLarge(value)
        }
    }
    impl ::core::convert::From<WithdrawToAddressZero> for operator_managerErrors {
        fn from(value: WithdrawToAddressZero) -> Self {
            Self::WithdrawToAddressZero(value)
        }
    }
    impl ::core::convert::From<WithdrawVaultBalanceNotEnough> for operator_managerErrors {
        fn from(value: WithdrawVaultBalanceNotEnough) -> Self {
            Self::WithdrawVaultBalanceNotEnough(value)
        }
    }
    impl ::core::convert::From<ZeroChainId> for operator_managerErrors {
        fn from(value: ZeroChainId) -> Self {
            Self::ZeroChainId(value)
        }
    }
    impl ::core::convert::From<ZeroDelegateContract> for operator_managerErrors {
        fn from(value: ZeroDelegateContract) -> Self {
            Self::ZeroDelegateContract(value)
        }
    }
    impl ::core::convert::From<ZeroDelegateSigner> for operator_managerErrors {
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
        name = "ChangeEngineUpload",
        abi = "ChangeEngineUpload(uint8,address,address)"
    )]
    pub struct ChangeEngineUploadFilter {
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
        Hash,
    )]
    #[ethevent(
        name = "ChangeOperatorImplA",
        abi = "ChangeOperatorImplA(address,address)"
    )]
    pub struct ChangeOperatorImplAFilter {
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
        name = "ChangeOperatorImplB",
        abi = "ChangeOperatorImplB(address,address)"
    )]
    pub struct ChangeOperatorImplBFilter {
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
    #[ethevent(name = "EventUpload", abi = "EventUpload(uint64)")]
    pub struct EventUpload1Filter {
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
        Hash,
    )]
    #[ethevent(name = "EventUpload", abi = "EventUpload(uint64,uint256)")]
    pub struct EventUpload2Filter {
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
        Hash,
    )]
    #[ethevent(name = "FuturesTradeUpload", abi = "FuturesTradeUpload(uint64)")]
    pub struct FuturesTradeUpload1Filter {
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
        Hash,
    )]
    #[ethevent(
        name = "FuturesTradeUpload",
        abi = "FuturesTradeUpload(uint64,uint256)"
    )]
    pub struct FuturesTradeUpload2Filter {
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
        Hash,
    )]
    #[ethevent(name = "FuturesTradeUploadV3", abi = "FuturesTradeUploadV3(uint64)")]
    pub struct FuturesTradeUploadV3Filter {
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
    #[ethevent(name = "RebalanceBurnUpload", abi = "RebalanceBurnUpload(uint64)")]
    pub struct RebalanceBurnUploadFilter {
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
    #[ethevent(name = "RebalanceMintUpload", abi = "RebalanceMintUpload(uint64)")]
    pub struct RebalanceMintUploadFilter {
        #[ethevent(indexed)]
        pub rebalance_id: u64,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum operator_managerEvents {
        ChangeEngineUploadFilter(ChangeEngineUploadFilter),
        ChangeLedgerFilter(ChangeLedgerFilter),
        ChangeMarketManagerFilter(ChangeMarketManagerFilter),
        ChangeOperatorFilter(ChangeOperatorFilter),
        ChangeOperatorImplAFilter(ChangeOperatorImplAFilter),
        ChangeOperatorImplBFilter(ChangeOperatorImplBFilter),
        EventUpload1Filter(EventUpload1Filter),
        EventUpload2Filter(EventUpload2Filter),
        FuturesTradeUpload1Filter(FuturesTradeUpload1Filter),
        FuturesTradeUpload2Filter(FuturesTradeUpload2Filter),
        FuturesTradeUploadV3Filter(FuturesTradeUploadV3Filter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        RebalanceBurnUploadFilter(RebalanceBurnUploadFilter),
        RebalanceMintUploadFilter(RebalanceMintUploadFilter),
    }
    impl ::ethers::contract::EthLogDecode for operator_managerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ChangeEngineUploadFilter::decode_log(log) {
                return Ok(operator_managerEvents::ChangeEngineUploadFilter(decoded));
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
            if let Ok(decoded) = ChangeOperatorImplAFilter::decode_log(log) {
                return Ok(operator_managerEvents::ChangeOperatorImplAFilter(decoded));
            }
            if let Ok(decoded) = ChangeOperatorImplBFilter::decode_log(log) {
                return Ok(operator_managerEvents::ChangeOperatorImplBFilter(decoded));
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
            if let Ok(decoded) = FuturesTradeUploadV3Filter::decode_log(log) {
                return Ok(operator_managerEvents::FuturesTradeUploadV3Filter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(operator_managerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(operator_managerEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = RebalanceBurnUploadFilter::decode_log(log) {
                return Ok(operator_managerEvents::RebalanceBurnUploadFilter(decoded));
            }
            if let Ok(decoded) = RebalanceMintUploadFilter::decode_log(log) {
                return Ok(operator_managerEvents::RebalanceMintUploadFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for operator_managerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ChangeEngineUploadFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeLedgerFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeMarketManagerFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeOperatorFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeOperatorImplAFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeOperatorImplBFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EventUpload1Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EventUpload2Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FuturesTradeUpload1Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FuturesTradeUpload2Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FuturesTradeUploadV3Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceBurnUploadFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceMintUploadFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ChangeEngineUploadFilter> for operator_managerEvents {
        fn from(value: ChangeEngineUploadFilter) -> Self {
            Self::ChangeEngineUploadFilter(value)
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
    impl ::core::convert::From<ChangeOperatorImplAFilter> for operator_managerEvents {
        fn from(value: ChangeOperatorImplAFilter) -> Self {
            Self::ChangeOperatorImplAFilter(value)
        }
    }
    impl ::core::convert::From<ChangeOperatorImplBFilter> for operator_managerEvents {
        fn from(value: ChangeOperatorImplBFilter) -> Self {
            Self::ChangeOperatorImplBFilter(value)
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
    impl ::core::convert::From<FuturesTradeUploadV3Filter> for operator_managerEvents {
        fn from(value: FuturesTradeUploadV3Filter) -> Self {
            Self::FuturesTradeUploadV3Filter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for operator_managerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for operator_managerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<RebalanceBurnUploadFilter> for operator_managerEvents {
        fn from(value: RebalanceBurnUploadFilter) -> Self {
            Self::RebalanceBurnUploadFilter(value)
        }
    }
    impl ::core::convert::From<RebalanceMintUploadFilter> for operator_managerEvents {
        fn from(value: RebalanceMintUploadFilter) -> Self {
            Self::RebalanceMintUploadFilter(value)
        }
    }
    ///Container type for all input parameters for the `bizTypeToSelectors` function with signature `bizTypeToSelectors(uint8)` and selector `0xb9847b92`
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
    #[ethcall(name = "bizTypeToSelectors", abi = "bizTypeToSelectors(uint8)")]
    pub struct BizTypeToSelectorsCall(pub u8);
    ///Container type for all input parameters for the `checkEngineDown` function with signature `checkEngineDown()` and selector `0x0bc70960`
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
    #[ethcall(name = "checkEngineDown", abi = "checkEngineDown()")]
    pub struct CheckEngineDownCall;
    ///Container type for all input parameters for the `engineEventUploadAddress` function with signature `engineEventUploadAddress()` and selector `0xc553cd84`
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
    #[ethcall(name = "engineEventUploadAddress", abi = "engineEventUploadAddress()")]
    pub struct EngineEventUploadAddressCall;
    ///Container type for all input parameters for the `engineMarketUploadAddress` function with signature `engineMarketUploadAddress()` and selector `0x895f9313`
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
        name = "engineMarketUploadAddress",
        abi = "engineMarketUploadAddress()"
    )]
    pub struct EngineMarketUploadAddressCall;
    ///Container type for all input parameters for the `enginePerpTradeUploadAddress` function with signature `enginePerpTradeUploadAddress()` and selector `0xe02a7d8d`
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
        name = "enginePerpTradeUploadAddress",
        abi = "enginePerpTradeUploadAddress()"
    )]
    pub struct EnginePerpTradeUploadAddressCall;
    ///Container type for all input parameters for the `engineRebalanceUploadAddress` function with signature `engineRebalanceUploadAddress()` and selector `0xc97d4f5a`
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
        name = "engineRebalanceUploadAddress",
        abi = "engineRebalanceUploadAddress()"
    )]
    pub struct EngineRebalanceUploadAddressCall;
    ///Container type for all input parameters for the `engineSpotTradeUploadAddress` function with signature `engineSpotTradeUploadAddress()` and selector `0x407d3915`
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
        name = "engineSpotTradeUploadAddress",
        abi = "engineSpotTradeUploadAddress()"
    )]
    pub struct EngineSpotTradeUploadAddressCall;
    ///Container type for all input parameters for the `eventUpload` function with signature `eventUpload(((uint8,uint64,bytes)[],bytes32,bytes32,uint8,uint8,uint64))` and selector `0xe8bb8f8f`
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
        name = "eventUpload",
        abi = "eventUpload(((uint8,uint64,bytes)[],bytes32,bytes32,uint8,uint8,uint64))"
    )]
    pub struct EventUploadCall {
        pub data: EventUpload,
    }
    ///Container type for all input parameters for the `eventUploadBatchId` function with signature `eventUploadBatchId()` and selector `0xd4b6de87`
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
    #[ethcall(name = "eventUploadBatchId", abi = "eventUploadBatchId()")]
    pub struct EventUploadBatchIdCall;
    ///Container type for all input parameters for the `futuresTradeUpload` function with signature `futuresTradeUpload((bytes32,bytes32,uint8,uint64,uint8,(bytes32,bytes32,bytes32,int128,int128,uint128,int128,int128,uint64,uint64,uint64,bool)[]))` and selector `0xdba9ab14`
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
        name = "futuresTradeUpload",
        abi = "futuresTradeUpload((bytes32,bytes32,uint8,uint64,uint8,(bytes32,bytes32,bytes32,int128,int128,uint128,int128,int128,uint64,uint64,uint64,bool)[]))"
    )]
    pub struct FuturesTradeUploadCall {
        pub data: FuturesTradeUploadData,
    }
    ///Container type for all input parameters for the `futuresTradeUploadV3` function with signature `futuresTradeUploadV3((bytes32,bytes32,uint8,uint64,uint8,(bytes32,bytes32,bytes32,int128,int128,uint128,int128,int128,uint64,uint64,uint64,bool,uint8,bytes32,int128)[]))` and selector `0x6d201adf`
    #[derive(Clone, ::ethers::contract::EthCall, ::ethers::contract::EthDisplay)]
    #[ethcall(
        name = "futuresTradeUploadV3",
        abi = "futuresTradeUploadV3((bytes32,bytes32,uint8,uint64,uint8,(bytes32,bytes32,bytes32,int128,int128,uint128,int128,int128,uint64,uint64,uint64,bool,uint8,bytes32,int128)[]))"
    )]
    pub struct FuturesTradeUploadV3Call {
        pub data: FuturesTradeUploadDataV3,
    }
    ///Container type for all input parameters for the `futuresUploadBatchId` function with signature `futuresUploadBatchId()` and selector `0x89ac77c5`
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
    #[ethcall(name = "futuresUploadBatchId", abi = "futuresUploadBatchId()")]
    pub struct FuturesUploadBatchIdCall;
    ///Container type for all input parameters for the `getOperatorManagerImpl` function with signature `getOperatorManagerImpl()` and selector `0x70664d30`
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
    #[ethcall(name = "getOperatorManagerImpl", abi = "getOperatorManagerImpl()")]
    pub struct GetOperatorManagerImplCall;
    ///Container type for all input parameters for the `initBizTypeToSelector` function with signature `initBizTypeToSelector()` and selector `0x466cc876`
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
    #[ethcall(name = "initBizTypeToSelector", abi = "initBizTypeToSelector()")]
    pub struct InitBizTypeToSelectorCall;
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
    ///Container type for all input parameters for the `lastOperatorInteraction` function with signature `lastOperatorInteraction()` and selector `0xfa8123c0`
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
    #[ethcall(name = "lastOperatorInteraction", abi = "lastOperatorInteraction()")]
    pub struct LastOperatorInteractionCall;
    ///Container type for all input parameters for the `ledger` function with signature `ledger()` and selector `0x56397c35`
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
    #[ethcall(name = "ledger", abi = "ledger()")]
    pub struct LedgerCall;
    ///Container type for all input parameters for the `marketManager` function with signature `marketManager()` and selector `0x41ed2c12`
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
    #[ethcall(name = "marketManager", abi = "marketManager()")]
    pub struct MarketManagerCall;
    ///Container type for all input parameters for the `operatorAddress` function with signature `operatorAddress()` and selector `0x127effb2`
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
    #[ethcall(name = "operatorAddress", abi = "operatorAddress()")]
    pub struct OperatorAddressCall;
    ///Container type for all input parameters for the `operatorManagerZipAddress` function with signature `operatorManagerZipAddress()` and selector `0xf92b162b`
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
        name = "operatorManagerZipAddress",
        abi = "operatorManagerZipAddress()"
    )]
    pub struct OperatorManagerZipAddressCall;
    ///Container type for all input parameters for the `operatorPing` function with signature `operatorPing()` and selector `0x6ecb9886`
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
    #[ethcall(name = "operatorPing", abi = "operatorPing()")]
    pub struct OperatorPingCall;
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
    ///Container type for all input parameters for the `perpPriceUpload` function with signature `perpPriceUpload((bytes32,bytes32,uint8,uint64,(bytes32,uint128,uint128,uint128,uint64)[]))` and selector `0x24ce6299`
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
        name = "perpPriceUpload",
        abi = "perpPriceUpload((bytes32,bytes32,uint8,uint64,(bytes32,uint128,uint128,uint128,uint64)[]))"
    )]
    pub struct PerpPriceUploadCall {
        pub data: UploadPerpPrice,
    }
    ///Container type for all input parameters for the `rebalanceBurnUpload` function with signature `rebalanceBurnUpload((bytes32,bytes32,uint8,uint64,uint128,bytes32,uint256,uint256))` and selector `0x7071c2df`
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
        name = "rebalanceBurnUpload",
        abi = "rebalanceBurnUpload((bytes32,bytes32,uint8,uint64,uint128,bytes32,uint256,uint256))"
    )]
    pub struct RebalanceBurnUploadCall {
        pub data: RebalanceBurnUploadData,
    }
    ///Container type for all input parameters for the `rebalanceMintUpload` function with signature `rebalanceMintUpload((bytes32,bytes32,uint8,uint64,uint128,bytes32,uint256,uint256,bytes,bytes))` and selector `0xc70a68ff`
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
        name = "rebalanceMintUpload",
        abi = "rebalanceMintUpload((bytes32,bytes32,uint8,uint64,uint128,bytes32,uint256,uint256,bytes,bytes))"
    )]
    pub struct RebalanceMintUploadCall {
        pub data: RebalanceMintUploadData,
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
        Hash,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setEngineEventUploadAddress` function with signature `setEngineEventUploadAddress(address)` and selector `0x3e172194`
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
        name = "setEngineEventUploadAddress",
        abi = "setEngineEventUploadAddress(address)"
    )]
    pub struct SetEngineEventUploadAddressCall {
        pub engine_event_upload_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setEngineMarketUploadAddress` function with signature `setEngineMarketUploadAddress(address)` and selector `0x681d4e34`
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
        name = "setEngineMarketUploadAddress",
        abi = "setEngineMarketUploadAddress(address)"
    )]
    pub struct SetEngineMarketUploadAddressCall {
        pub engine_market_upload_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setEnginePerpTradeUploadAddress` function with signature `setEnginePerpTradeUploadAddress(address)` and selector `0xb5f552a0`
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
        name = "setEnginePerpTradeUploadAddress",
        abi = "setEnginePerpTradeUploadAddress(address)"
    )]
    pub struct SetEnginePerpTradeUploadAddressCall {
        pub engine_perp_trade_upload_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setEngineRebalanceUploadAddress` function with signature `setEngineRebalanceUploadAddress(address)` and selector `0x314c3157`
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
        name = "setEngineRebalanceUploadAddress",
        abi = "setEngineRebalanceUploadAddress(address)"
    )]
    pub struct SetEngineRebalanceUploadAddressCall {
        pub engine_rebalance_upload_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setEngineSpotTradeUploadAddress` function with signature `setEngineSpotTradeUploadAddress(address)` and selector `0x7e2ae251`
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
        name = "setEngineSpotTradeUploadAddress",
        abi = "setEngineSpotTradeUploadAddress(address)"
    )]
    pub struct SetEngineSpotTradeUploadAddressCall {
        pub engine_spot_trade_upload_address: ::ethers::core::types::Address,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "setOperator", abi = "setOperator(address)")]
    pub struct SetOperatorCall {
        pub operator_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setOperatorManagerImplA` function with signature `setOperatorManagerImplA(address)` and selector `0xb314b59f`
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
        name = "setOperatorManagerImplA",
        abi = "setOperatorManagerImplA(address)"
    )]
    pub struct SetOperatorManagerImplACall {
        pub operator_manager_impl_a: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setOperatorManagerImplB` function with signature `setOperatorManagerImplB(address)` and selector `0x68a1bab8`
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
        name = "setOperatorManagerImplB",
        abi = "setOperatorManagerImplB(address)"
    )]
    pub struct SetOperatorManagerImplBCall {
        pub operator_manager_impl_b: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setOperatorManagerZipAddress` function with signature `setOperatorManagerZipAddress(address)` and selector `0x892021dc`
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
        name = "setOperatorManagerZipAddress",
        abi = "setOperatorManagerZipAddress(address)"
    )]
    pub struct SetOperatorManagerZipAddressCall {
        pub operator_manager_zip_address: ::ethers::core::types::Address,
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
        Hash,
    )]
    #[ethcall(
        name = "sumUnitaryFundingsUpload",
        abi = "sumUnitaryFundingsUpload((bytes32,bytes32,uint8,uint64,(bytes32,int128,uint64)[]))"
    )]
    pub struct SumUnitaryFundingsUploadCall {
        pub data: UploadSumUnitaryFundings,
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
    #[derive(Clone, ::ethers::contract::EthAbiType)]
    pub enum operator_managerCalls {
        BizTypeToSelectors(BizTypeToSelectorsCall),
        CheckEngineDown(CheckEngineDownCall),
        EngineEventUploadAddress(EngineEventUploadAddressCall),
        EngineMarketUploadAddress(EngineMarketUploadAddressCall),
        EnginePerpTradeUploadAddress(EnginePerpTradeUploadAddressCall),
        EngineRebalanceUploadAddress(EngineRebalanceUploadAddressCall),
        EngineSpotTradeUploadAddress(EngineSpotTradeUploadAddressCall),
        EventUpload(EventUploadCall),
        EventUploadBatchId(EventUploadBatchIdCall),
        FuturesTradeUpload(FuturesTradeUploadCall),
        FuturesTradeUploadV3(FuturesTradeUploadV3Call),
        FuturesUploadBatchId(FuturesUploadBatchIdCall),
        GetOperatorManagerImpl(GetOperatorManagerImplCall),
        InitBizTypeToSelector(InitBizTypeToSelectorCall),
        Initialize(InitializeCall),
        LastOperatorInteraction(LastOperatorInteractionCall),
        Ledger(LedgerCall),
        MarketManager(MarketManagerCall),
        OperatorAddress(OperatorAddressCall),
        OperatorManagerZipAddress(OperatorManagerZipAddressCall),
        OperatorPing(OperatorPingCall),
        Owner(OwnerCall),
        PerpPriceUpload(PerpPriceUploadCall),
        RebalanceBurnUpload(RebalanceBurnUploadCall),
        RebalanceMintUpload(RebalanceMintUploadCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetEngineEventUploadAddress(SetEngineEventUploadAddressCall),
        SetEngineMarketUploadAddress(SetEngineMarketUploadAddressCall),
        SetEnginePerpTradeUploadAddress(SetEnginePerpTradeUploadAddressCall),
        SetEngineRebalanceUploadAddress(SetEngineRebalanceUploadAddressCall),
        SetEngineSpotTradeUploadAddress(SetEngineSpotTradeUploadAddressCall),
        SetLedger(SetLedgerCall),
        SetMarketManager(SetMarketManagerCall),
        SetOperator(SetOperatorCall),
        SetOperatorManagerImplA(SetOperatorManagerImplACall),
        SetOperatorManagerImplB(SetOperatorManagerImplBCall),
        SetOperatorManagerZipAddress(SetOperatorManagerZipAddressCall),
        SumUnitaryFundingsUpload(SumUnitaryFundingsUploadCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for operator_managerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <BizTypeToSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BizTypeToSelectors(decoded));
            }
            if let Ok(decoded) =
                <CheckEngineDownCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckEngineDown(decoded));
            }
            if let Ok(decoded) =
                <EngineEventUploadAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EngineEventUploadAddress(decoded));
            }
            if let Ok(decoded) =
                <EngineMarketUploadAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EngineMarketUploadAddress(decoded));
            }
            if let Ok(decoded) =
                <EnginePerpTradeUploadAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EnginePerpTradeUploadAddress(decoded));
            }
            if let Ok(decoded) =
                <EngineRebalanceUploadAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EngineRebalanceUploadAddress(decoded));
            }
            if let Ok(decoded) =
                <EngineSpotTradeUploadAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EngineSpotTradeUploadAddress(decoded));
            }
            if let Ok(decoded) = <EventUploadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EventUpload(decoded));
            }
            if let Ok(decoded) =
                <EventUploadBatchIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EventUploadBatchId(decoded));
            }
            if let Ok(decoded) =
                <FuturesTradeUploadCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FuturesTradeUpload(decoded));
            }
            if let Ok(decoded) =
                <FuturesTradeUploadV3Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FuturesTradeUploadV3(decoded));
            }
            if let Ok(decoded) =
                <FuturesUploadBatchIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FuturesUploadBatchId(decoded));
            }
            if let Ok(decoded) =
                <GetOperatorManagerImplCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOperatorManagerImpl(decoded));
            }
            if let Ok(decoded) =
                <InitBizTypeToSelectorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InitBizTypeToSelector(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <LastOperatorInteractionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LastOperatorInteraction(decoded));
            }
            if let Ok(decoded) = <LedgerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ledger(decoded));
            }
            if let Ok(decoded) = <MarketManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MarketManager(decoded));
            }
            if let Ok(decoded) =
                <OperatorAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorAddress(decoded));
            }
            if let Ok(decoded) =
                <OperatorManagerZipAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorManagerZipAddress(decoded));
            }
            if let Ok(decoded) = <OperatorPingCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorPing(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <PerpPriceUploadCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PerpPriceUpload(decoded));
            }
            if let Ok(decoded) =
                <RebalanceBurnUploadCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RebalanceBurnUpload(decoded));
            }
            if let Ok(decoded) =
                <RebalanceMintUploadCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RebalanceMintUpload(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetEngineEventUploadAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetEngineEventUploadAddress(decoded));
            }
            if let Ok(decoded) =
                <SetEngineMarketUploadAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetEngineMarketUploadAddress(decoded));
            }
            if let Ok(decoded) =
                <SetEnginePerpTradeUploadAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SetEnginePerpTradeUploadAddress(decoded));
            }
            if let Ok(decoded) =
                <SetEngineRebalanceUploadAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SetEngineRebalanceUploadAddress(decoded));
            }
            if let Ok(decoded) =
                <SetEngineSpotTradeUploadAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SetEngineSpotTradeUploadAddress(decoded));
            }
            if let Ok(decoded) = <SetLedgerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetLedger(decoded));
            }
            if let Ok(decoded) =
                <SetMarketManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetMarketManager(decoded));
            }
            if let Ok(decoded) = <SetOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetOperator(decoded));
            }
            if let Ok(decoded) =
                <SetOperatorManagerImplACall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetOperatorManagerImplA(decoded));
            }
            if let Ok(decoded) =
                <SetOperatorManagerImplBCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetOperatorManagerImplB(decoded));
            }
            if let Ok(decoded) =
                <SetOperatorManagerZipAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetOperatorManagerZipAddress(decoded));
            }
            if let Ok(decoded) =
                <SumUnitaryFundingsUploadCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SumUnitaryFundingsUpload(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for operator_managerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BizTypeToSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckEngineDown(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EngineEventUploadAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EngineMarketUploadAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnginePerpTradeUploadAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EngineRebalanceUploadAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EngineSpotTradeUploadAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EventUpload(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EventUploadBatchId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FuturesTradeUpload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FuturesTradeUploadV3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FuturesUploadBatchId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorManagerImpl(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitBizTypeToSelector(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LastOperatorInteraction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Ledger(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MarketManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OperatorAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OperatorManagerZipAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorPing(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PerpPriceUpload(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RebalanceBurnUpload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RebalanceMintUpload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetEngineEventUploadAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEngineMarketUploadAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEnginePerpTradeUploadAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEngineRebalanceUploadAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEngineSpotTradeUploadAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetLedger(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetMarketManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetOperator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetOperatorManagerImplA(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetOperatorManagerImplB(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetOperatorManagerZipAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SumUnitaryFundingsUpload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for operator_managerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BizTypeToSelectors(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckEngineDown(element) => ::core::fmt::Display::fmt(element, f),
                Self::EngineEventUploadAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::EngineMarketUploadAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnginePerpTradeUploadAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EngineRebalanceUploadAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EngineSpotTradeUploadAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EventUpload(element) => ::core::fmt::Display::fmt(element, f),
                Self::EventUploadBatchId(element) => ::core::fmt::Display::fmt(element, f),
                Self::FuturesTradeUpload(element) => ::core::fmt::Display::fmt(element, f),
                Self::FuturesTradeUploadV3(element) => ::core::fmt::Display::fmt(element, f),
                Self::FuturesUploadBatchId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorManagerImpl(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitBizTypeToSelector(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastOperatorInteraction(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ledger(element) => ::core::fmt::Display::fmt(element, f),
                Self::MarketManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorManagerZipAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorPing(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PerpPriceUpload(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceBurnUpload(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceMintUpload(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEngineEventUploadAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEngineMarketUploadAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetEnginePerpTradeUploadAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetEngineRebalanceUploadAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetEngineSpotTradeUploadAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetLedger(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMarketManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOperatorManagerImplA(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOperatorManagerImplB(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOperatorManagerZipAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SumUnitaryFundingsUpload(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BizTypeToSelectorsCall> for operator_managerCalls {
        fn from(value: BizTypeToSelectorsCall) -> Self {
            Self::BizTypeToSelectors(value)
        }
    }
    impl ::core::convert::From<CheckEngineDownCall> for operator_managerCalls {
        fn from(value: CheckEngineDownCall) -> Self {
            Self::CheckEngineDown(value)
        }
    }
    impl ::core::convert::From<EngineEventUploadAddressCall> for operator_managerCalls {
        fn from(value: EngineEventUploadAddressCall) -> Self {
            Self::EngineEventUploadAddress(value)
        }
    }
    impl ::core::convert::From<EngineMarketUploadAddressCall> for operator_managerCalls {
        fn from(value: EngineMarketUploadAddressCall) -> Self {
            Self::EngineMarketUploadAddress(value)
        }
    }
    impl ::core::convert::From<EnginePerpTradeUploadAddressCall> for operator_managerCalls {
        fn from(value: EnginePerpTradeUploadAddressCall) -> Self {
            Self::EnginePerpTradeUploadAddress(value)
        }
    }
    impl ::core::convert::From<EngineRebalanceUploadAddressCall> for operator_managerCalls {
        fn from(value: EngineRebalanceUploadAddressCall) -> Self {
            Self::EngineRebalanceUploadAddress(value)
        }
    }
    impl ::core::convert::From<EngineSpotTradeUploadAddressCall> for operator_managerCalls {
        fn from(value: EngineSpotTradeUploadAddressCall) -> Self {
            Self::EngineSpotTradeUploadAddress(value)
        }
    }
    impl ::core::convert::From<EventUploadCall> for operator_managerCalls {
        fn from(value: EventUploadCall) -> Self {
            Self::EventUpload(value)
        }
    }
    impl ::core::convert::From<EventUploadBatchIdCall> for operator_managerCalls {
        fn from(value: EventUploadBatchIdCall) -> Self {
            Self::EventUploadBatchId(value)
        }
    }
    impl ::core::convert::From<FuturesTradeUploadCall> for operator_managerCalls {
        fn from(value: FuturesTradeUploadCall) -> Self {
            Self::FuturesTradeUpload(value)
        }
    }
    impl ::core::convert::From<FuturesTradeUploadV3Call> for operator_managerCalls {
        fn from(value: FuturesTradeUploadV3Call) -> Self {
            Self::FuturesTradeUploadV3(value)
        }
    }
    impl ::core::convert::From<FuturesUploadBatchIdCall> for operator_managerCalls {
        fn from(value: FuturesUploadBatchIdCall) -> Self {
            Self::FuturesUploadBatchId(value)
        }
    }
    impl ::core::convert::From<GetOperatorManagerImplCall> for operator_managerCalls {
        fn from(value: GetOperatorManagerImplCall) -> Self {
            Self::GetOperatorManagerImpl(value)
        }
    }
    impl ::core::convert::From<InitBizTypeToSelectorCall> for operator_managerCalls {
        fn from(value: InitBizTypeToSelectorCall) -> Self {
            Self::InitBizTypeToSelector(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for operator_managerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LastOperatorInteractionCall> for operator_managerCalls {
        fn from(value: LastOperatorInteractionCall) -> Self {
            Self::LastOperatorInteraction(value)
        }
    }
    impl ::core::convert::From<LedgerCall> for operator_managerCalls {
        fn from(value: LedgerCall) -> Self {
            Self::Ledger(value)
        }
    }
    impl ::core::convert::From<MarketManagerCall> for operator_managerCalls {
        fn from(value: MarketManagerCall) -> Self {
            Self::MarketManager(value)
        }
    }
    impl ::core::convert::From<OperatorAddressCall> for operator_managerCalls {
        fn from(value: OperatorAddressCall) -> Self {
            Self::OperatorAddress(value)
        }
    }
    impl ::core::convert::From<OperatorManagerZipAddressCall> for operator_managerCalls {
        fn from(value: OperatorManagerZipAddressCall) -> Self {
            Self::OperatorManagerZipAddress(value)
        }
    }
    impl ::core::convert::From<OperatorPingCall> for operator_managerCalls {
        fn from(value: OperatorPingCall) -> Self {
            Self::OperatorPing(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for operator_managerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PerpPriceUploadCall> for operator_managerCalls {
        fn from(value: PerpPriceUploadCall) -> Self {
            Self::PerpPriceUpload(value)
        }
    }
    impl ::core::convert::From<RebalanceBurnUploadCall> for operator_managerCalls {
        fn from(value: RebalanceBurnUploadCall) -> Self {
            Self::RebalanceBurnUpload(value)
        }
    }
    impl ::core::convert::From<RebalanceMintUploadCall> for operator_managerCalls {
        fn from(value: RebalanceMintUploadCall) -> Self {
            Self::RebalanceMintUpload(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for operator_managerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetEngineEventUploadAddressCall> for operator_managerCalls {
        fn from(value: SetEngineEventUploadAddressCall) -> Self {
            Self::SetEngineEventUploadAddress(value)
        }
    }
    impl ::core::convert::From<SetEngineMarketUploadAddressCall> for operator_managerCalls {
        fn from(value: SetEngineMarketUploadAddressCall) -> Self {
            Self::SetEngineMarketUploadAddress(value)
        }
    }
    impl ::core::convert::From<SetEnginePerpTradeUploadAddressCall> for operator_managerCalls {
        fn from(value: SetEnginePerpTradeUploadAddressCall) -> Self {
            Self::SetEnginePerpTradeUploadAddress(value)
        }
    }
    impl ::core::convert::From<SetEngineRebalanceUploadAddressCall> for operator_managerCalls {
        fn from(value: SetEngineRebalanceUploadAddressCall) -> Self {
            Self::SetEngineRebalanceUploadAddress(value)
        }
    }
    impl ::core::convert::From<SetEngineSpotTradeUploadAddressCall> for operator_managerCalls {
        fn from(value: SetEngineSpotTradeUploadAddressCall) -> Self {
            Self::SetEngineSpotTradeUploadAddress(value)
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
    impl ::core::convert::From<SetOperatorManagerImplACall> for operator_managerCalls {
        fn from(value: SetOperatorManagerImplACall) -> Self {
            Self::SetOperatorManagerImplA(value)
        }
    }
    impl ::core::convert::From<SetOperatorManagerImplBCall> for operator_managerCalls {
        fn from(value: SetOperatorManagerImplBCall) -> Self {
            Self::SetOperatorManagerImplB(value)
        }
    }
    impl ::core::convert::From<SetOperatorManagerZipAddressCall> for operator_managerCalls {
        fn from(value: SetOperatorManagerZipAddressCall) -> Self {
            Self::SetOperatorManagerZipAddress(value)
        }
    }
    impl ::core::convert::From<SumUnitaryFundingsUploadCall> for operator_managerCalls {
        fn from(value: SumUnitaryFundingsUploadCall) -> Self {
            Self::SumUnitaryFundingsUpload(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for operator_managerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `bizTypeToSelectors` function with signature `bizTypeToSelectors(uint8)` and selector `0xb9847b92`
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
    pub struct BizTypeToSelectorsReturn(pub [u8; 4]);
    ///Container type for all return fields from the `checkEngineDown` function with signature `checkEngineDown()` and selector `0x0bc70960`
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
    pub struct CheckEngineDownReturn(pub bool);
    ///Container type for all return fields from the `engineEventUploadAddress` function with signature `engineEventUploadAddress()` and selector `0xc553cd84`
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
    pub struct EngineEventUploadAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `engineMarketUploadAddress` function with signature `engineMarketUploadAddress()` and selector `0x895f9313`
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
    pub struct EngineMarketUploadAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `enginePerpTradeUploadAddress` function with signature `enginePerpTradeUploadAddress()` and selector `0xe02a7d8d`
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
    pub struct EnginePerpTradeUploadAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `engineRebalanceUploadAddress` function with signature `engineRebalanceUploadAddress()` and selector `0xc97d4f5a`
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
    pub struct EngineRebalanceUploadAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `engineSpotTradeUploadAddress` function with signature `engineSpotTradeUploadAddress()` and selector `0x407d3915`
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
    pub struct EngineSpotTradeUploadAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `eventUploadBatchId` function with signature `eventUploadBatchId()` and selector `0xd4b6de87`
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
    pub struct EventUploadBatchIdReturn(pub u64);
    ///Container type for all return fields from the `futuresUploadBatchId` function with signature `futuresUploadBatchId()` and selector `0x89ac77c5`
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
    pub struct FuturesUploadBatchIdReturn(pub u64);
    ///Container type for all return fields from the `getOperatorManagerImpl` function with signature `getOperatorManagerImpl()` and selector `0x70664d30`
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
    pub struct GetOperatorManagerImplReturn(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all return fields from the `lastOperatorInteraction` function with signature `lastOperatorInteraction()` and selector `0xfa8123c0`
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
    pub struct LastOperatorInteractionReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `ledger` function with signature `ledger()` and selector `0x56397c35`
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
    pub struct LedgerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `marketManager` function with signature `marketManager()` and selector `0x41ed2c12`
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
    pub struct MarketManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `operatorAddress` function with signature `operatorAddress()` and selector `0x127effb2`
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
    pub struct OperatorAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `operatorManagerZipAddress` function with signature `operatorManagerZipAddress()` and selector `0xf92b162b`
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
    pub struct OperatorManagerZipAddressReturn(pub ::ethers::core::types::Address);
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
    ///`EventUpload((uint8,uint64,bytes)[],bytes32,bytes32,uint8,uint8,uint64)`
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
        Hash,
    )]
    pub struct EventUploadData {
        pub biz_type: u8,
        pub event_id: u64,
        pub data: ::ethers::core::types::Bytes,
    }
    ///`PerpPrice(bytes32,uint128,uint128,uint128,uint64)`
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
        pub stork_price: u128,
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
    ///`UploadPerpPrice(bytes32,bytes32,uint8,uint64,(bytes32,uint128,uint128,uint128,uint64)[])`
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
    ///`FuturesTradeUploadData(bytes32,bytes32,uint8,uint64,uint8,(bytes32,bytes32,bytes32,int128,int128,uint128,int128,int128,uint64,uint64,uint64,bool)[])`
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
    pub struct FuturesTradeUploadData {
        pub r: [u8; 32],
        pub s: [u8; 32],
        pub v: u8,
        pub batch_id: u64,
        pub count: u8,
        pub trades: ::std::vec::Vec<FuturesTradeUpload>,
    }
    ///`FuturesTradeUploadDataV3(bytes32,bytes32,uint8,uint64,uint8,(bytes32,bytes32,bytes32,int128,int128,uint128,int128,int128,uint64,uint64,uint64,bool,uint8,bytes32,int128)[])`
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
    pub struct FuturesTradeUploadDataV3 {
        pub r: [u8; 32],
        pub s: [u8; 32],
        pub v: u8,
        pub batch_id: u64,
        pub count: u8,
        pub trades: ::std::vec::Vec<FuturesTradeUploadV3>,
    }
    ///`FuturesTradeUploadV3(bytes32,bytes32,bytes32,int128,int128,uint128,int128,int128,uint64,uint64,uint64,bool,uint8,bytes32,int128)`
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
    pub struct FuturesTradeUploadV3 {
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
        pub margin_mode: u8,
        pub iso_margin_asset_hash: [u8; 32],
        pub margin_from_cross: i128,
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

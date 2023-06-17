#![no_std]

use gear_lib::non_fungible_token::token::TokenMetadata;
use gear_lib::non_fungible_token::{
    io::{NFTApproval, NFTTransfer, NFTTransferPayout},
    state::NFTState,
    royalties::*,
    token::*,
};
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};

pub use gear_lib::non_fungible_token::delegated::DelegatedApproveMessage;
use primitive_types::H256;

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<InitContract>;
    type Handle = InOut<ContractAction, ContractEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = IoContractState;
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ContractAction {
    Mint {
        transaction_id: u64,
        token_metadata: TokenMetadata,
    },
    TransferPayout {
        transaction_id: u64,
        to: ActorId,
        token_id: TokenId,
        amount: u128,
    },
    Approve {
        transaction_id: u64,
        to: ActorId,
        token_id: TokenId,
    },
    DelegatedApprove {
        transaction_id: u64,
        message: DelegatedApproveMessage,
        signature: [u8; 64],
    },
    IsApproved {
        to: ActorId,
        token_id: TokenId,
    },
    Clear {
        transaction_hash: H256,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitContract {
    pub seller_wallet: ActorId,
    pub client_wallet: ActorId,
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ContractEvent {
    Transfer(NFTTransfer),
    TransferPayout(NFTTransferPayout),
    NFTPayout(Payout),
    Approval(NFTApproval),
    Owner {
        owner: ActorId,
        token_id: TokenId,
    },
    IsApproved {
        to: ActorId,
        token_id: TokenId,
        approved: bool,
    },
}

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct User {
    pub wallet: ActorId,
    pub confirmation: Option<bool>,
}

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IoContractState {
    pub seller: User,
    pub client: User,
    pub closed: bool,
}

pub struct IoContract {
    pub handshake: IoContractState,
    pub transactions: Vec<(H256, ContractEvent)>,
}

pub struct ContractState {
    seller: User,
    client: User,
    closed: bool,
}

impl From<&ContractState> for IoContractState {
    fn from(value: &ContractState) -> Self {
        let ContractState{
            seller,
            client,
            closed,
        } = value;

        Self {
            seller: seller.clone(),
            client: client.clone(),
            closed: *closed,
        }
    }
}
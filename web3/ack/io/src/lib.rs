#![no_std]

use gear_lib::non_fungible_token::token::TokenMetadata;
use gear_lib::non_fungible_token::{
    io::{NFTApproval, NFTTransfer, NFTTransferPayout},
    royalties::*,
    state::NFTState,
    token::*,
};
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};

pub use gear_lib::non_fungible_token::delegated::DelegatedApproveMessage;
use primitive_types::H256;

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = ();
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
        token_metadata: ContractTokenMetaData,
    },
    AckAction {
        user_wallet: ActorId,
        role: String,
        token_id: TokenId,
    },
    Owner {
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
    pub name: String,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct ContractTokenMetaData {
    client_wallet: ActorId,
    seller_waller: ActorId,
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ContractEvent {
    Transfer(NFTTransfer),
    AckAction(),
    Owner { owner: ActorId, token_id: TokenId },
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
    pub name: String,
    pub seller: User,
    pub client: User,
    pub closed: bool,
    pub owner_by_id: Vec<(TokenId, ActorId)>,
    pub token_metadata_by_id: Vec<(TokenId, Option<TokenMetadata>)>,
    pub tokens_for_owner: Vec<(ActorId, Vec<TokenId>)>,
}

pub struct IoContract {
    pub handshake: IoContractState,
    pub transactions: Vec<(H256, ContractEvent)>,
}

pub struct ContractState {
    name: String,
    seller: User,
    client: User,
    closed: bool,
    owner_by_id: Vec<(TokenId, ActorId)>,
    token_metadata_by_id: Vec<(TokenId, Option<TokenMetadata>)>,
    tokens_for_owner: Vec<(ActorId, Vec<TokenId>)>,
}

impl From<&ContractState> for IoContractState {
    fn from(value: &ContractState) -> Self {
        let ContractState {
            name,
            seller,
            client,
            closed,
            owner_by_id,
            token_metadata_by_id,
            tokens_for_owner,
        } = value;

        let owner_by_id = owner_by_id
            .iter()
            .map(|(hash, actor_id)| (*hash, *actor_id))
            .collect();

        let token_metadata_by_id = token_metadata_by_id
            .iter()
            .map(|(id, metadata)| (*id, metadata.clone()))
            .collect();

        let tokens_for_owner = tokens_for_owner
            .iter()
            .map(|(id, tokens)| (*id, tokens.clone()))
            .collect();

        Self {
            name: name.clone(),
            seller: seller.clone(),
            client: client.clone(),
            closed: *closed,
            owner_by_id,
            token_metadata_by_id,
            tokens_for_owner,
        }
    }
}

#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};

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

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FTAction {
    Mint(ContractTokenMetaData, objeto:ActorId),
    Burn(u128),
    Transfer {
        from: ActorId,
        to: ActorId,
        users: ContractMetadata,
        objeto: ActorId,
    },
    Approve {
        to: ActorId,
        amount: u128,
    },
    TotalSupply,
    BalanceOf(ActorId),
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
    seller_wallet: ActorId,
    objeto: ActorId,
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ContractEvent {
    Transfer {
        from: ActorId,
        to: ActorId,
        users: ContractMetadata,
        objeto: ActorId,
    },
    Approve {
        to: ActorId,
        amount: u128,
    },
    TotalSupply,
    BalanceOf(ActorId),
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
    pub objeto: ActorId,
    pub balances: Vec<(ActorId, u128)>,
    pub allowances: Vec<(ActorId, Vec<(ActorId, u128)>)>,
    pub decimals: u8,
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
    objeto: ActorId,
}

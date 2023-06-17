#![no_std]

use gear_lib::non_fungible_token::{
    state::NFTQueryReply,
    token::{Token, TokenId},
};
use gmeta::{metawasm, Metadata};
use gstd::{ActorId, Vec};
use nft_io::ContractMetadata;

pub enum ContractQueryReply {
    ContractInfo {
        seller: User,
        client: User,
        closed: bool,
    },
}

#[metawasm]
pub mod metafns {
    pub type State = <ContractMetadata as Metadata>::State;
    pub fn info(state: State) -> ContractQueryReply {
        ContractQueryReply::ContractInfo {
            seller: state.token.seller.clone(),
            client: state.token.client.clone(),
            closed: state.closed,
        }
    }

    pub fn token(state: State, token_id: TokenId) -> Token {
        token_helper(&token_id, &state)
    }
}

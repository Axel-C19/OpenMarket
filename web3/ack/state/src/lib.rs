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

    pub fn supply_for_owner(state: State, owner: ActorId) -> u128 {
        let tokens = state
            .token
            .tokens_for_owner
            .iter()
            .find(|(id, _tokens)| owner.eq(id));

        tokens
            .map(|(_id, tokens)| tokens.len() as u128)
            .unwrap_or(0)
    }

    pub fn all_tokens(state: State) -> Vec<Token> {
        state
            .token
            .owner_by_id
            .iter()
            .map(|(id, _owner)| token_helper(id, &state))
            .collect()
    }

    pub fn token_by_id(state: State, id: TokenId) -> Option<Token> {
        state
            .token
            .owner_by_id
            .iter()
            .find(|(i, _)| id.eq(i))
            .map(|(token_id, _)| token_helper(token_id, &state))
    }
}

fn token_helper(token_id: &TokenId, state: &<NFTMetadata as Metadata>::State) -> Token {
    let mut token = Token::default();
    if let Some((_token_id, owner_id)) = state
        .token
        .owner_by_id
        .iter()
        .find(|(id, _metadata)| token_id.eq(id))
    {
        token.id = *token_id;
        token.owner_id = *owner_id;
    }
    if let Some((_token_id, approved_account_ids)) = state
        .token
        .token_approvals
        .iter()
        .find(|(id, _metadata)| token_id.eq(id))
    {
        token.approved_account_ids = approved_account_ids.iter().copied().collect();
    }
    if let Some((_token_id, Some(metadata))) = state
        .token
        .token_metadata_by_id
        .iter()
        .find(|(id, _metadata)| token_id.eq(id))
    {
        token.name = metadata.name.clone();
        token.description = metadata.description.clone();
        token.media = metadata.media.clone();
        token.reference = metadata.reference.clone();
    }
    token
}

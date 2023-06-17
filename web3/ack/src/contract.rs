#![no_std]
use gear_lib::non_fungible_token::{io::NFTTransfer, nft_core::*, state::*, token::*};
use gear_lib_derive::{NFTCore, NFTMetaState, NFTStateKeeper};
use gmeta::Metadata;
use gstd::{errors::Result as GstdResult, exec, msg, prelude::*, ActorId, MessageId};
use hashbrown::HashMap;
use nft_io::{
    ContractAction, ContractEvent, ContractMetadata, ContractState, InitContract, IoContract,
};
use primitive_types::{H256, U256};

#[derive(Debug, Default, NFTStateKeeper, NFTCore, NFTMetaState)]
pub struct Contract {
    pub token: ContractState,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub transactions: HashMap<H256, ContractEvent>,
}

static mut CONTRACT: Option<Contract> = None;

#[no_mangle]
unsafe extern "C" fn init() {
    let config: InitContract = msg::load().expect("Unable to decode InitContract");
    let contrato = Contract {
        token: ContractState {
            name: config.name,
            seller: config.seller,
            client: config.client,
            closed: config.closed,
            ..Default::default()
        },
        owner: msg::source(),
        ..Default::default()
    };
    CONTRACT = Some(contrato);
}

#[no_mangle]
unsafe extern "C" fn handle() {
    let action: ContractAction = msg::load().expect("Could not load ContractAction");
    let contrato = CONTRACT.get_or_insert(Default::default());
    match action {
        ContractAction::Mint {
            transaction_id,
            token_metadata,
        } => {
            msg::reply(
                contrato.process_transaction(transaction_id, |contrato| {
                    ContractEvent::Transfer(MyNFTCore::mint(contrato, token_metadata))
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::Transfer`");
        }
        ContractAction::Owner { token_id } => {
            msg::reply(
                ContractEvent::Owner {
                    owner: NFTCore::owner_of(contrato, token_id),
                    token_id,
                },
                0,
            )
            .expect("Error during replying with `NFTEvent::Owner`");
        }
        ContractAction::Clear { transaction_hash } => contrato.clear(transaction_hash),
    };
}

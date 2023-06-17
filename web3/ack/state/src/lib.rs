#![no_std]

use ft_io::*;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};

#[metawasm]
pub mod metafns {
    pub type State = <ContractTokenMetaData as Metadata>::State;

    pub fn name(state: State) -> String {
        state.name
    }

    pub fn balances_of(state: State, account: ActorId) -> u128 {
        match state.balances.iter().find(|(id, _balance)| account.eq(id)) {
            Some((_id, balance)) => *balance,
            None => panic!("Balance for account ID {account:?} not found",),
        }
    }
}

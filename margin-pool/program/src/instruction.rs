//! Instruction types

#![allow(clippy::too_many_arguments)]

use crate::curve::base::MarginPoolCurve;
use crate::error::MarginPoolError;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};
use std::convert::TryInto;
use std::mem::size_of;

/// Instructions supported by the MarginPoolInfo program.
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum MarginPoolInstruction {
    ///   Initializes a new MarginPool.
    ///
    ///   0. `[writable, signer]` New MarginPool to create.
    ///   1. `[]` $authority derived from `create_program_address(&[MarginPool account])`
    ///   4. `[]` Swap Market for token_A and token_B
    ///   2. `[]` token_A Account. Must be non zero, owned by $authority.
    ///   3. `[]` token_B Account. Must be non zero, owned by $authority.
    ///   5. `[writable]` Pool Token Mint. Must be empty, owned by $authority.
    ///   8. '[]` Token program id
    ///   9. '[]` Serum program id
    Initialize {
        /// nonce used to create valid program address
        nonce: u8,
    },

    ///   Open a position.
    ///
    ///   0. `[]` MarginPool
    ///   1. `[]` $authority
    ///   4. `[]` Swap Market for token_A and token_A
    ///   2. `[writable]` token_X SOURCE Account, amount is transferable by $authority,
    ///   3. `[writable]` token_X Base BORROW Account.
    ///   4. `[writable]` token_Y Base Account to deposit into. 
    ///   4. `[writable]` Uninitialized MarginPool state for the open position.
    ///   8. '[]` Token program id
    OpenPosition {
        /// SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
        amount_in: u64,
        /// SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
        borrow: u64,
        /// Minimum amount of DESTINATION token to output, prevents excessive slippage
        minimum_amount_out: u64,
    },

    ///   Close a position.
    ///
    ///   0. `[]` MarginPool
    ///   1. `[]` $authority
    ///   4. `[]` Swap Market for token_A and token_A
    ///   2. `[writable]` token_X SOURCE Account, amount is transferable by $authority,
    ///   3. `[writable]` token_X Base BORROW Account.
    ///   4. `[writable]` token_Y Base Account to deposit into. 
    ///   4. `[writable]` OpenPosition.
    ///   8. '[]` Token program id
    ClosePosition {
        /// SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
        amount_in: u64,
        /// Minimum amount of DESTINATION token to output, prevents excessive slippage
        minimum_amount_out: u64,
    },


    ///   Deposit some tokens into the pool.  The output is a "pool" token representing ownership
    ///   into the pool. Inputs are converted to the current ratio.
    ///
    ///   0. `[]` Token-swap
    ///   1. `[]` $authority
    ///   2. `[writable]` token_a $authority can transfer amount,
    ///   3. `[writable]` token_b $authority can transfer amount,
    ///   4. `[writable]` token_a Base Account to deposit into.
    ///   5. `[writable]` token_b Base Account to deposit into.
    ///   6. `[writable]` Pool MINT account, $authority is the owner.
    ///   7. `[writable]` Pool Account to deposit the generated tokens, user is the owner.
    ///   8. '[]` Token program id
    Deposit {
        /// Pool token amount to transfer. token_a and token_b amount are set by
        /// the current exchange rate and size of the pool
        pool_token_amount: u64,
        /// Maximum LP amount to deposit, prevents excessive slippage
        maximum_token_lp_amount: u64,
    },

    ///   Withdraw the token from the pool at the current ratio.
    ///
    ///   0. `[]` Token-swap
    ///   1. `[]` $authority
    ///   2. `[writable]` Pool mint account, $authority is the owner
    ///   3. `[writable]` SOURCE Pool account, amount is transferable by $authority.
    ///   4. `[writable]` token_a MarginPool Account to withdraw FROM.
    ///   5. `[writable]` token_b MarginPool Account to withdraw FROM.
    ///   6. `[writable]` token_a user Account to credit.
    ///   7. `[writable]` token_b user Account to credit.
    ///   8. `[writable]` Fee account, to receive withdrawal fees
    ///   9. '[]` Token program id
    Withdraw {
        /// Amount of pool tokens to burn. User receives an output of token a
        /// and b based on the percentage of the pool tokens that are returned.
        pool_token_amount: u64,
        /// Minimum amount of LP to receive, prevents excessive slippage
        minimum_token_LP_amount: u64,
    },
}

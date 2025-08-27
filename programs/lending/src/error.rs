use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
#[msg("Low balance")]
InsufficientFunds,

#[msg("Attempting to borrow more than allowed")]
OverBorrowableAmount,

#[msg("Attempting to repay more than borrowed")]
OverRepayAmount,
#[msg("Attempting to liquidate healthy account")]
OverLiquidation

}
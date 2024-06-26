use anchor_lang::prelude::*;

#[error_code]
pub enum WenWnsMarketplaceError {
    #[msg("Buy amount mismatch with listing amount")]
    ListingAmountMismatch,
    #[msg("SPL Payment token account required")]
    PaymentTokenAccountNotExistant,
    #[msg("Invalid SPL Payment token account")]
    InvalidPaymentTokenAccount,
    #[msg("Arithmetic error")]
    ArithmeticError,
}

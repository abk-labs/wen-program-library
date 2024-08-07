//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct ClaimDistribution {
    pub creator: solana_program::pubkey::Pubkey,

    pub distribution: solana_program::pubkey::Pubkey,

    pub payment_mint: solana_program::pubkey::Pubkey,

    pub distribution_token_account: Option<solana_program::pubkey::Pubkey>,

    pub creator_token_account: Option<solana_program::pubkey::Pubkey>,

    pub token_program: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,
}

impl ClaimDistribution {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.creator,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.distribution,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.payment_mint,
            false,
        ));
        if let Some(distribution_token_account) = self.distribution_token_account {
            accounts.push(solana_program::instruction::AccountMeta::new(
                distribution_token_account,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::WEN_ROYALTY_DISTRIBUTION_ID,
                false,
            ));
        }
        if let Some(creator_token_account) = self.creator_token_account {
            accounts.push(solana_program::instruction::AccountMeta::new(
                creator_token_account,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::WEN_ROYALTY_DISTRIBUTION_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = ClaimDistributionInstructionData::new()
            .try_to_vec()
            .unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::WEN_ROYALTY_DISTRIBUTION_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ClaimDistributionInstructionData {
    discriminator: [u8; 8],
}

impl ClaimDistributionInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [204, 156, 94, 85, 2, 125, 232, 180],
        }
    }
}

impl Default for ClaimDistributionInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `ClaimDistribution`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` creator
///   1. `[writable]` distribution
///   2. `[]` payment_mint
///   3. `[writable, optional]` distribution_token_account
///   4. `[writable, optional]` creator_token_account
///   5. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   6. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct ClaimDistributionBuilder {
    creator: Option<solana_program::pubkey::Pubkey>,
    distribution: Option<solana_program::pubkey::Pubkey>,
    payment_mint: Option<solana_program::pubkey::Pubkey>,
    distribution_token_account: Option<solana_program::pubkey::Pubkey>,
    creator_token_account: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ClaimDistributionBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn creator(&mut self, creator: solana_program::pubkey::Pubkey) -> &mut Self {
        self.creator = Some(creator);
        self
    }
    #[inline(always)]
    pub fn distribution(&mut self, distribution: solana_program::pubkey::Pubkey) -> &mut Self {
        self.distribution = Some(distribution);
        self
    }
    #[inline(always)]
    pub fn payment_mint(&mut self, payment_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payment_mint = Some(payment_mint);
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn distribution_token_account(
        &mut self,
        distribution_token_account: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.distribution_token_account = distribution_token_account;
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn creator_token_account(
        &mut self,
        creator_token_account: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.creator_token_account = creator_token_account;
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = ClaimDistribution {
            creator: self.creator.expect("creator is not set"),
            distribution: self.distribution.expect("distribution is not set"),
            payment_mint: self.payment_mint.expect("payment_mint is not set"),
            distribution_token_account: self.distribution_token_account,
            creator_token_account: self.creator_token_account,
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `claim_distribution` CPI accounts.
pub struct ClaimDistributionCpiAccounts<'a, 'b> {
    pub creator: &'b solana_program::account_info::AccountInfo<'a>,

    pub distribution: &'b solana_program::account_info::AccountInfo<'a>,

    pub payment_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub distribution_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub creator_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `claim_distribution` CPI instruction.
pub struct ClaimDistributionCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub creator: &'b solana_program::account_info::AccountInfo<'a>,

    pub distribution: &'b solana_program::account_info::AccountInfo<'a>,

    pub payment_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub distribution_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub creator_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> ClaimDistributionCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ClaimDistributionCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            creator: accounts.creator,
            distribution: accounts.distribution,
            payment_mint: accounts.payment_mint,
            distribution_token_account: accounts.distribution_token_account,
            creator_token_account: accounts.creator_token_account,
            token_program: accounts.token_program,
            system_program: accounts.system_program,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.creator.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.distribution.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.payment_mint.key,
            false,
        ));
        if let Some(distribution_token_account) = self.distribution_token_account {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *distribution_token_account.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::WEN_ROYALTY_DISTRIBUTION_ID,
                false,
            ));
        }
        if let Some(creator_token_account) = self.creator_token_account {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *creator_token_account.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::WEN_ROYALTY_DISTRIBUTION_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = ClaimDistributionInstructionData::new()
            .try_to_vec()
            .unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::WEN_ROYALTY_DISTRIBUTION_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(7 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.creator.clone());
        account_infos.push(self.distribution.clone());
        account_infos.push(self.payment_mint.clone());
        if let Some(distribution_token_account) = self.distribution_token_account {
            account_infos.push(distribution_token_account.clone());
        }
        if let Some(creator_token_account) = self.creator_token_account {
            account_infos.push(creator_token_account.clone());
        }
        account_infos.push(self.token_program.clone());
        account_infos.push(self.system_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `ClaimDistribution` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` creator
///   1. `[writable]` distribution
///   2. `[]` payment_mint
///   3. `[writable, optional]` distribution_token_account
///   4. `[writable, optional]` creator_token_account
///   5. `[]` token_program
///   6. `[]` system_program
#[derive(Clone, Debug)]
pub struct ClaimDistributionCpiBuilder<'a, 'b> {
    instruction: Box<ClaimDistributionCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ClaimDistributionCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ClaimDistributionCpiBuilderInstruction {
            __program: program,
            creator: None,
            distribution: None,
            payment_mint: None,
            distribution_token_account: None,
            creator_token_account: None,
            token_program: None,
            system_program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn creator(
        &mut self,
        creator: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.creator = Some(creator);
        self
    }
    #[inline(always)]
    pub fn distribution(
        &mut self,
        distribution: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.distribution = Some(distribution);
        self
    }
    #[inline(always)]
    pub fn payment_mint(
        &mut self,
        payment_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.payment_mint = Some(payment_mint);
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn distribution_token_account(
        &mut self,
        distribution_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.distribution_token_account = distribution_token_account;
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn creator_token_account(
        &mut self,
        creator_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.creator_token_account = creator_token_account;
        self
    }
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = ClaimDistributionCpi {
            __program: self.instruction.__program,

            creator: self.instruction.creator.expect("creator is not set"),

            distribution: self
                .instruction
                .distribution
                .expect("distribution is not set"),

            payment_mint: self
                .instruction
                .payment_mint
                .expect("payment_mint is not set"),

            distribution_token_account: self.instruction.distribution_token_account,

            creator_token_account: self.instruction.creator_token_account,

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct ClaimDistributionCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    creator: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    distribution: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payment_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    distribution_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    creator_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}

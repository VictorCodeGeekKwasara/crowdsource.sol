use borsh::{BorshDeserialize,BorshSerialize};
use std::collections::HashMap;
use std::convert::TryInto;

use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint,
  entrypoint::ProgramResult,
  msg,
  program_error::ProgramError,
  pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CampaignAccount {

  pub campaign_owner: Pubkey,

  pub campaign_amount: u64,

  pub campaign_description: String,

  pub campaign_fulfilled: u64,
}

pub fn process_instruction(
  program_id:&Pubkey,
  accounts: &[AccountInfo],
  data:&[u8],
)-> ProgramResult {

  let (instruction_byte, rest_of_data) = data.split_first().unwrap();

      let iterable_accounts = &mut accounts.iter();
      let campaign_account = next_account_info(iterable_accounts)?;
      let amount = rest_of_data
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .unwrap();

    let description = String::from_utf8(rest_of_data[9..].to_vec()).unwrap();

  match *instruction_byte {
    0 => { 
    let campaign_owner_account = next_account_info(iterable_accounts)?;
    let mut campaign_account_data = CampaignAccount::try_from_slice(&campaign_account.data.borrow())?;

    campaign_account_data.campaign_owner = *campaign_owner_account.owner;
    campaign_account_data.campaign_amount = amount ;
    campaign_account_data.campaign_description = description;

    campaign_account_data.campaign_fulfilled = 0 ;

    campaign_account_data.serialize(&mut &mut campaign_account.data.borrow_mut()[..])?;
   
    },
    1 => {
      // fund a campaign

      // let mut campaign_account_data = CampaignAccount::try_from_slice(&campaign_account.data.borrow())?;
      // msg!("{}", campaign_account_data.campaign_amount - campaign_account_data.campaign_fulfilled);
    },
    2 => {
      // get how much funds are left to reach the requested amount
    
      let mut campaign_account_data = CampaignAccount::try_from_slice(&campaign_account.data.borrow())?;
      msg!("{}", campaign_account_data.campaign_amount - campaign_account_data.campaign_fulfilled);
    },
    3 => {
      // withdraw all collected funds and close campaign
    }
    _=> ()
  } 

  Ok(())
}

entrypoint!(process_instruction);
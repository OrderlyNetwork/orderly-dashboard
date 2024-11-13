use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct AccountWithdrawSol {
    pub account_id: [u8; 32],
    pub sender: [u8; 32],
    pub receiver: [u8; 32],
    pub broker_hash: [u8; 32],
    pub token_hash: [u8; 32],
    pub token_amount: u128,
    pub fee: u128,
    pub chain_id: u128,
    pub withdraw_nonce: u64,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct VaultWithdrawn {
    pub account_id: [u8; 32],
    pub sender: [u8; 32],
    pub receiver: [u8; 32],
    pub broker_hash: [u8; 32],
    pub token_hash: [u8; 32],
    pub token_amount: u64,
    pub fee: u128,
    pub chain_id: u128,
    pub withdraw_nonce: u64,
}

#[cfg(test)]
mod tests {
    use borsh::BorshDeserialize;

    use super::{AccountWithdrawSol, VaultWithdrawn};

    #[test]
    fn test_account_withdraw_sol_decode() {
        let raw: &str = "tc6PPKaR1tUBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUF6AMAAAAAAAAAAAAAAAAAAAoAAAAAAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAA=";

        let data = base64::decode(raw).unwrap();
        let discriminator_preimage = format!("event:AccountWithdrawSol").into_bytes();
        let discriminator = anchor_syn::hash::hash(&discriminator_preimage);
        let discriminator = discriminator.0[..8].to_vec();

        let discriminator_decode = data[0..8].to_vec();
        let data = data[8..].to_vec();
        let withdraw = AccountWithdrawSol::deserialize(&mut &data[..]).unwrap();
        println!(
            "discriminator: {:?}, discriminator_decode: {:?}, withdraw data: {:?}",
            discriminator, discriminator_decode, withdraw
        );
        assert_eq!(discriminator, discriminator_decode);
    }

    #[test]
    fn test_vault_withdraw_decode() {
        let raw: &str = "7gnbrLxNSGie1Jia6mc54NV1rHibAPAcena9vfHggSRgtdYAPYzkkJrGUm8NoSNA6xUSse2F5LwTtcdTbEVvGbTOxzMkH/W5msZSbw2hI0DrFRKx7YXkvBO1x1NsRW8ZtM7HMyQf9bkIMJjFk/OVvqHeRd2lUtnxTo/LC+P6qnoZA8VHfXun/dasob6XKcE9Z3M1FhMhZJzMrmpZFVR3JRZwD5hvlC6qQEtMAAAAAABAQg8AAAAAAAAAAAAAAAAAdjTRNQAAAAAAAAAAAAAAAAMAAAAAAAAA";

        let data = base64::decode(raw).unwrap();
        let discriminator_preimage = format!("event:VaultWithdrawn").into_bytes();
        let discriminator = anchor_syn::hash::hash(&discriminator_preimage);
        let discriminator = discriminator.0[..8].to_vec();

        let discriminator_decode = data[0..8].to_vec();
        let data = data[8..].to_vec();
        let withdraw = VaultWithdrawn::deserialize(&mut &data[..]).unwrap();
        println!(
            "discriminator: {:?}, discriminator_decode: {:?}, vault withdraw data: {:?}",
            discriminator, discriminator_decode, withdraw
        );
        assert_eq!(discriminator, discriminator_decode);
    }
}

use solana_client::{rpc_config::RpcProgramAccountsConfig, rpc_filter::{Memcmp, RpcFilterType}};
use steel::*;

pub struct RpcClient {
    rpc: solana_client::nonblocking::rpc_client::RpcClient,
}

impl RpcClient {
    pub fn new(rpc_url: String) -> Self {
        let rpc = solana_client::nonblocking::rpc_client::RpcClient::new(rpc_url);
        Self { rpc }
    }
    
    pub async fn get_program_account<T>(&self, pubkey: Pubkey) -> Result<T, anyhow::Error> 
    where T: AccountDeserialize + Discriminator + Clone {
        let account = self.rpc.get_account(&pubkey).await?;
        let account = T::try_from_bytes(&account.data).unwrap().clone();
        Ok(account)
    }

    pub async fn get_program_accounts<T>(&self, program_id: Pubkey, filters: Vec<RpcFilterType>) -> Result<Vec<(Pubkey, T)>, anyhow::Error> 
    where T: AccountDeserialize + Discriminator + Clone {
        let mut all_filters = vec![
            RpcFilterType::Memcmp(Memcmp::new_raw_bytes(
                0,
                T::discriminator().to_le_bytes().to_vec(),
            )),
        ];
        all_filters.extend(filters);
        let accounts = self
            .rpc
            .get_program_accounts_with_config(
                &program_id,
                RpcProgramAccountsConfig {
                    filters: Some(all_filters),
                    ..Default::default()
                },
            )
            .await?
            .into_iter()
            .map(|(pubkey, account)| {
                let account = T::try_from_bytes(&account.data).unwrap().clone();
                (pubkey, account)
            })
        .collect();
        Ok(accounts)
    }
}

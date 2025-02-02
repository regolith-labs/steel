use steel::*;

/// Trait for a client that can be used to interact with a Steel program.
#[allow(async_fn_in_trait)]
pub trait SteelClient {
    /// Get a program account.
    async fn get_program_account<T>(&self, pubkey: Pubkey) -> Result<T, anyhow::Error> 
    where T: AccountDeserialize + Discriminator + Clone;

    /// Get all program accounts.
    #[cfg(not(feature = "wasm"))]
    async fn get_program_accounts<T>(
        &self, 
        program_id: Pubkey, 
        filters: Vec<solana_client::rpc_filter::RpcFilterType>
    ) -> Result<Vec<(Pubkey, T)>, anyhow::Error> 
    where T: AccountDeserialize + Discriminator + Clone;

    #[cfg(feature = "wasm")]
    async fn get_program_accounts<T>(
        &self, 
        program_id: Pubkey, 
        filters: Vec<solana_client_wasm::utils::rpc_filter::RpcFilterType>
    ) -> Result<Vec<(Pubkey, T)>, anyhow::Error> 
    where T: AccountDeserialize + Discriminator + Clone;
}

#[cfg(not(feature = "wasm"))]
impl SteelClient for solana_client::nonblocking::rpc_client::RpcClient {
    async fn get_program_account<T>(&self, pubkey: Pubkey) -> Result<T, anyhow::Error> 
    where T: AccountDeserialize + Discriminator + Clone {
        let account = self.get_account(&pubkey).await?;
        let account = T::try_from_bytes(&account.data).unwrap().clone();
        Ok(account)
    }

    async fn get_program_accounts<T>(
        &self, 
        program_id: Pubkey, 
        filters: Vec<solana_client::rpc_filter::RpcFilterType>
    ) -> Result<Vec<(Pubkey, T)>, anyhow::Error> 
    where T: AccountDeserialize + Discriminator + Clone {
        let mut all_filters = vec![
            solana_client::rpc_filter::RpcFilterType::Memcmp(solana_client::rpc_filter::Memcmp::new_raw_bytes(
                0,
                T::discriminator().to_le_bytes().to_vec(),
            )),
        ];
        all_filters.extend(filters);
        let accounts = self
            .get_program_accounts_with_config(
                &program_id,
                solana_client::rpc_config::RpcProgramAccountsConfig {
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


#[cfg(feature = "wasm")]
impl SteelClient for solana_client_wasm::WasmClient {
    async fn get_program_account<T>(&self, pubkey: Pubkey) -> Result<T, anyhow::Error> 
    where T: AccountDeserialize + Discriminator + Clone {
        let account = self.get_account(&pubkey).await?;
        let account = T::try_from_bytes(&account.data).unwrap().clone();
        Ok(account)
    }

    async fn get_program_accounts<T>(
        &self, 
        program_id: Pubkey, 
        filters: Vec<solana_client_wasm::utils::rpc_filter::RpcFilterType>
    ) -> Result<Vec<(Pubkey, T)>, anyhow::Error> 
    where T: AccountDeserialize + Discriminator + Clone {
        let mut all_filters = vec![
            solana_client_wasm::utils::rpc_filter::RpcFilterType::Memcmp(solana_client_wasm::utils::rpc_filter::Memcmp {
                offset: 0,
                bytes: solana_client_wasm::utils::rpc_filter::MemcmpEncodedBytes::Bytes(T::discriminator().to_le_bytes().to_vec()),
                encoding: Some(solana_client_wasm::utils::rpc_filter::MemcmpEncoding::Binary),
            }),
        ];
        all_filters.extend(filters);
        let accounts = self
            .get_program_accounts_with_config(
                &program_id,
                solana_client_wasm::utils::rpc_config::RpcProgramAccountsConfig {
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

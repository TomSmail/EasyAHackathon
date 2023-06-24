#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod contract {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    
    // import has
    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct OwnershipChange {
        previous_owner: AccountId,
        previous_location: String,
        timestamp: u64,
    }
    
    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Diamond {
        diamond_id: u32,
        origin: String,
        current_owner: AccountId,
        certifications: String,
        is_responsibly_sourced: bool,
        has_fair_labor_practices: bool,
        weight: u32,
        purchase_location: String,
        ownership_history: Vec<OwnershipChange>,
    }

    #[ink(storage)]
    pub struct DiamondTracker {
        diamonds: Mapping<u32, Diamond>,
    }

    impl DiamondTracker {
        #[ink(constructor)]
        pub fn new() ->  Self {
            Self {
                diamonds: Mapping::new(),
            }
        }

        /// Register a new diamond with the provided information.
        #[ink(message)]
        pub fn register_diamond(
            &mut self,
            diamond_id: u32,
            origin: String,
            current_owner: AccountId,
            certifications: String,
            is_responsibly_sourced: bool,
            has_fair_labor_practices: bool,
            weight: u32,
            purchase_location: String,
        ) -> Diamond {
            assert!(
                !self.diamonds.contains(diamond_id),
                "Diamond with this ID already registered"
            );

            let ownership_history = Vec::new(); // Initialize ownership history as an empty vector

            let diamond = Diamond {
                diamond_id,
                origin,
                current_owner: Self::env().caller(),
                certifications,
                is_responsibly_sourced,
                has_fair_labor_practices,
                weight,
                purchase_location,
                ownership_history,
            };

            self.diamonds.insert(&diamond_id, &diamond);

            diamond
        }

        // Get diamond with provided ID
        #[ink(message)]
        pub fn get_diamond(&mut self, diamond_id: u32) -> Diamond {
            assert!(
                self.diamonds.contains(&diamond_id),
                "Diamond with this ID not registered"
            );

            self.diamonds.get(&diamond_id).unwrap()
        }

        // transfer diamond with provided ID to new owner
        #[ink(message)]
        pub fn transfer_ownership(&mut self, diamond_id: u32, new_owner: AccountId, new_location: String) -> Diamond {
            assert!(
                self.diamonds.contains(&diamond_id),
                "Diamond with this ID not registered"
            );

            let mut diamond = self.diamonds.get(&diamond_id).unwrap();

            let ownership_change = OwnershipChange {
                previous_owner: diamond.current_owner,
                previous_location: diamond.purchase_location,
                timestamp: Self::env().block_timestamp(),
            };

            // check AccountId is not same as current owner
            assert!(
                diamond.current_owner != new_owner,
                "New owner cannot be the same as current owner"
            );

            // check new location is not empty
            assert!(
                new_location != String::default(),
                "New location cannot be empty"
            );

            diamond.current_owner = new_owner;
            diamond.purchase_location = new_location;
            diamond.ownership_history.push(ownership_change);

            // update diamond in storage
            self.diamonds.insert(&diamond_id, &diamond);

            diamond
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let contract = Contract::default();
            assert_eq!(contract.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut contract = Contract::new(false);
            assert_eq!(contract.get(), false);
            contract.flip();
            assert_eq!(contract.get(), true);
        }
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = ContractRef::default();

            // When
            let contract_account_id = client
                .instantiate("contract", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<ContractRef>(contract_account_id.clone())
                .call(|contract| contract.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = ContractRef::new(false);
            let contract_account_id = client
                .instantiate("contract", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<ContractRef>(contract_account_id.clone())
                .call(|contract| contract.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<ContractRef>(contract_account_id.clone())
                .call(|contract| contract.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<ContractRef>(contract_account_id.clone())
                .call(|contract| contract.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}

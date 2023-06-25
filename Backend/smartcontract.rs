use ink_lang::contract;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "ink-generate-abi", derive(type_metadata::Metadata))]
pub struct Diamond {
    diamond_id: u32,
    origin: String,
    current_owner: AccountId,
    certifications: String,
    is_responsibly_sourced: bool,
    has_fair_labor_practices: bool,
    weight: u32,
    ownership_history: Vec<OwnershipChange>, // New field for ownership history
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "ink-generate-abi", derive(type_metadata::Metadata))]
pub struct OwnershipChange {
    previous_owner: AccountId,
    previous_location: String, // New field for previous owner's location
    timestamp: u64,
}

#[contract]
pub mod diamond_traceability {
    use super::*;

    #[ink(storage)]
    pub struct DiamondTraceability {
        diamonds: ink_storage::collections::HashMap<u32, Diamond>,
    }

    impl DiamondTraceability {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                diamonds: ink_storage::collections::HashMap::new(),
            }
        }

        /// Register a new diamond with the provided information.
        #[ink(message)]
        pub fn register_diamond(
            &mut self,
            diamond_id: u32,
            origin: String,
            certifications: String,
            is_responsibly_sourced: bool,
            has_fair_labor_practices: bool,
            weight: u32,
            purchase_location: String,
        ) {
            assert!(
                !self.diamonds.contains_key(&diamond_id),
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
                ownership_history,
            };

            self.diamonds.insert(diamond_id, diamond);
        }

        /// Transfer ownership of a diamond to a new owner.
        #[ink(message)]
        pub fn transfer_ownership(&mut self, diamond_id: u32, new_owner: AccountId, new_location: String) {
            let diamond = self
                .diamonds
                .get_mut(&diamond_id)
                .expect("Diamond with this ID does not exist");

            assert_eq!(
                diamond.current_owner,
                Self::env().caller(),
                "You are not the current owner of this diamond"
            );

            // Capture the previous owner and previous location
            let previous_owner = diamond.current_owner;
            let previous_location = diamond
                .ownership_history
                .last()
                .map(|change| change.previous_location.clone())
                .unwrap_or_else(|| String::new());

            // Create a new ownership change event and add it to the ownership history
            diamond.ownership_history.push(OwnershipChange {
                previous_owner,
                previous_location,
                timestamp: Self::env().block_timestamp(),
            });

            // Update the current owner and location
            diamond.current_owner = new_owner;
        }

        #[ink(message)]
        pub fn check_compliance(&self, diamond_id: u32) -> bool {
            // Retrieve the diamond with the given ID
            let diamond = self
                .diamonds
                .get(&diamond_id)
                .expect("Diamond with this ID does not exist");

            // Check if all compliance criteria are met
            let certifications_valid = self.check_certifications(diamond_id);
            let responsible_sourcing_valid = self.check_responsible_sourcing(diamond_id);
            let fair_labor_practices_valid = self.check_fair_labor_practices(diamond_id);
            let weight_valid = self.check_weight(diamond_id);

            // Return true if all criteria are valid, false otherwise
            certifications_valid
                && responsible_sourcing_valid
                && fair_labor_practices_valid
                && weight_valid
        }

        #[ink(message)]
        pub fn enforce_compliance(&self, diamond_id: u32) {
            // Retrieve the diamond with the given ID
            let diamond = self
                .diamonds
                .get(&diamond_id)
                .expect("Diamond with this ID does not exist");

            // Check if the diamond complies with all criteria
            if !self.check_compliance(diamond_id) {
                // Emit an event indicating a compliance violation
                Self::env().emit_event(ComplianceViolation {
                    diamond_id,
                    owner: diamond.current_owner,
                });
            }
        }

        // Check if the diamond's certifications meet the criteria
        fn check_certifications(&self, diamond_id: u32) -> bool {
            // Retrieve the diamond with the given ID
            let diamond = self
                .diamonds
                .get(&diamond_id)
                .expect("Diamond with this ID does not exist");

            // Example certification criteria: Certifications should include GIA or AGS
            diamond.certifications.contains("GIA") || diamond.certifications.contains("AGS")
        }

        // Check if the diamond's sourcing is responsible
        fn check_responsible_sourcing(&self, diamond_id: u32) -> bool {
            // Retrieve the diamond with the given ID
            let diamond = self
                .diamonds
                .get(&diamond_id)
                .expect("Diamond with this ID does not exist");

            // Example responsible sourcing criteria: Diamond should be from a country
            diamond.origin == "Botswana" || diamond.origin == "Canada"
        }

        // Check if the diamond's labor practices are fair
        fn check_fair_labor_practices(&self, diamond_id: u32) -> bool {
            // Retrieve the diamond with the given ID
            let diamond = self
                .diamonds
                .get(&diamond_id)
                .expect("Diamond with this ID does not exist");

            // Example fair labor practices criteria: Diamond should not involve child labor
            !diamond.has_fair_labor_practices
        }

        // Check if the diamond's weight meets the criteria
        fn check_weight(&self, diamond_id: u32) -> bool {
            // Retrieve the diamond with the given ID
            let diamond = self
                .diamonds
                .get(&diamond_id)
                .expect("Diamond with this ID does not exist");

            // Example weight criteria: Diamond weight should be greater than 0
            diamond.weight > 0
        }
    }

    #[ink(event)]
    pub struct ComplianceViolation {
        #[ink(topic)]
        diamond_id: u32,
        #[ink(topic)]
        owner: AccountId,
    }
}

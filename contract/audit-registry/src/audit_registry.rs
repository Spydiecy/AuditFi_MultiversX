#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// Define the Audit struct outside the trait
#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone)]
pub struct Audit<M: ManagedTypeApi> {
    pub stars: u8,
    pub summary: ManagedBuffer<M>,
    pub auditor: ManagedAddress<M>,
    pub timestamp: u64,
}

// MultiversX implementation of the AuditRegistry contract
#[multiversx_sc::contract]
pub trait AuditRegistry {
    #[init]
    fn init(&self) {
        self.owner().set(&self.blockchain().get_caller());
    }

    // Endpoints
    #[endpoint]
    fn register_audit(
        &self,
        contract_hash: ManagedByteArray<Self::Api, 32>,
        stars: u8,
        summary: ManagedBuffer,
    ) {
        require!(stars <= 5, "Stars must be between 0 and 5");
        require!(!summary.is_empty(), "Summary cannot be empty");
        require!(summary.len() <= 500, "Summary too long");

        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();

        let new_audit = Audit {
            stars,
            summary: summary.clone(),
            auditor: caller.clone(),
            timestamp: current_timestamp,
        };

        // Add to contractAudits
        self.contract_audits(&contract_hash).push(&new_audit);

        // Add to allContractHashes if new
        if !self.hash_exists(&contract_hash).get() {
            self.hash_exists(&contract_hash).set(true);
            self.all_contract_hashes().push(&contract_hash);
        }

        // Check if this is the first audit by this auditor for this contract
        let mut is_new_contract = true;
        let auditor_history_mapper = self.auditor_history(&caller);
        
        for i in 0..auditor_history_mapper.len() {
            let existing_hash = auditor_history_mapper.get(i);
            if existing_hash == contract_hash {
                is_new_contract = false;
                break;
            }
        }
        
        if is_new_contract {
            self.auditor_history(&caller).push(&contract_hash);
        }

        // Emit event
        self.audit_registered_event(
            &contract_hash,
            stars,
            &caller,
            &summary,
        );
    }

    #[view]
    fn get_all_audits(
        &self,
        start_index: usize,
        limit: usize,
    ) -> MultiValueEncoded<
        MultiValue5<
            ManagedByteArray<Self::Api, 32>,
            u8,
            ManagedBuffer,
            ManagedAddress,
            u64,
        >
    > {
        let total_hashes = self.all_contract_hashes().len();
        require!(start_index < total_hashes, "Start index out of bounds");

        let actual_limit = core::cmp::min(limit, total_hashes - start_index);
        let mut result = MultiValueEncoded::new();

        for i in 0..actual_limit {
            let hash = self.all_contract_hashes().get(start_index + i);
            let audits_mapper = self.contract_audits(&hash);
            
            if audits_mapper.len() > 0 {
                let latest_audit = audits_mapper.get(audits_mapper.len() - 1);
                result.push(
                    (
                        hash,
                        latest_audit.stars,
                        latest_audit.summary,
                        latest_audit.auditor,
                        latest_audit.timestamp
                    ).into()
                );
            }
        }

        result
    }

    #[view(getTotalContracts)]
    fn get_total_contracts(&self) -> usize {
        self.all_contract_hashes().len()
    }

    #[view(getContractAudits)]
    fn get_contract_audits(
        &self,
        contract_hash: ManagedByteArray<Self::Api, 32>,
    ) -> MultiValueEncoded<Audit<Self::Api>> {
        let audits_mapper = self.contract_audits(&contract_hash);
        let mut result = MultiValueEncoded::new();
        
        for i in 0..audits_mapper.len() {
            let audit = audits_mapper.get(i);
            result.push(audit);
        }
        
        result
    }

    #[view(getAuditorHistory)]
    fn get_auditor_history(
        &self,
        auditor: ManagedAddress,
    ) -> MultiValueEncoded<ManagedByteArray<Self::Api, 32>> {
        let history_mapper = self.auditor_history(&auditor);
        let mut result = MultiValueEncoded::new();
        
        for i in 0..history_mapper.len() {
            let hash = history_mapper.get(i);
            result.push(hash);
        }
        
        result
    }

    #[view(getLatestAudit)]
    fn get_latest_audit(
        &self,
        contract_hash: ManagedByteArray<Self::Api, 32>,
    ) -> Audit<Self::Api> {
        let audits_mapper = self.contract_audits(&contract_hash);
        require!(audits_mapper.len() > 0, "No audits found for this contract");
        
        let latest_index = audits_mapper.len() - 1;
        audits_mapper.get(latest_index)
    }

    #[endpoint]
    fn withdraw(&self) {
        let caller = self.blockchain().get_caller();
        let owner = self.owner().get();
        
        require!(caller == owner, "Only owner can withdraw");
        
        // Get SC balance directly
        let sc_balance = self.blockchain().get_balance(&self.blockchain().get_sc_address());
        
        // Transfer the balance to the owner
        self.send().direct_egld(&owner, &sc_balance);
    }

    // Storage mappings
    #[view(getOwner)]
    #[storage_mapper("owner")]
    fn owner(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("contractAudits")]
    fn contract_audits(&self, contract_hash: &ManagedByteArray<Self::Api, 32>) 
        -> VecMapper<Audit<Self::Api>>;

    #[storage_mapper("auditorHistory")]
    fn auditor_history(&self, auditor: &ManagedAddress) 
        -> VecMapper<ManagedByteArray<Self::Api, 32>>;

    #[storage_mapper("allContractHashes")]
    fn all_contract_hashes(&self) -> VecMapper<ManagedByteArray<Self::Api, 32>>;

    #[storage_mapper("hashExists")]
    fn hash_exists(&self, contract_hash: &ManagedByteArray<Self::Api, 32>) -> SingleValueMapper<bool>;

    // Events
    #[event("auditRegistered")]
    fn audit_registered_event(
        &self,
        #[indexed] contract_hash: &ManagedByteArray<Self::Api, 32>,
        #[indexed] stars: u8,
        #[indexed] auditor: &ManagedAddress,
        summary: &ManagedBuffer,
    );
}
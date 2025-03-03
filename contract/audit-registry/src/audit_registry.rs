#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct Audit<M: ManagedTypeApi> {
    pub stars: u8,
    pub summary: ManagedBuffer<M>,
    pub auditor: ManagedAddress<M>,
    pub timestamp: u64,
}

#[multiversx_sc::contract]
pub trait AuditRegistry {
    #[init]
    fn init(&self) {}
    
    #[upgrade]
    fn upgrade(&self) {}
    
    /// Register a new audit for a contract
    #[endpoint]
    fn register_audit(
        &self,
        contract_hash: ManagedByteArray<Self::Api, 32>, // 32 bytes hash of the contract
        stars: u8,
        summary: ManagedBuffer<Self::Api>,
    ) {
        // Validation
        require!(stars <= 5, "Stars must be between 0 and 5");
        require!(!summary.is_empty(), "Summary cannot be empty");
        require!(summary.len() <= 500, "Summary too long");
        
        let auditor = self.blockchain().get_caller();
        let timestamp = self.blockchain().get_block_timestamp();
        
        let new_audit = Audit {
            stars,
            summary,
            auditor: auditor.clone(),
            timestamp,
        };
        
        // Store the audit in the contract
        self.contract_audits(&contract_hash).push(&new_audit);
        
        // Add to all contract hashes if new
        if !self.hash_exists(&contract_hash).get() {
            self.hash_exists(&contract_hash).set(true);
            self.all_contract_hashes().push(&contract_hash);
        }
        
        // Check if this is the first audit by this auditor for this contract
        let mut is_new_contract = true;
        let auditor_contracts = self.auditor_history(&auditor);
        
        for i in 0..auditor_contracts.len() {
            if &auditor_contracts.get(i) == &contract_hash {
                is_new_contract = false;
                break;
            }
        }
        
        if is_new_contract {
            self.auditor_history(&auditor).push(&contract_hash);
        }
        
        // Emit event
        self.audit_registered_event(
            &contract_hash,
            stars,
            &summary,
            &auditor,
            timestamp,
        );
    }
    
    /// Get total number of registered contracts
    #[view]
    fn get_total_contracts(&self) -> usize {
        self.all_contract_hashes().len()
    }
    
    /// Get all audits with pagination
    #[view]
    fn get_all_audits(
        &self,
        start_index: usize,
        limit: usize,
    ) -> MultiValueEncoded<MultiValue5<ManagedByteArray<Self::Api, 32>, u8, ManagedBuffer<Self::Api>, ManagedAddress<Self::Api>, u64>> {
        let total_hashes = self.all_contract_hashes().len();
        require!(start_index < total_hashes, "Start index out of bounds");
        
        let actual_limit = core::cmp::min(limit, total_hashes - start_index);
        let mut result = MultiValueEncoded::new();
        
        for i in 0..actual_limit {
            let hash = self.all_contract_hashes().get(start_index + i);
            let audits = self.contract_audits(&hash);
            
            if !audits.is_empty() {
                let latest_audit = audits.get(audits.len() - 1);
                
                result.push((
                    hash,
                    latest_audit.stars,
                    latest_audit.summary,
                    latest_audit.auditor,
                    latest_audit.timestamp,
                ));
            }
        }
        
        result
    }
    
    /// Get all audits for a specific contract
    #[view]
    fn get_contract_audits(
        &self,
        contract_hash: &ManagedByteArray<Self::Api, 32>,
    ) -> MultiValueEncoded<MultiValue4<u8, ManagedBuffer<Self::Api>, ManagedAddress<Self::Api>, u64>> {
        let audits = self.contract_audits(contract_hash);
        let mut result = MultiValueEncoded::new();
        
        for i in 0..audits.len() {
            let audit = audits.get(i);
            result.push((
                audit.stars,
                audit.summary,
                audit.auditor,
                audit.timestamp,
            ));
        }
        
        result
    }
    
    /// Get all contracts audited by a specific auditor
    #[view]
    fn get_auditor_history(
        &self,
        auditor: &ManagedAddress<Self::Api>,
    ) -> MultiValueEncoded<ManagedByteArray<Self::Api, 32>> {
        let history = self.auditor_history(auditor);
        let mut result = MultiValueEncoded::new();
        
        for i in 0..history.len() {
            result.push(history.get(i));
        }
        
        result
    }
    
    /// Get the latest audit for a specific contract
    #[view]
    fn get_latest_audit(
        &self,
        contract_hash: &ManagedByteArray<Self::Api, 32>,
    ) -> MultiValue4<u8, ManagedBuffer<Self::Api>, ManagedAddress<Self::Api>, u64> {
        let audits = self.contract_audits(contract_hash);
        require!(!audits.is_empty(), "No audits found for this contract");
        
        let latest = audits.get(audits.len() - 1);
        (latest.stars, latest.summary, latest.auditor, latest.timestamp).into()
    }
    
    // Storage mappers
    
    #[storage_mapper("contractAudits")]
    fn contract_audits(&self, contract_hash: &ManagedByteArray<Self::Api, 32>) 
        -> VecMapper<Audit<Self::Api>>;
    
    #[storage_mapper("auditorHistory")]
    fn auditor_history(&self, auditor: &ManagedAddress<Self::Api>) 
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
        stars: u8,
        summary: &ManagedBuffer<Self::Api>,
        #[indexed] auditor: &ManagedAddress<Self::Api>,
        timestamp: u64,
    );
}
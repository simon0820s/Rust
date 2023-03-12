//Dependences
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
near_sdk::setup_alloc!();

//class
pub struct Counter {
    value: i8,
}

impl Counter{

    //get counter
    pub fn get_num(&self) -> i8 {
        return self.value;
    }

    //increment counter
    pub fn incrementar(&mut self) {
        self.value += 1;
        let log_message = format!("counter incremented to => {}", self.value);
        env::log(log_message.as_bytes());
    }    
    
    //reduce counter
    pub fn decrementar(&mut self) {
        self.value -= 1;
        let log_message = format!("reduced counter to => {}", self.value);
        env::log(log_message.as_bytes());
    }    

    // Reset counter
    pub fn reset(&mut self) {
        self.value = 0;
        let log_message = format!("counter has been reseted {}", self.value);
        env::log(log_message.as_bytes());
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // Preparación del contaxto de la Blockchain virtual
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext{
        VMContext{
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "bob.testnet".to_string(),
            signer_account_pk: vec![0,1,2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }
    
    // Nuestras pruebas unitarias

    #[test]
    fn incrementar() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Counter{ value: 0};
        contract.incrementar();

        println!("Valor despues del incremento: {}", contract.value);

        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrementar() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Counter{ value: 0};
        contract.decrementar();

        println!("Valor despues del incremento: {}", contract.value);

        assert_eq!(-1, contract.get_num());
    }


    #[test]
    fn reset() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Counter{ value: 10};

        assert_eq!(10, contract.get_num());
        println!("Valor Antes del reset: {}", contract.value);

        contract.reset();

        println!("Valor despues del reset: {}", contract.value);

        assert_eq!(0, contract.get_num());
    }
}
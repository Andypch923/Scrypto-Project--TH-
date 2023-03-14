#![allow(non_snake_case)]
use scrypto::prelude::*;
#[blueprint]
 mod liquidty_pool{
    struct liquidty_pool{
        th_vault: Vault,
        xrd_tokens_vault: Vault,
        price_per_token: Decimal,
        locked: bool,
    }

    impl liquidty_pool{
        pub fn new(price_per_token : Decimal) -> ComponentAddress{
            let bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name","Throne")
                .metadata("symbol","TH")
                .mint_initial_supply(1000);
        

        Self{
            th_vault: Vault::with_bucket(bucket),
            xrd_tokens_vault: Vault::new(RADIX_TOKEN),
            price_per_token: price_per_token,
            locked: true
        }
        .instantiate()
        }

        pub fn instantiate() -> (ComponentAddress, Bucket, Bucket){
            let admin_badge: Bucket = ResourceBuilder::new_fungible()
                .mint_initial_supply(2);
            let moderator_badge: Bucket = ResourceBuilder::new_fungible
                .mint_initial_supply(6);

            let admin_badge_add = admin_badge.resource_address();
            let moderator_badge = moderator_badge.resource_address();
            let mut component = Self {}. instantiate();

            let access_rules = AccessRules::new()
                .default(access_rules);

            component.add_access_check(access_rules);

            (component.globalize(),admin_badge,moderator_badge)
        }

        fn lock_Unlock(){
            if locked = true{
                let locked = false;
            }else{
                let locked = true;
            }
        }   

        pub fn withdraw(&mut self, receiver_address:ComponentAddress,receiver_deposit: Bucket){
            let purchase_amount: Decimal = receiver_deposit.amount()/price_per_token;
            self.xrd_tokens_vault.put(funds);
            self.th_vault.take(purchase_amount);
            receiver_address.th_vault.put(purchase_amount);

            //Add algorithm to check liquidity pool xrd:useful token capacity
        }

        pub fn checkCapacity() -> (String){
            let mut message: String::from("XRD Amount: ");  
            message.push_str(self.xrd_tokens_vault.amount());
            message.push_str("\n");
            message.push_str("TH Amount: ");
            message.push_str(self.th_tokens_vault.amount());
            (message)
        }

        fn mintTokens(&mut self,amount: Decimal){
            self.lock_Unlock();
            
            self.lock_Unlock();
        }

        fn set_exchange_rate(&mut self, amount: Decimal){

        }

        fn checkBadge(user_address: ComponentAddress){

        }

    }
}
    


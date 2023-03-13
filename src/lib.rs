#![allow(non_snake_case)]
use scrypto::prelude::*;
#[blueprint]
mod terminal_hx{
    struct TokenSale{
        useful_tokens_vault: Vault,
        xrd_tokens_vault: Vault,
        price_per_token: Decimal
    }

    impl TokenSale{
        pub fn new(price_per_token : Decimal) -> ComponentAddress{  //function
            let bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name","Useful Token")
                .metadata("symbol","USEFUL")
                .mint_initial_supply(1000);

            Self{
                useful_tokens_vault: Vault::with_bucket(bucket),
                xrd_tokens_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
            }
            .instantiate()
            .globalize()
        }

        pub fn buy(&mut self, funds: Bucket) -> Bucket{      //method
            let purchase_amount: Decimal = funds.amount() / self.price_per_token;
            self.xrd_tokens_vault.put(funds);
            self.useful_tokens_vault.take(purchase_amount)

        }
    }
}



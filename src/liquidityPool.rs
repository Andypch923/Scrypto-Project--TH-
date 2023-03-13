#![allow(non_snake_case)]
use scrypto::prelude::*;
#[blueprint]
 mod liquidty_pool{
    struct liquidty_pool{
        th_vault: Vault,
        xrd_tokens_vault: Vault,
        price_per_token: Decimal
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
            price_per_token: price_per_token
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
                .method()
                .method()
                .default();

            component.add_access_check(access_rules);

            (component.globalize(),admin_badge,moderator_badge)
        }


    }
}
    


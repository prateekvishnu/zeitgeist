#![no_main]

use libfuzzer_sys::fuzz_target;
use zrml_swaps::mock::{ExtBuilder, Origin, Swaps};

use utils::ExactAmountData;
mod utils;
use orml_traits::MultiCurrency;
use utils::construct_asset;
use zeitgeist_primitives::constants::MinLiquidity;
use zrml_swaps::mock::Shares;

fuzz_target!(|data: ExactAmountData| {
    let mut ext = ExtBuilder::default().build();
    let _ = ext.execute_with(|| {
        // ensure that the account origin has a sufficient balance
        // use orml_traits::MultiCurrency; required for this
        for a in &data.pool_creation.assets {
            let _ = Shares::deposit(
                construct_asset(*a),
                &data.pool_creation.origin,
                // In order to successfully join the pool, data.asset_amount more tokens needed
                MinLiquidity::get().saturating_add(data.asset_amount),
            );
        }
        let pool_id = data.pool_creation._create_pool();
        let _ = Swaps::pool_join_with_exact_pool_amount(
            Origin::signed(data.origin),
            pool_id,
            construct_asset(data.asset),
            data.pool_amount,
            data.asset_amount,
        );
    });
    let _ = ext.commit_all();
});

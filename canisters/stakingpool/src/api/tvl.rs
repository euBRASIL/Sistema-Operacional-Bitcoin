use ic_cdk::api::management_canister::bitcoin::Satoshi;

use crate::repositories;

pub(super) fn serve() -> Satoshi {
    repositories::staking_record::sum_staking_amount()
}

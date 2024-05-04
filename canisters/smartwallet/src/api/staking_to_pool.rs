use ic_cdk::api::management_canister::main::CanisterId;
use wallet::{bitcoins, utils::ic_time};

use crate::{
    domain::{
        request::{StakingRequest, TransferInfo, TransferRequest},
        Metadata, StakingRecord,
    },
    error::WalletError,
    repositories,
};

use super::transfer_from_p2pkh;

pub(super) async fn serve(
    public_key: &[u8],
    metadata: Metadata,
    sender_canister: CanisterId,
    req: StakingRequest,
) -> Result<String, WalletError> {
    let tx_req = TransferRequest {
        txs: vec![TransferInfo {
            recipient: req.staking_address.clone(),
            amount: req.amount,
        }],
    };

    let network = metadata.network;
    let sender = metadata.owner;
    let sender_address = bitcoins::public_key_to_p2pkh_address(network, public_key);
    let txid = transfer_from_p2pkh::serve(public_key, metadata, tx_req).await?;

    // Save Staking record in wallet
    let stakings = StakingRecord {
        txid: txid.clone(),
        sender,
        sender_canister,
        sender_address,
        sent_amount: req.amount,
        sent_time: ic_time(),
        recipient: req.staking_canister,
        recipient_address: req.staking_address,
        network,
    };

    repositories::staking_record::save(stakings)?;

    Ok(txid)
}

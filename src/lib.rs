mod abi;
mod pb;
use hex_literal::hex;
use pb::eth::mailbox::v1 as mailbox;
use substreams::prelude::*;
use substreams::{log, Hex};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

// Hyperlane Mailbox Contract
const TRACKED_CONTRACT: [u8; 20] = hex!("35231d4c2D8B8ADcB5617A638A0c4548684c7C70");

substreams_ethereum::init!();

/// Extracts dispatches events from the contract
#[substreams::handlers::map]
fn map_dispatches(blk: eth::Block) -> Result<Option<mailbox::Dispatches>, substreams::errors::Error> {
    let dispatches: Vec<_> = blk
        .events::<abi::mailbox::events::Dispatch>(&[&TRACKED_CONTRACT])
        .map(|(dispatch, log)| {
            substreams::log::info!("Mailbox Dispatch seen");

            mailbox::Dispatch {
                sender: Hex::encode(&dispatch.sender),
                destination: dispatch.destination.to_u64() as u32,
                recipient: Hex::encode(&dispatch.recipient),
                body: Hex::encode(&dispatch.message),
                ordinal: log.block_index() as u64,
                trx_hash: Hex::encode(&log.receipt.transaction.hash),
            }
        })
        .collect();
    if dispatches.len() == 0 {
        return Ok(None);
    }

    Ok(Some(mailbox::Dispatches { dispatches }))
}

/// Store the total balance of NFT tokens for the specific TRACKED_CONTRACT by holder
#[substreams::handlers::store]
fn store_dispatches(dispatches: mailbox::Dispatches, s: StoreSetString) {
    log::info!("Mailbox messages state builder");
    for dispatch in dispatches.dispatches {
        s.set(dispatch.ordinal, generate_key(&dispatch.body), &dispatch.body);
    }
}

#[substreams::handlers::map]
fn db_out(
    dispatches: mailbox::Dispatches,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    for dispatch in dispatches.dispatches {
        tables
            .create_row(
                "dispatch",
                format!("{}-{}", &dispatch.trx_hash, &dispatch.ordinal),
            )
            .set("trx_hash", dispatch.trx_hash)
            .set("sender", dispatch.sender)
            .set("destination", dispatch.destination)
            .set("recipient", &dispatch.recipient)
            .set("body", &dispatch.body)
            .set("ordinal", dispatch.ordinal);
    }

    Ok(tables.to_database_changes())
}

fn generate_key(body: &String) -> String {
    return format!("{}:dispatched:{}:", Hex(TRACKED_CONTRACT), body);
}

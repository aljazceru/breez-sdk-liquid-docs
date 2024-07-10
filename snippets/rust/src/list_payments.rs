use std::sync::Arc;

use anyhow::Result;
use breez_sdk_liquid::prelude::*;

async fn list_payments(sdk: Arc<LiquidSdk>) -> Result<Vec<Payment>> {
    // ANCHOR: list-payments
    let payments = sdk.list_payments().await?;
    // ANCHOR_END: list-payments

    Ok(payments)
}

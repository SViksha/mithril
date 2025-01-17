use mithril_common::store::adapter::AdapterError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("physical adapter returned an error: {0}")]
    AdapterError(#[from] AdapterError),
}

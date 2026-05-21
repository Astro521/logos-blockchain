use lb_log_targets_macros::log_targets;

log_targets! {
    root = tx_service;

    tx::{
        SERVICE,
    },
    backend::{
        POOL,
    },
    network::{
        LIBP2P,
        MOCK,
    },
}

/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// Note: final re-exports from godot-core are in lib.rs, mod register::private.
// These are public here for simplicity, but many are not imported by the main crate.

pub mod callbacks;
pub mod class;
pub mod constant;
pub mod method;
pub mod plugin;
pub mod property;

#[cfg(since_api = "4.2")]
pub mod signal;

// Contents re-exported in `godot` crate; just keep empty.
#[cfg(before_api = "4.2")]
pub mod signal {
    pub mod re_export {}
}

// RpcConfig uses MultiplayerPeer::TransferMode and MultiplayerApi::RpcMode, which are only enabled in `codegen-full` feature.
#[cfg(feature = "codegen-full")]
mod rpc_config;
#[cfg(feature = "codegen-full")]
pub use rpc_config::RpcConfig;

#[doc(hidden)]
pub mod godot_register_wrappers;

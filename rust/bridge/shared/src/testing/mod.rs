//
// Copyright 2023 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use futures_util::FutureExt;
use libsignal_bridge_macros::*;
use libsignal_protocol::SignalProtocolError;

use std::future::Future;

use crate::support::*;
use crate::*;

#[cfg(feature = "node")]
mod net;
mod types;
use types::*;

pub struct NonSuspendingBackgroundThreadRuntime;
bridge_handle!(
    NonSuspendingBackgroundThreadRuntime,
    clone = false,
    ffi = testing_NonSuspendingBackgroundThreadRuntime,
    jni = TESTING_1NonSuspendingBackgroundThreadRuntime
);

impl<F> AsyncRuntime<F> for NonSuspendingBackgroundThreadRuntime
where
    F: Future<Output = ()> + Send + 'static,
{
    fn run_future(&self, future: F) {
        std::thread::spawn(move || {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
                future
                    .now_or_never()
                    .expect("no need to suspend in testing methods")
            }))
            .unwrap_or_else(|_| {
                // Since this is a testing method, make sure we crash on uncaught panics.
                std::process::abort()
            })
        });
    }
}

#[bridge_fn(ffi = false, jni = false)]
fn TESTING_NonSuspendingBackgroundThreadRuntime_New() -> NonSuspendingBackgroundThreadRuntime {
    NonSuspendingBackgroundThreadRuntime
}

#[bridge_io(NonSuspendingBackgroundThreadRuntime)]
async fn TESTING_FutureSuccess(input: u8) -> i32 {
    i32::from(input) * 2
}

#[bridge_io(NonSuspendingBackgroundThreadRuntime)]
async fn TESTING_FutureFailure(_input: u8) -> Result<i32, SignalProtocolError> {
    Err(SignalProtocolError::InvalidArgument("failure".to_string()))
}

#[bridge_fn]
fn TESTING_PanicOnBorrowSync(_input: Ignored<PanicOnBorrow>) {}

#[bridge_fn]
async fn TESTING_PanicOnBorrowAsync(_input: Ignored<PanicOnBorrow>) {}

#[bridge_io(NonSuspendingBackgroundThreadRuntime)]
async fn TESTING_PanicOnBorrowIo(_input: Ignored<PanicOnBorrow>) {}

#[bridge_fn]
fn TESTING_ErrorOnBorrowSync(_input: Ignored<ErrorOnBorrow>) {}

#[bridge_fn]
async fn TESTING_ErrorOnBorrowAsync(_input: Ignored<ErrorOnBorrow>) {}

#[bridge_io(NonSuspendingBackgroundThreadRuntime)]
async fn TESTING_ErrorOnBorrowIo(_input: Ignored<ErrorOnBorrow>) {}

#[bridge_fn]
fn TESTING_PanicOnLoadSync(_needs_cleanup: Ignored<NeedsCleanup>, _input: Ignored<PanicOnLoad>) {}

#[bridge_fn]
async fn TESTING_PanicOnLoadAsync(
    _needs_cleanup: Ignored<NeedsCleanup>,
    _input: Ignored<PanicOnLoad>,
) {
}

#[bridge_io(NonSuspendingBackgroundThreadRuntime)]
async fn TESTING_PanicOnLoadIo(
    _needs_cleanup: Ignored<NeedsCleanup>,
    _input: Ignored<PanicOnLoad>,
) {
}

#[bridge_fn]
fn TESTING_PanicInBodySync(_input: Ignored<NeedsCleanup>) {
    panic!("deliberate panic");
}

#[bridge_fn]
async fn TESTING_PanicInBodyAsync(_input: Ignored<NeedsCleanup>) {
    panic!("deliberate panic");
}

#[bridge_io(NonSuspendingBackgroundThreadRuntime)]
async fn TESTING_PanicInBodyIo(_input: Ignored<NeedsCleanup>) {
    panic!("deliberate panic");
}

#[bridge_fn]
fn TESTING_PanicOnReturnSync(_needs_cleanup: Ignored<NeedsCleanup>) -> Ignored<PanicOnReturn> {
    PanicOnReturn
}

#[bridge_fn]
async fn TESTING_PanicOnReturnAsync(
    _needs_cleanup: Ignored<NeedsCleanup>,
) -> Ignored<PanicOnReturn> {
    PanicOnReturn
}

#[bridge_io(NonSuspendingBackgroundThreadRuntime)]
async fn TESTING_PanicOnReturnIo(_needs_cleanup: Ignored<NeedsCleanup>) -> Ignored<PanicOnReturn> {
    PanicOnReturn
}

#[bridge_fn]
fn TESTING_ErrorOnReturnSync(_needs_cleanup: Ignored<NeedsCleanup>) -> Ignored<ErrorOnReturn> {
    ErrorOnReturn
}

#[bridge_fn]
async fn TESTING_ErrorOnReturnAsync(
    _needs_cleanup: Ignored<NeedsCleanup>,
) -> Ignored<ErrorOnReturn> {
    ErrorOnReturn
}

#[bridge_io(NonSuspendingBackgroundThreadRuntime)]
async fn TESTING_ErrorOnReturnIo(_needs_cleanup: Ignored<NeedsCleanup>) -> Ignored<ErrorOnReturn> {
    ErrorOnReturn
}

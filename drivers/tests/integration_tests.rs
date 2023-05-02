// Licensed under the Apache-2.0 license

use caliptra_builder::FwId;
use caliptra_hw_model::{BootParams, HwModel, InitParams};

fn run_driver_test(test_bin_name: &str) {
    let rom = caliptra_builder::build_firmware_rom(&FwId {
        crate_name: "caliptra-drivers-test-bin",
        bin_name: test_bin_name,
        features: &["emu"],
    })
    .unwrap();
    let mut model = caliptra_hw_model::new(BootParams {
        init_params: InitParams {
            rom: &rom,
            ..Default::default()
        },
        ..Default::default()
    })
    .unwrap();

    // Wrap in a line-writer so output from different test threads doesn't multiplex within a line.
    model.step_until_exit_success().unwrap();
}

#[test]
fn test_doe() {
    run_driver_test("doe");
}

#[test]
fn test_ecc384() {
    run_driver_test("ecc384");
}

#[test]
fn test_error_reporter() {
    run_driver_test("error_reporter");
}

#[test]
fn test_hmac384() {
    run_driver_test("hmac384");
}

#[test]
fn test_keyvault() {
    run_driver_test("keyvault");
}

#[test]
fn test_pcrbank() {
    run_driver_test("pcrbank");
}

#[test]
fn test_sha1() {
    run_driver_test("sha1");
}

#[test]
fn test_sha256() {
    run_driver_test("sha256");
}

#[test]
fn test_sha384() {
    run_driver_test("sha384");
}

#[test]
fn test_sha384acc() {
    run_driver_test("sha384acc");
}

#[test]
fn test_status_reporter() {
    run_driver_test("status_reporter");
}

#[test]
fn test_lms_24() {
    run_driver_test("test_lms_24");
}

#[test]
fn test_lms_32() {
    run_driver_test("test_lms_32");
}

#[test]
fn test_negative_lms() {
    run_driver_test("test_negative_lms");
}

#[test]
#[cfg(feature = "verilator")]
fn test_csrng() {
    // https://github.com/chipsalliance/caliptra-rtl/blob/fa91d66f30223899403f4e65a6f697a6f9100fd1/src/csrng/tb/csrng_tb.sv#L461
    const TRNG_ENTROPY: &str = "33F63B65F57AD68765693560E743CC5010518E4BF4ECBEBA71DC56AAA08B394311731D9DF763FC5D27E4ED3E4B7DE947";

    let rom = caliptra_builder::build_firmware_rom(&FwId {
        crate_name: "caliptra-drivers-test-bin",
        bin_name: "csrng",
        features: &["emu"],
    })
    .unwrap();

    let trng_nibbles = TRNG_ENTROPY
        .chars()
        .rev()
        .map(|b| b.to_digit(16).expect("bad nibble digit") as u8);

    let mut model = caliptra_hw_model::new(BootParams {
        init_params: InitParams {
            rom: &rom,
            trng_nibbles: Box::new(trng_nibbles),
            ..Default::default()
        },
        ..Default::default()
    })
    .unwrap();

    model.step_until_exit_success().unwrap();
}
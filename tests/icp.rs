use std::fs;

use candid::{encode_one, Principal};
use pocket_ic::{PocketIc, PocketIcBuilder, UserError, WasmResult};

// 2T cycles
const INIT_CYCLES: u128 = 2_000_000_000_000;
const WASM_PATH: &str = "../target/wasm32-unknown-unknown/release/icp.wasm";

fn setup() -> (PocketIc, Principal) {
    let mut pic = PocketIcBuilder::new()
        .with_nns_subnet()
        .with_application_subnet()
        .build();

    let _endpoint = pic.make_live(None);

    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, INIT_CYCLES);

    let wasm = load_wasm();
    pic.install_canister(canister_id, wasm, vec![], None);

    (pic, canister_id)
}

fn load_wasm() -> Vec<u8> {
    fs::read(WASM_PATH).expect("Wasm file not found, run 'dfx build'.")
}

fn update_call(
    pic: &PocketIc,
    canister_id: Principal,
    sender: Principal,
    method: &str,
    args: Vec<u8>,
) -> Result<WasmResult, UserError> {
    pic.update_call(canister_id, sender, method, args)
}

#[test]
fn test_deposit() {
    let (pic, canister_id) = setup();

    let reply = update_call(
        &pic,
        canister_id,
        Principal::anonymous(),
        "deposit",
        encode_one(Principal::anonymous()).unwrap(),
    )
    .unwrap();

    // assert_eq!(reply, Ok(vec![0, 0, 0, 0]));

    println!("{:?}", reply);
}

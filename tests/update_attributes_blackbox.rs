use multiversx_sc_scenario::imports::*;
mod proxy;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("update-attributes");
const CODE_PATH: MxscPath = MxscPath::new("output/update-attributes.mxsc.json");
const TOKEN_NAME: &str = "Test";
const TOKEN_TICKER: &str = "TSTT";
const ISSUE_COST: u64 = 50000000000000000;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(CODE_PATH, update_attributes::ContractBuilder);

    blockchain
        .account(OWNER_ADDRESS)
        .nonce(1)
        .balance(5000000000000000000u64);

    blockchain
}

#[test]
fn issue_fungible_token_mapper() {
    let mut world = world();
    world.start_trace();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .init()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .issue_fungible_token_mapper(TOKEN_NAME, TOKEN_TICKER)
        .run();

    world.write_scenario_trace("scenarios/issue_fungible_token_mapper.scen.json");
}

#[test]
fn issue_non_fungible() {
    let mut world = world();
    world.start_trace();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .init()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .issue_non_fungible(TOKEN_NAME, TOKEN_TICKER)
        .egld(ISSUE_COST)
        .run();

    world.write_scenario_trace("scenarios/issue_non_fungible.scen.json");
}

#[test]
fn issue_fungible() {
    let mut world = world();
    world.start_trace();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .init()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .issue_fungible(TOKEN_NAME, TOKEN_TICKER, 100u64)
        .run();

    world.write_scenario_trace("scenarios/issue_fungible.scen.json");
}

#[test]
fn set_roles() {
    let mut world = world();
    world.start_trace();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .init()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .set_roles()
        .run();

    world.write_scenario_trace("scenarios/set_roles.scen.json");
}

#[test]
fn issue_fungible_and_non_fungible() {
    let mut world = world();
    world.start_trace();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .init()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .issue_fungible_token_mapper(TOKEN_NAME, TOKEN_TICKER)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .issue_non_fungible(TOKEN_NAME, TOKEN_TICKER)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .issue_fungible(TOKEN_NAME, TOKEN_TICKER, 100u64)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .set_roles()
        .run();

    world.write_scenario_trace("scenarios/full_functionality.scen.json");
}

#[test]
fn update_attributes() {
    let mut world = world();
    world.start_trace();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .init()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .set_roles()
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .issue_non_fungible(TOKEN_NAME, TOKEN_TICKER)
        .egld(ISSUE_COST)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .create_nft(OWNER_ADDRESS)
        .run();

    let token_id = world
        .query()
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .nft_token_id()
        .returns(ReturnsResult)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .update_attributes(ManagedBuffer::new_from_bytes(&b"Burnable"[..]))
        .egld_or_single_esdt(
            &EgldOrEsdtTokenIdentifier::esdt(TokenIdentifier::from(token_id)),
            1,
            &BigUint::from(1u64),
        )
        .run();

    world.write_scenario_trace("scenarios/update_attributes.scen.json");
}

#[test]
fn create_nft() {
    let mut world = world();
    world.start_trace();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .init()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .set_roles()
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .issue_non_fungible(TOKEN_NAME, TOKEN_TICKER)
        .egld(ISSUE_COST)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .create_nft(OWNER_ADDRESS)
        .run();

    world.write_scenario_trace("scenarios/create_nft.scen.json");
}

#[test]
fn send_nft() {
    let mut world = world();
    world.start_trace();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .init()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .set_roles()
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .issue_non_fungible(TOKEN_NAME, TOKEN_TICKER)
        .egld(ISSUE_COST)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .create_nft(SC_ADDRESS)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .send_nft(OWNER_ADDRESS, 1u64)
        .run();

    world.write_scenario_trace("scenarios/send_nft.scen.json");
}

#[test]
fn nft_token_id() {
    let mut world = world();
    world.start_trace();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .init()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .issue_non_fungible(TOKEN_NAME, TOKEN_TICKER)
        .egld(ISSUE_COST)
        .run();

    let nft_token_id = world
        .query()
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .nft_token_id()
        .returns(ReturnsResult)
        .run();

    assert_eq!(nft_token_id, TokenIdentifier::from(""));

    world.write_scenario_trace("scenarios/nft_token_id.scen.json");
}

#[test]
fn test_token_mapper() {
    let mut world = world();
    world.start_trace();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .init()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .issue_fungible(TOKEN_NAME, TOKEN_TICKER, 100u64)
        .egld(ISSUE_COST)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .issue_fungible_token_mapper(TOKEN_NAME, TOKEN_TICKER)
        .egld(ISSUE_COST)
        .run();

    let token_mapper = world
        .query()
        .to(SC_ADDRESS)
        .typed(proxy::UpdateAttributesProxy)
        .test_token_mapper()
        .returns(ReturnsResult)
        .run();

    assert_eq!(token_mapper.ticker().to_string(), TOKEN_TICKER);

    world.write_scenario_trace("scenarios/test_token_mapper.scen.json");
}

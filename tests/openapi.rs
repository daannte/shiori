use shiori::BaseOpenApi;

#[test]
fn test_open_api_snapshot() {
    let (_, newest) = BaseOpenApi::build();
    insta::assert_snapshot!(newest.to_pretty_json().unwrap())
}

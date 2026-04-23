use std::sync::Arc;

use shiori::BaseOpenApi;
use shiori_core::ShioriCore;

#[test]
fn test_open_api_snapshot() {
    let core = ShioriCore::new();
    let app = Arc::new(core.get_app());
    let (_, newest) = BaseOpenApi::build(app);
    insta::assert_snapshot!(newest.to_pretty_json().unwrap())
}

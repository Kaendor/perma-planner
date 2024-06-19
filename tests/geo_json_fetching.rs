use perma_planner::CartoClient;

#[test]
fn get_commune_svg() {
    let svg = CartoClient::get_commune_svg("78646").unwrap();
    assert!(!svg.is_empty());
}

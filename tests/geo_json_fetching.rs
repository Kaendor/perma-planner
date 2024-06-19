use geojson::{FeatureCollection, GeoJson};
use perma_planner::CartoClient;

#[test]
fn fetch_geo_json() {
    let json = CartoClient::get_commune("78646").unwrap();

    assert!(matches!(json, GeoJson::FeatureCollection(..)));

    let features: FeatureCollection = json.try_into().unwrap();

    assert_eq!(features.features, vec![]);
}

#[test]
fn convert_json_to_svg() {
    let json = CartoClient::get_commune("78646").unwrap();
    let svg = CartoClient::json_to_svg(json).unwrap();
}

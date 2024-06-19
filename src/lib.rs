use eyre::Result;
use geojson::GeoJson;
use geozero::geojson::GeoJsonString;
use geozero::ToSvg;

pub struct CartoClient {}

impl CartoClient {
    pub fn new() -> Self {
        CartoClient {}
    }

    pub fn get_commune_svg(code_insee: &str) -> Result<String> {
        let json = CartoClient::get_commune(code_insee)?;
        let svg = CartoClient::json_to_svg(json)?;

        dbg!(&svg);

        Ok(svg)
    }

    fn get_commune(code_insee: &str) -> Result<GeoJson> {
        let url = format!(
            "https://apicarto.ign.fr/api/cadastre/commune?code_insee={}",
            code_insee
        );

        let response = reqwest::blocking::get(&url)?;
        let map: GeoJson = response.json()?;
        Ok(map)
    }

    fn json_to_svg(json: GeoJson) -> Result<String> {
        let string = json.to_string();
        let svg = GeoJsonString(string).to_svg_document()?;

        Ok(svg)
    }
}

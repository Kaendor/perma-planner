use geojson::GeoJson;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let geo_json: GeoJson =
        reqwest::blocking::get("https://apicarto.ign.fr/api/cadastre/commune?code_insee=75056")?
            .json()?;
    println!("body = {:?}", geo_json);

    Ok(())
}

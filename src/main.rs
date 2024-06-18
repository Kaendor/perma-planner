fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body =
        reqwest::blocking::get("https://apicarto.ign.fr/api/cadastre/commune?code_insee=01384")?
            .text()?;
    println!("body = {:?}", body);

    Ok(())
}

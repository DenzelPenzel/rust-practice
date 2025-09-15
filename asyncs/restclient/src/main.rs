use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Weather {
    latitude: f64,
    longitude: f64,
    current_weather: CurrentWeather,
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=38.9517&longitude=-92.3341&current_weather=true";
    let resp = reqwest::get(URL).await?;
    let weather: serde_json::Value = resp.json().await?;
    println!("{weather:#?}");

    Ok(())
}

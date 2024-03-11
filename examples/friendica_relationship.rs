use megalodon::{entities, error, generator};
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();

    let Ok(url) = env::var("FRIENDICA_URL") else {
        println!("Specify FRIENDICA_URL!!");
        return;
    };
    let Ok(token) = env::var("FRIENDICA_ACCESS_TOKEN") else {
        println!("Specify FRIENDICA_ACCESS_TOKEN!!");
        return;
    };

    let res = get_relationship(url.as_str(), token, "2245768").await;
    match res {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

async fn get_relationship(
    url: &str,
    access_token: String,
    id: &str,
) -> Result<Vec<entities::Relationship>, error::Error> {
    let client = generator(
        megalodon::SNS::Friendica,
        url.to_string(),
        Some(access_token),
        None,
    );
    let res = client.get_relationships([id.to_string()].to_vec()).await?;
    Ok(res.json())
}

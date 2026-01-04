use serde::Serialize;

use crate::{LobClient, MailType, Size, UseType};

#[tokio::test]
async fn test() {
    let _ = dotenvy::dotenv();

    let api_key = match std::env::var("LOB_API_KEY") {
        Ok(ok) => ok,
        Err(err) => {
            eprintln!(
                "Did Not Run Geocodio Test Because `LOB_API_KEY` Could Not Be Retrieved From Environment Variables: {err}"
            );
            return;
        }
    };

    let client = reqwest::Client::new();

    let lob_client = LobClient::builder()
        .client(client)
        .api_key(api_key.as_str())
        .build();

    const CASE_NUMBER: &str = "JP01-25-E01";
    const FRONT_HTML: &str = r#"
<html>
    <head>
    <style>
        p {
            padding: 20px;
        }
    </style>
    </head>
    <body>
        <p>Hello {{case_number}}!</p>
    </body>
</html>
    "#;
    const BACK_HTML: &str = r#"
<html>
    <head>
    <style>
        p {
            padding: 20px;
        }
    </style>
    </head>
    <body>
        <p>Back Side: {{case_number}}!</p>
    </body>
</html>
    "#;

    #[derive(Serialize)]
    struct MergeVar {
        case_number: &'static str,
    }

    let response = lob_client
        .create_postcard()
        .idempotency_key(CASE_NUMBER)
        .name("Spanish Consulate")
        .address_line_1("1800 Bering Dr")
        .address_line_2("750")
        .city("Houston")
        .state("TX")
        .zip_code("77057")
        .front(FRONT_HTML)
        .back(BACK_HTML)
        .use_type(UseType::Marketing)
        .size(Size::FourBySix)
        .mail_type(MailType::UspsFirstClass)
        .description("Postcard To Spanish Consulate")
        .merge(MergeVar {
            case_number: CASE_NUMBER,
        })
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}

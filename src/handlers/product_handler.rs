use std::{fmt::Error, iter::Product, path::PathBuf, str, string};
use axum::{
    body::{Body, Bytes}, extract::{Extension, Json, Path, Query, Request}, handler::Handler, http::{header, response, HeaderMap, StatusCode}, response::IntoResponse, routing::{get, post}
};

use lettre::{message::{header::{ContentType, Headers}, Attachment, MultiPart, SinglePart}, transport::smtp::{authentication::Credentials, commands::Mail}, Message, SmtpTransport, Transport};
use serde::{de::value::EnumAccessDeserializer, Deserialize, Serialize};
use utoipa::{openapi::{self, path::Parameter}, OpenApi};
use std::fs;


/// Sends an order aliexpress and mail
#[utoipa::path(
    post,
    path = "/send_order/{adress}/{mail}",
    params(
        ("adress" = String, Path, description = "adressen"),
        ("mail" = String, Path, description = "mailen meldingen blir sendt til"),
        //("image" = )
    ),
    responses(
        (status = 200, description = "An order is sent on mail but not to aliexpress"),
        (status = 201, description = "An order is sendt", body = String)
    )
    // dokumenter input
    // leg til produkt og adresse osv
)]
pub async fn send_order(
    Path((adress, mail)): Path<(String, String)>,
    body : Bytes
) -> impl IntoResponse {
    let content_type = ContentType::parse("image/png").unwrap();
    let attachment = Attachment::new("image.png".to_string()).body(body.to_vec(), content_type);
    println!("{:?}", attachment);    

    let to : String = format!("Andre <{}>", mail);

    println!("{}", to);

    let email : Message = Message::builder()
        .from("Intern <infojobb7567@gmail.com>".parse().unwrap())
        //.to(mail.parse().unwrap())
        //.to("Marcus <marcusskarmothomassen@gmail.com>".parse().unwrap())
        .to(to.parse().unwrap())
        .subject("bildetest")
        .multipart(
            MultiPart::mixed()
                .singlepart(
                    SinglePart::plain(adress.to_string()),
                )
                .singlepart(attachment)
        )
        .unwrap();

    let cred = Credentials::new("infojobb7567@gmail.com".to_string(), "vpss jfuv vtts xrdk".to_string());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(cred)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent seccessfully!"),
        Err(e) => eprint!("Failed to send email: {:?}", e),
    }
    ""
}
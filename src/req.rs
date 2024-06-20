use std::str::FromStr;
use serde::{Serialize, Deserialize};
use url::Url;
use reqwest::{Client, header, header::HeaderMap, Method, RequestBuilder, Response};
use anyhow::Result;
use reqwest::header::{HeaderName, HeaderValue};
use serde_json::json;
use crate::ExtraArgs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestProfile {
    #[serde(with = "http_serde::method", default)]
    pub method: Method,
    pub url: Url,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub params: Option<serde_json::Value>,
    #[serde(
    skip_serializing_if = "HeaderMap::is_empty",
    with = "http_serde::header_map",
    default
    )]
    pub headers: HeaderMap,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub body: Option<serde_json::Value>,
}

#[derive(Debug)]
pub struct ResponseExt(Response);

impl RequestProfile {
    pub async fn send(&self, args: &super::ExtraArgs) -> Result<ResponseExt> {
        let mut req: RequestBuilder
            = reqwest::Client::new().request(self.method.clone(), self.url.clone());
        let (header, query, body) = self.generate(args)?;
        let client = Client::new();
        let req = client
            .request(self.method.clone())
            .query(&query)
            .headers(header)
            .body(body)
            .build()?;
        let res = client.execute(req).await?;
        Ok(ResponseExt(res))
    }

    pub fn generate(
        &self,
        args: &ExtraArgs,
    ) -> Result<(HeaderMap, serde_json::Value, String)> {
        let mut headers: HeaderMap = self.headers.clone();
        let mut query = self.params.clone().unwrap_or_else(|| serde_json::json!({}));
        let mut body = self.body.clone().unwrap_or_else(|| serde_json::json!({}));
        for (k, v) in &args.headers {
            headers.insert(HeaderName::from_str(k)?, HeaderValue::from_str(v)?);
        }

        if !headers.contains_key(header::CONTENT_TYPE) {
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
        }

        for (k, v) in &args.query {
            query[k] = v.parse()?;
        }
        for (k, v) in &args.body {
            body[k] = v.parse()?;
        }

        let content_type = get_content_type(&headers);

        match content_type.as_deref() {
            Some("application/json") => {
                let body = serde_json::to_string(&body)?;
                Ok((headers, query, body))
            }
            Some("application/x-www-form-urlencoded" |
                 "multipart/form-data") => {
                let body = serde_urlencoded::to_string(&body)?;
                Ok((headers, query, body))
            }
            _ => Err(anyhow::anyhow!("Unsupported content type: {}", content_type.unwrap_or_default())),
        }
    }
}

impl ResponseExt {
    pub fn filter_text(self, profile: &RequestProfile) -> Result<String> {
        let  res = self.0;
        let mut output = String::new();
        output.push_str(&format!("{:?} {}\r", res.version(), res.status()));
        let headers = res.headers();
        for (k, v) in headers.iter() {
            if !profile.skip_headers.iter().any(|sh| sh == k.as_str()) {
                output.push_str(&format!("{}: {:?}\r", k, v));
            }
        }
        output.push_str("\n");

        let content_type = get_content_type(&headers);
        let text = res.text().await?;
        match content_type.as_deref() {
            Some("application/json") => {
                let text = filter_json(&text, &profile.skip_body)?;
                output.push_str(&text);
            }
            _ => output.push_str(&text),
        }
        Ok(output)
    }
}

fn filter_json(text: &str, skip: &[String]) -> Result<String> {
    let mut json: serde_json::Value = serde_json::from_str(text)?;

    match json {
        serde_json::Value::Object(ref mut obj) => {
            for k in skip {
                obj.remove(k);
            }
        }
        _ => {}
    }

    for k in skip {
        json.as_object_mut().unwrap().remove(k);
        json[k] = json!(null);
    }
    Ok(serde_json::to_string_pretty(&json)?)
}

fn get_content_type(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().split(';').next())
        .flatten()
        .map(|v| v.to_string())
}

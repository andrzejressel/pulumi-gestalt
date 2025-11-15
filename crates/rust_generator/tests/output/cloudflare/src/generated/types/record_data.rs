#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RecordData {
    #[builder(into)]
    #[serde(rename = "algorithm")]
    pub r#algorithm: Option<i32>,
    #[builder(into)]
    #[serde(rename = "altitude")]
    pub r#altitude: Option<f64>,
    #[builder(into)]
    #[serde(rename = "certificate")]
    pub r#certificate: Option<String>,
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    #[builder(into)]
    #[serde(rename = "digest")]
    pub r#digest: Option<String>,
    #[builder(into)]
    #[serde(rename = "digestType")]
    pub r#digest_type: Option<i32>,
    #[builder(into)]
    #[serde(rename = "fingerprint")]
    pub r#fingerprint: Option<String>,
    #[builder(into)]
    #[serde(rename = "flags")]
    pub r#flags: Option<String>,
    #[builder(into)]
    #[serde(rename = "keyTag")]
    pub r#key_tag: Option<i32>,
    #[builder(into)]
    #[serde(rename = "latDegrees")]
    pub r#lat_degrees: Option<i32>,
    #[builder(into)]
    #[serde(rename = "latDirection")]
    pub r#lat_direction: Option<String>,
    #[builder(into)]
    #[serde(rename = "latMinutes")]
    pub r#lat_minutes: Option<i32>,
    #[builder(into)]
    #[serde(rename = "latSeconds")]
    pub r#lat_seconds: Option<f64>,
    #[builder(into)]
    #[serde(rename = "longDegrees")]
    pub r#long_degrees: Option<i32>,
    #[builder(into)]
    #[serde(rename = "longDirection")]
    pub r#long_direction: Option<String>,
    #[builder(into)]
    #[serde(rename = "longMinutes")]
    pub r#long_minutes: Option<i32>,
    #[builder(into)]
    #[serde(rename = "longSeconds")]
    pub r#long_seconds: Option<f64>,
    #[builder(into)]
    #[serde(rename = "matchingType")]
    pub r#matching_type: Option<i32>,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Option<i32>,
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    #[builder(into)]
    #[serde(rename = "precisionHorz")]
    pub r#precision_horz: Option<f64>,
    #[builder(into)]
    #[serde(rename = "precisionVert")]
    pub r#precision_vert: Option<f64>,
    #[builder(into)]
    #[serde(rename = "preference")]
    pub r#preference: Option<i32>,
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Option<i32>,
    #[builder(into)]
    #[serde(rename = "proto")]
    pub r#proto: Option<String>,
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<i32>,
    #[builder(into)]
    #[serde(rename = "publicKey")]
    pub r#public_key: Option<String>,
    #[builder(into)]
    #[serde(rename = "regex")]
    pub r#regex: Option<String>,
    #[builder(into)]
    #[serde(rename = "replacement")]
    pub r#replacement: Option<String>,
    #[builder(into)]
    #[serde(rename = "selector")]
    pub r#selector: Option<i32>,
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Option<String>,
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Option<f64>,
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Option<String>,
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Option<String>,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<i32>,
    #[builder(into)]
    #[serde(rename = "usage")]
    pub r#usage: Option<i32>,
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: Option<i32>,
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFunctionUrlCor {
    #[builder(into)]
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: bool,
    #[builder(into)]
    #[serde(rename = "allowHeaders")]
    pub r#allow_headers: Vec<String>,
    #[builder(into)]
    #[serde(rename = "allowMethods")]
    pub r#allow_methods: Vec<String>,
    #[builder(into)]
    #[serde(rename = "allowOrigins")]
    pub r#allow_origins: Vec<String>,
    #[builder(into)]
    #[serde(rename = "exposeHeaders")]
    pub r#expose_headers: Vec<String>,
    #[builder(into)]
    #[serde(rename = "maxAge")]
    pub r#max_age: i32,
}

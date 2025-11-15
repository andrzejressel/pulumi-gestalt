#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomHostnameSslValidationRecord {
    #[builder(into)]
    #[serde(rename = "cnameName")]
    pub r#cname_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "cnameTarget")]
    pub r#cname_target: Option<String>,
    #[builder(into)]
    #[serde(rename = "emails")]
    pub r#emails: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "httpBody")]
    pub r#http_body: Option<String>,
    #[builder(into)]
    #[serde(rename = "httpUrl")]
    pub r#http_url: Option<String>,
    #[builder(into)]
    #[serde(rename = "txtName")]
    pub r#txt_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "txtValue")]
    pub r#txt_value: Option<String>,
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificatePackValidationRecord {
    #[builder(into, default)]
    #[serde(rename = "cnameName")]
    pub r#cname_name: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "cnameTarget")]
    pub r#cname_target: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "httpBody")]
    pub r#http_body: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "httpUrl")]
    pub r#http_url: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "txtName")]
    pub r#txt_name: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "txtValue")]
    pub r#txt_value: Box<Option<String>>,
}

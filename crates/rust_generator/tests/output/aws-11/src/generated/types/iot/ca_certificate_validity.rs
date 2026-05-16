#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CaCertificateValidity {
    /// The certificate is not valid after this date.
    #[builder(into)]
    #[serde(rename = "notAfter")]
    pub r#not_after: Option<String>,
    /// The certificate is not valid before this date.
    #[builder(into)]
    #[serde(rename = "notBefore")]
    pub r#not_before: Option<String>,
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TargetServerSSlInfoCommonName {
    /// The TLS Common Name string of the certificate.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
    /// Indicates whether the cert should be matched against as a wildcard cert.
    #[builder(into)]
    #[serde(rename = "wildcardMatch")]
    pub r#wildcard_match: Option<bool>,
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KeystoresAliasesSelfSignedCertCertsInfo {
    /// (Output)
    /// List of all properties in the object.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "certInfos")]
    pub r#cert_infos: Option<Vec<super::super::types::apigee::KeystoresAliasesSelfSignedCertCertsInfoCertInfo>>,
}

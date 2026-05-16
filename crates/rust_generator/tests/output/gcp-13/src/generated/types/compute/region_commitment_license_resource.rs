#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionCommitmentLicenseResource {
    /// The number of licenses purchased.
    #[builder(into)]
    #[serde(rename = "amount")]
    pub r#amount: Option<String>,
    /// Specifies the core range of the instance for which this license applies.
    #[builder(into)]
    #[serde(rename = "coresPerLicense")]
    pub r#cores_per_license: Option<String>,
    /// Any applicable license URI.
    #[builder(into)]
    #[serde(rename = "license")]
    pub r#license: String,
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BareMetalClusterValidationCheck {
    /// (Output)
    /// Options used for the validation check.
    #[builder(into)]
    #[serde(rename = "options")]
    pub r#options: Option<String>,
    /// (Output)
    /// The scenario when the preflight checks were run..
    #[builder(into)]
    #[serde(rename = "scenario")]
    pub r#scenario: Option<String>,
    /// (Output)
    /// Specifies the detailed validation check status
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "statuses")]
    pub r#statuses: Option<Vec<super::super::types::gkeonprem::BareMetalClusterValidationCheckStatus>>,
}

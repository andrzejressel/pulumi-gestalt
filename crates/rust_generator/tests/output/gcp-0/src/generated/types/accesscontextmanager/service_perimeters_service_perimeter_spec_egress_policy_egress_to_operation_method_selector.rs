#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServicePerimetersServicePerimeterSpecEgressPolicyEgressToOperationMethodSelector {
    /// Value for `method` should be a valid method name for the corresponding
    /// `serviceName` in `ApiOperation`. If `*` used as value for method,
    /// then ALL methods and permissions are allowed.
    #[builder(into)]
    #[serde(rename = "method")]
    pub r#method: Option<String>,
    /// Value for permission should be a valid Cloud IAM permission for the
    /// corresponding `serviceName` in `ApiOperation`.
    #[builder(into)]
    #[serde(rename = "permission")]
    pub r#permission: Option<String>,
}

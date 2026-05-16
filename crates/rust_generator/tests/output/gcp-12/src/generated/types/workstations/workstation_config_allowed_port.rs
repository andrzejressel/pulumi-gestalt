#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkstationConfigAllowedPort {
    /// Starting port number for the current range of ports. Valid ports are 22, 80, and ports within the range 1024-65535.
    #[builder(into)]
    #[serde(rename = "first")]
    pub r#first: Option<i32>,
    /// Ending port number for the current range of ports. Valid ports are 22, 80, and ports within the range 1024-65535.
    #[builder(into)]
    #[serde(rename = "last")]
    pub r#last: Option<i32>,
}

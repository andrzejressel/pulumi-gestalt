#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LaunchTemplateCreditSpecification {
    /// The credit option for CPU usage.
    /// Can be `standard` or `unlimited`.
    /// T3 instances are launched as `unlimited` by default.
    /// T2 instances are launched as `standard` by default.
    #[builder(into)]
    #[serde(rename = "cpuCredits")]
    pub r#cpu_credits: Option<String>,
}

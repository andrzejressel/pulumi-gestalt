#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlexibleAppVersionDeploymentContainer {
    /// URI to the hosted container image in Google Container Registry. The URI must be fully qualified and include a tag or digest.
    /// Examples: "gcr.io/my-project/image:tag" or "gcr.io/my-project/image@digest"
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: String,
}

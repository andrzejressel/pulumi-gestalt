#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetImageOutputResourceAmi {
    /// Account identifier of the AMI.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: String,
    /// Description of the AMI.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// Identifier of the AMI.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: String,
    /// Name of the AMI.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Region of the container image.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: String,
}

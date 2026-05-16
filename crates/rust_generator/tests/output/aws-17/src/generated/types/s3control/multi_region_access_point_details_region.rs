#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MultiRegionAccessPointDetailsRegion {
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: String,
    #[builder(into)]
    #[serde(rename = "bucketAccountId")]
    pub r#bucket_account_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Option<String>,
}

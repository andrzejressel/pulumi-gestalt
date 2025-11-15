#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSharedImagePurchasePlan {
    /// The name of the Shared Image.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// (Optional) The Purchase Plan Product for this Gallery Image.
    #[builder(into)]
    #[serde(rename = "product")]
    pub r#product: String,
    /// (Optional) The Purchase Plan Publisher for this Gallery Image.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: String,
}

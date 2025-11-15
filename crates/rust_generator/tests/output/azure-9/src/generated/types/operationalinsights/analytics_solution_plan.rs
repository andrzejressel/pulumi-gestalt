#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AnalyticsSolutionPlan {
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The product name of the solution. For example `OMSGallery/Containers`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "product")]
    pub r#product: String,
    /// A promotion code to be used with the solution. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "promotionCode")]
    pub r#promotion_code: Option<String>,
    /// The publisher of the solution. For example `Microsoft`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: String,
}

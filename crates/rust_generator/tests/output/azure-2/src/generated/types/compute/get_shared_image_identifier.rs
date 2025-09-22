#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSharedImageIdentifier {
    /// The Offer Name for this Shared Image.
    #[builder(into)]
    #[serde(rename = "offer")]
    pub r#offer: String,
    /// (Optional) The Purchase Plan Publisher for this Gallery Image.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: String,
    /// The Name of the SKU for this Gallery Image.
    #[builder(into)]
    #[serde(rename = "sku")]
    pub r#sku: String,
}

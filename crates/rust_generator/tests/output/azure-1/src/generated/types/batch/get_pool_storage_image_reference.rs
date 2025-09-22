#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPoolStorageImageReference {
    /// The fully qualified ID of the certificate installed on the pool.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    #[builder(into)]
    #[serde(rename = "offer")]
    pub r#offer: String,
    /// The name of the extension handler publisher.The name of the extension handler publisher.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: String,
    #[builder(into)]
    #[serde(rename = "sku")]
    pub r#sku: String,
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAccountSasServices {
    /// Should permission be granted to `blob` services within this storage account?
    #[builder(into)]
    #[serde(rename = "blob")]
    pub r#blob: bool,
    /// Should permission be granted to `file` services within this storage account?
    #[builder(into)]
    #[serde(rename = "file")]
    pub r#file: bool,
    /// Should permission be granted to `queue` services within this storage account?
    #[builder(into)]
    #[serde(rename = "queue")]
    pub r#queue: bool,
    /// Should permission be granted to `table` services within this storage account?
    #[builder(into)]
    #[serde(rename = "table")]
    pub r#table: bool,
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FileSystemSizeInByte {
    /// The latest known metered size (in bytes) of data stored in the file system.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<i32>,
    /// The latest known metered size (in bytes) of data stored in the Infrequent Access storage class.
    #[builder(into)]
    #[serde(rename = "valueInIa")]
    pub r#value_in_ia: Option<i32>,
    /// The latest known metered size (in bytes) of data stored in the Standard storage class.
    #[builder(into)]
    #[serde(rename = "valueInStandard")]
    pub r#value_in_standard: Option<i32>,
}

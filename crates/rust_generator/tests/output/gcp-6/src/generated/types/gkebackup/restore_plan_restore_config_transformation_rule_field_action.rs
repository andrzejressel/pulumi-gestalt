#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RestorePlanRestoreConfigTransformationRuleFieldAction {
    /// A string containing a JSON Pointer value that references the
    /// location in the target document to move the value from.
    #[builder(into)]
    #[serde(rename = "fromPath")]
    pub r#from_path: Option<String>,
    /// Specifies the operation to perform.
    /// Possible values are: `REMOVE`, `MOVE`, `COPY`, `ADD`, `TEST`, `REPLACE`.
    #[builder(into)]
    #[serde(rename = "op")]
    pub r#op: String,
    /// A string containing a JSON-Pointer value that references a
    /// location within the target document where the operation is performed.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// A string that specifies the desired value in string format
    /// to use for transformation.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

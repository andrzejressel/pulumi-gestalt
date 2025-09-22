#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableTableConstraintsPrimaryKey {
    /// The columns that are composed of the primary key constraint.
    #[builder(into)]
    #[serde(rename = "columns")]
    pub r#columns: Vec<String>,
}

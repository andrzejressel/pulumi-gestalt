#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountRestoreGremlinDatabase {
    /// A list of the Graph names for the restore request. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "graphNames")]
    pub r#graph_names: Option<Vec<String>>,
    /// The Gremlin Database name for the restore request. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}

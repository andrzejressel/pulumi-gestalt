#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MigrationJobDumpFlags {
    /// A list of dump flags
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dumpFlags")]
    pub r#dump_flags: Option<Vec<super::super::types::databasemigrationservice::MigrationJobDumpFlagsDumpFlag>>,
}

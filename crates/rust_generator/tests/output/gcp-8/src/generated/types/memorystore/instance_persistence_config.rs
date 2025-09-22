#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstancePersistenceConfig {
    /// Configuration for AOF based persistence.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "aofConfig")]
    pub r#aof_config: Option<Box<super::super::types::memorystore::InstancePersistenceConfigAofConfig>>,
    /// Optional. Current persistence mode.
    /// Possible values:
    /// DISABLED
    /// RDB
    /// AOF
    /// Possible values are: `DISABLED`, `RDB`, `AOF`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    /// Configuration for RDB based persistence.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rdbConfig")]
    pub r#rdb_config: Option<Box<super::super::types::memorystore::InstancePersistenceConfigRdbConfig>>,
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatabaseInstanceSettingsAdvancedMachineFeatures {
    /// The number of threads per core. The value of this flag can be 1 or 2. To disable SMT, set this flag to 1. Only available in Cloud SQL for SQL Server instances. See [smt](https://cloud.google.com/sql/docs/sqlserver/create-instance#smt-create-instance) for more details.
    #[builder(into)]
    #[serde(rename = "threadsPerCore")]
    pub r#threads_per_core: Option<i32>,
}

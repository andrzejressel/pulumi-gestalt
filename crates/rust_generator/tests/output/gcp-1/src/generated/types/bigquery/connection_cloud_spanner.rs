#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionCloudSpanner {
    /// Cloud Spanner database in the form `project/instance/database'.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: String,
    /// Cloud Spanner database role for fine-grained access control. The Cloud Spanner admin should have provisioned the database role with appropriate permissions, such as `SELECT` and `INSERT`. Other users should only use roles provided by their Cloud Spanner admins. The database role name must start with a letter, and can only contain letters, numbers, and underscores. For more details, see https://cloud.google.com/spanner/docs/fgac-about.
    #[builder(into)]
    #[serde(rename = "databaseRole")]
    pub r#database_role: Option<String>,
    /// Allows setting max parallelism per query when executing on Spanner independent compute resources. If unspecified, default values of parallelism are chosen that are dependent on the Cloud Spanner instance configuration. `useParallelism` and `useDataBoost` must be set when setting max parallelism.
    #[builder(into)]
    #[serde(rename = "maxParallelism")]
    pub r#max_parallelism: Option<i32>,
    /// If set, the request will be executed via Spanner independent compute resources. `use_parallelism` must be set when using data boost.
    #[builder(into)]
    #[serde(rename = "useDataBoost")]
    pub r#use_data_boost: Option<bool>,
    /// If parallelism should be used when reading from Cloud Spanner.
    #[builder(into)]
    #[serde(rename = "useParallelism")]
    pub r#use_parallelism: Option<bool>,
    /// (Optional, Deprecated)
    /// If the serverless analytics service should be used to read data from Cloud Spanner. `useParallelism` must be set when using serverless analytics.
    /// 
    /// > **Warning:** `useServerlessAnalytics` is deprecated and will be removed in a future major release. Use `useDataBoost` instead.
    #[builder(into)]
    #[serde(rename = "useServerlessAnalytics")]
    pub r#use_serverless_analytics: Option<bool>,
}

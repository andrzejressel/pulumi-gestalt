#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EnterpriseDatabaseModule {
    /// Configuration options for the module (e.g. `ERROR_RATE 0.00 INITIAL_SIZE 400`). Changing this forces a new resource to be created. Defaults to `""`.
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Option<String>,
    /// The name which should be used for this module. Possible values are `RedisBloom`, `RedisTimeSeries`, `RediSearch` and `RedisJSON`. Changing this forces a new Redis Enterprise Database to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}

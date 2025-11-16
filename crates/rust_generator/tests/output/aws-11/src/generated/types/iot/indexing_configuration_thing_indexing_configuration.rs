#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IndexingConfigurationThingIndexingConfiguration {
    /// Contains custom field names and their data type. See below.
    #[builder(into)]
    #[serde(rename = "customFields")]
    pub r#custom_fields: Option<Vec<super::super::types::iot::IndexingConfigurationThingIndexingConfigurationCustomField>>,
    /// Device Defender indexing mode. Valid values: `VIOLATIONS`, `OFF`. Default: `OFF`.
    #[builder(into)]
    #[serde(rename = "deviceDefenderIndexingMode")]
    pub r#device_defender_indexing_mode: Option<String>,
    /// Required if `named_shadow_indexing_mode` is `ON`. Enables to add named shadows filtered by `filter` to fleet indexing configuration.
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: Option<Box<super::super::types::iot::IndexingConfigurationThingIndexingConfigurationFilter>>,
    /// Contains fields that are indexed and whose types are already known by the Fleet Indexing service. See below.
    #[builder(into)]
    #[serde(rename = "managedFields")]
    pub r#managed_fields: Option<Vec<super::super::types::iot::IndexingConfigurationThingIndexingConfigurationManagedField>>,
    /// [Named shadow](https://docs.aws.amazon.com/iot/latest/developerguide/iot-device-shadows.html) indexing mode. Valid values: `ON`, `OFF`. Default: `OFF`.
    #[builder(into)]
    #[serde(rename = "namedShadowIndexingMode")]
    pub r#named_shadow_indexing_mode: Option<String>,
    /// Thing connectivity indexing mode. Valid values: `STATUS`, `OFF`. Default: `OFF`.
    #[builder(into)]
    #[serde(rename = "thingConnectivityIndexingMode")]
    pub r#thing_connectivity_indexing_mode: Option<String>,
    /// Thing indexing mode. Valid values: `REGISTRY`, `REGISTRY_AND_SHADOW`, `OFF`.
    #[builder(into)]
    #[serde(rename = "thingIndexingMode")]
    pub r#thing_indexing_mode: String,
}

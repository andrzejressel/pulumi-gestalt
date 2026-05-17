#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDomainClusterConfig {
    /// Configuration block containing cold storage configuration.
    #[builder(into)]
    #[serde(rename = "coldStorageOptions")]
    pub r#cold_storage_options: Vec<super::super::types::elasticsearch::GetDomainClusterConfigColdStorageOption>,
    /// Number of dedicated master nodes in the cluster.
    #[builder(into)]
    #[serde(rename = "dedicatedMasterCount")]
    pub r#dedicated_master_count: i32,
    /// Indicates whether dedicated master nodes are enabled for the cluster.
    #[builder(into)]
    #[serde(rename = "dedicatedMasterEnabled")]
    pub r#dedicated_master_enabled: bool,
    /// Instance type of the dedicated master nodes in the cluster.
    #[builder(into)]
    #[serde(rename = "dedicatedMasterType")]
    pub r#dedicated_master_type: String,
    /// Number of instances in the cluster.
    #[builder(into)]
    #[serde(rename = "instanceCount")]
    pub r#instance_count: i32,
    /// Instance type of data nodes in the cluster.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: String,
    /// The number of warm nodes in the cluster.
    #[builder(into)]
    #[serde(rename = "warmCount")]
    pub r#warm_count: i32,
    /// Warm storage is enabled.
    #[builder(into)]
    #[serde(rename = "warmEnabled")]
    pub r#warm_enabled: bool,
    /// The instance type for the Elasticsearch cluster's warm nodes.
    #[builder(into)]
    #[serde(rename = "warmType")]
    pub r#warm_type: String,
    /// Configuration block containing zone awareness settings.
    #[builder(into)]
    #[serde(rename = "zoneAwarenessConfigs")]
    pub r#zone_awareness_configs: Vec<super::super::types::elasticsearch::GetDomainClusterConfigZoneAwarenessConfig>,
    /// Indicates whether zone awareness is enabled.
    #[builder(into)]
    #[serde(rename = "zoneAwarenessEnabled")]
    pub r#zone_awareness_enabled: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDomainClusterConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "cold_storage_options",
                    &self.r#cold_storage_options,
                ),
                to_pulumi_object_field(
                    "dedicated_master_count",
                    &self.r#dedicated_master_count,
                ),
                to_pulumi_object_field(
                    "dedicated_master_enabled",
                    &self.r#dedicated_master_enabled,
                ),
                to_pulumi_object_field(
                    "dedicated_master_type",
                    &self.r#dedicated_master_type,
                ),
                to_pulumi_object_field(
                    "instance_count",
                    &self.r#instance_count,
                ),
                to_pulumi_object_field(
                    "instance_type",
                    &self.r#instance_type,
                ),
                to_pulumi_object_field(
                    "warm_count",
                    &self.r#warm_count,
                ),
                to_pulumi_object_field(
                    "warm_enabled",
                    &self.r#warm_enabled,
                ),
                to_pulumi_object_field(
                    "warm_type",
                    &self.r#warm_type,
                ),
                to_pulumi_object_field(
                    "zone_awareness_configs",
                    &self.r#zone_awareness_configs,
                ),
                to_pulumi_object_field(
                    "zone_awareness_enabled",
                    &self.r#zone_awareness_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDomainClusterConfig {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#cold_storage_options: {
                        let field_value = match fields_map.get("cold_storage_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'cold_storage_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dedicated_master_count: {
                        let field_value = match fields_map.get("dedicated_master_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'dedicated_master_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dedicated_master_enabled: {
                        let field_value = match fields_map.get("dedicated_master_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'dedicated_master_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dedicated_master_type: {
                        let field_value = match fields_map.get("dedicated_master_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'dedicated_master_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_count: {
                        let field_value = match fields_map.get("instance_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_type: {
                        let field_value = match fields_map.get("instance_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#warm_count: {
                        let field_value = match fields_map.get("warm_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'warm_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#warm_enabled: {
                        let field_value = match fields_map.get("warm_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'warm_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#warm_type: {
                        let field_value = match fields_map.get("warm_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'warm_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zone_awareness_configs: {
                        let field_value = match fields_map.get("zone_awareness_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'zone_awareness_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zone_awareness_enabled: {
                        let field_value = match fields_map.get("zone_awareness_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'zone_awareness_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

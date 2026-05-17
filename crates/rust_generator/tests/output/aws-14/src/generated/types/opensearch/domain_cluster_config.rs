#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainClusterConfig {
    /// Configuration block containing cold storage configuration. Detailed below.
    #[builder(into)]
    #[serde(rename = "coldStorageOptions")]
    pub r#cold_storage_options: Option<Box<super::super::types::opensearch::DomainClusterConfigColdStorageOptions>>,
    /// Number of dedicated main nodes in the cluster.
    #[builder(into)]
    #[serde(rename = "dedicatedMasterCount")]
    pub r#dedicated_master_count: Option<i32>,
    /// Whether dedicated main nodes are enabled for the cluster.
    #[builder(into)]
    #[serde(rename = "dedicatedMasterEnabled")]
    pub r#dedicated_master_enabled: Option<bool>,
    /// Instance type of the dedicated main nodes in the cluster.
    #[builder(into)]
    #[serde(rename = "dedicatedMasterType")]
    pub r#dedicated_master_type: Option<String>,
    /// Number of instances in the cluster.
    #[builder(into)]
    #[serde(rename = "instanceCount")]
    pub r#instance_count: Option<i32>,
    /// Instance type of data nodes in the cluster.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Option<String>,
    /// Whether a multi-AZ domain is turned on with a standby AZ. For more information, see [Configuring a multi-AZ domain in Amazon OpenSearch Service](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/managedomains-multiaz.html).
    #[builder(into)]
    #[serde(rename = "multiAzWithStandbyEnabled")]
    pub r#multi_az_with_standby_enabled: Option<bool>,
    /// Number of warm nodes in the cluster. Valid values are between `2` and `150`. `warm_count` can be only and must be set when `warm_enabled` is set to `true`.
    #[builder(into)]
    #[serde(rename = "warmCount")]
    pub r#warm_count: Option<i32>,
    /// Whether to enable warm storage.
    #[builder(into)]
    #[serde(rename = "warmEnabled")]
    pub r#warm_enabled: Option<bool>,
    /// Instance type for the OpenSearch cluster's warm nodes. Valid values are `ultrawarm1.medium.search`, `ultrawarm1.large.search` and `ultrawarm1.xlarge.search`. `warm_type` can be only and must be set when `warm_enabled` is set to `true`.
    #[builder(into)]
    #[serde(rename = "warmType")]
    pub r#warm_type: Option<String>,
    /// Configuration block containing zone awareness settings. Detailed below.
    #[builder(into)]
    #[serde(rename = "zoneAwarenessConfig")]
    pub r#zone_awareness_config: Option<Box<super::super::types::opensearch::DomainClusterConfigZoneAwarenessConfig>>,
    /// Whether zone awareness is enabled, set to `true` for multi-az deployment. To enable awareness with three Availability Zones, the `availability_zone_count` within the `zone_awareness_config` must be set to `3`.
    #[builder(into)]
    #[serde(rename = "zoneAwarenessEnabled")]
    pub r#zone_awareness_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainClusterConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "cold_storage_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cold_storage_options,
                )
                .await,
            );
            map.insert(
                "dedicated_master_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dedicated_master_count,
                )
                .await,
            );
            map.insert(
                "dedicated_master_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dedicated_master_enabled,
                )
                .await,
            );
            map.insert(
                "dedicated_master_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dedicated_master_type,
                )
                .await,
            );
            map.insert(
                "instance_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_count,
                )
                .await,
            );
            map.insert(
                "instance_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_type,
                )
                .await,
            );
            map.insert(
                "multi_az_with_standby_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#multi_az_with_standby_enabled,
                )
                .await,
            );
            map.insert(
                "warm_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#warm_count,
                )
                .await,
            );
            map.insert(
                "warm_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#warm_enabled,
                )
                .await,
            );
            map.insert(
                "warm_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#warm_type,
                )
                .await,
            );
            map.insert(
                "zone_awareness_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#zone_awareness_config,
                )
                .await,
            );
            map.insert(
                "zone_awareness_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#zone_awareness_enabled,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainClusterConfig {
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
                    r#multi_az_with_standby_enabled: {
                        let field_value = match fields_map.get("multi_az_with_standby_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'multi_az_with_standby_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#zone_awareness_config: {
                        let field_value = match fields_map.get("zone_awareness_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'zone_awareness_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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

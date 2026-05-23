#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDomainClusterConfig {
    /// Configuration block containing cold storage configuration.
    #[builder(into)]
    pub r#cold_storage_options: Vec<super::super::types::opensearch::GetDomainClusterConfigColdStorageOption>,
    /// Number of dedicated master nodes in the cluster.
    #[builder(into)]
    pub r#dedicated_master_count: i32,
    /// Indicates whether dedicated master nodes are enabled for the cluster.
    #[builder(into)]
    pub r#dedicated_master_enabled: bool,
    /// Instance type of the dedicated master nodes in the cluster.
    #[builder(into)]
    pub r#dedicated_master_type: String,
    /// Number of instances in the cluster.
    #[builder(into)]
    pub r#instance_count: i32,
    /// Instance type of data nodes in the cluster.
    #[builder(into)]
    pub r#instance_type: String,
    /// Whether a multi-AZ domain is turned on with a standby AZ.
    #[builder(into)]
    pub r#multi_az_with_standby_enabled: bool,
    /// Number of warm nodes in the cluster.
    #[builder(into)]
    pub r#warm_count: i32,
    /// Warm storage is enabled.
    #[builder(into)]
    pub r#warm_enabled: Option<bool>,
    /// Instance type for the OpenSearch cluster's warm nodes.
    #[builder(into)]
    pub r#warm_type: String,
    /// Configuration block containing zone awareness settings.
    #[builder(into)]
    pub r#zone_awareness_configs: Vec<super::super::types::opensearch::GetDomainClusterConfigZoneAwarenessConfig>,
    /// Indicates whether zone awareness is enabled.
    #[builder(into)]
    pub r#zone_awareness_enabled: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDomainClusterConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "coldStorageOptions",
                    &self.r#cold_storage_options,
                ),
                to_pulumi_object_field(
                    "dedicatedMasterCount",
                    &self.r#dedicated_master_count,
                ),
                to_pulumi_object_field(
                    "dedicatedMasterEnabled",
                    &self.r#dedicated_master_enabled,
                ),
                to_pulumi_object_field(
                    "dedicatedMasterType",
                    &self.r#dedicated_master_type,
                ),
                to_pulumi_object_field(
                    "instanceCount",
                    &self.r#instance_count,
                ),
                to_pulumi_object_field(
                    "instanceType",
                    &self.r#instance_type,
                ),
                to_pulumi_object_field(
                    "multiAzWithStandbyEnabled",
                    &self.r#multi_az_with_standby_enabled,
                ),
                to_pulumi_object_field(
                    "warmCount",
                    &self.r#warm_count,
                ),
                to_pulumi_object_field(
                    "warmEnabled",
                    &self.r#warm_enabled,
                ),
                to_pulumi_object_field(
                    "warmType",
                    &self.r#warm_type,
                ),
                to_pulumi_object_field(
                    "zoneAwarenessConfigs",
                    &self.r#zone_awareness_configs,
                ),
                to_pulumi_object_field(
                    "zoneAwarenessEnabled",
                    &self.r#zone_awareness_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("coldStorageOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'coldStorageOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dedicated_master_count: {
                        let field_value = match fields_map.get("dedicatedMasterCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'dedicatedMasterCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dedicated_master_enabled: {
                        let field_value = match fields_map.get("dedicatedMasterEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'dedicatedMasterEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dedicated_master_type: {
                        let field_value = match fields_map.get("dedicatedMasterType") {
                            Some(value) => value,
                            None => bail!("Missing field 'dedicatedMasterType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_count: {
                        let field_value = match fields_map.get("instanceCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_type: {
                        let field_value = match fields_map.get("instanceType") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#multi_az_with_standby_enabled: {
                        let field_value = match fields_map.get("multiAzWithStandbyEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'multiAzWithStandbyEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#warm_count: {
                        let field_value = match fields_map.get("warmCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'warmCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#warm_enabled: {
                        let field_value = match fields_map.get("warmEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'warmEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#warm_type: {
                        let field_value = match fields_map.get("warmType") {
                            Some(value) => value,
                            None => bail!("Missing field 'warmType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zone_awareness_configs: {
                        let field_value = match fields_map.get("zoneAwarenessConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'zoneAwarenessConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zone_awareness_enabled: {
                        let field_value = match fields_map.get("zoneAwarenessEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'zoneAwarenessEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

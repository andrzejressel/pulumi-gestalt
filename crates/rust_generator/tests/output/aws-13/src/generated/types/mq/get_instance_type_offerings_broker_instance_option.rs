#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetInstanceTypeOfferingsBrokerInstanceOption {
    /// List of available AZs. See Availability Zones. below
    #[builder(into)]
    #[serde(rename = "availabilityZones")]
    pub r#availability_zones: Vec<super::super::types::mq::GetInstanceTypeOfferingsBrokerInstanceOptionAvailabilityZone>,
    /// Filter response by engine type.
    #[builder(into)]
    #[serde(rename = "engineType")]
    pub r#engine_type: String,
    /// Filter response by host instance type.
    #[builder(into)]
    #[serde(rename = "hostInstanceType")]
    pub r#host_instance_type: String,
    /// Filter response by storage type.
    #[builder(into)]
    #[serde(rename = "storageType")]
    pub r#storage_type: String,
    /// The list of supported deployment modes.
    #[builder(into)]
    #[serde(rename = "supportedDeploymentModes")]
    pub r#supported_deployment_modes: Vec<String>,
    /// The list of supported engine versions.
    #[builder(into)]
    #[serde(rename = "supportedEngineVersions")]
    pub r#supported_engine_versions: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetInstanceTypeOfferingsBrokerInstanceOption {
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
                    "availability_zones",
                    &self.r#availability_zones,
                ),
                to_pulumi_object_field(
                    "engine_type",
                    &self.r#engine_type,
                ),
                to_pulumi_object_field(
                    "host_instance_type",
                    &self.r#host_instance_type,
                ),
                to_pulumi_object_field(
                    "storage_type",
                    &self.r#storage_type,
                ),
                to_pulumi_object_field(
                    "supported_deployment_modes",
                    &self.r#supported_deployment_modes,
                ),
                to_pulumi_object_field(
                    "supported_engine_versions",
                    &self.r#supported_engine_versions,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetInstanceTypeOfferingsBrokerInstanceOption {
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
                    r#availability_zones: {
                        let field_value = match fields_map.get("availability_zones") {
                            Some(value) => value,
                            None => bail!("Missing field 'availability_zones' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#engine_type: {
                        let field_value = match fields_map.get("engine_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'engine_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_instance_type: {
                        let field_value = match fields_map.get("host_instance_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_instance_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_type: {
                        let field_value = match fields_map.get("storage_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#supported_deployment_modes: {
                        let field_value = match fields_map.get("supported_deployment_modes") {
                            Some(value) => value,
                            None => bail!("Missing field 'supported_deployment_modes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#supported_engine_versions: {
                        let field_value = match fields_map.get("supported_engine_versions") {
                            Some(value) => value,
                            None => bail!("Missing field 'supported_engine_versions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

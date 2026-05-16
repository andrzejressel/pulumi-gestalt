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
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("availability_zones".to_string(), self.r#availability_zones.to_pulumi_value().await);
            map.insert("engine_type".to_string(), self.r#engine_type.to_pulumi_value().await);
            map.insert("host_instance_type".to_string(), self.r#host_instance_type.to_pulumi_value().await);
            map.insert("storage_type".to_string(), self.r#storage_type.to_pulumi_value().await);
            map.insert("supported_deployment_modes".to_string(), self.r#supported_deployment_modes.to_pulumi_value().await);
            map.insert("supported_engine_versions".to_string(), self.r#supported_engine_versions.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetInstanceTypeOfferingsBrokerInstanceOption {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#availability_zones: {
                        let field_value = match fields_map.get("availability_zones") {
                            Some(value) => value,
                            None => bail!("Missing field 'availability_zones' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::mq::GetInstanceTypeOfferingsBrokerInstanceOptionAvailabilityZone> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#engine_type: {
                        let field_value = match fields_map.get("engine_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'engine_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#host_instance_type: {
                        let field_value = match fields_map.get("host_instance_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_instance_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#storage_type: {
                        let field_value = match fields_map.get("storage_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#supported_deployment_modes: {
                        let field_value = match fields_map.get("supported_deployment_modes") {
                            Some(value) => value,
                            None => bail!("Missing field 'supported_deployment_modes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#supported_engine_versions: {
                        let field_value = match fields_map.get("supported_engine_versions") {
                            Some(value) => value,
                            None => bail!("Missing field 'supported_engine_versions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

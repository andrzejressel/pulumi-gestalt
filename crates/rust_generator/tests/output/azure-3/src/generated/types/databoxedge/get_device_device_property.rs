#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDeviceDeviceProperty {
    /// The Data Box Edge/Gateway device local capacity in MB.
    #[builder(into)]
    #[serde(rename = "capacity")]
    pub r#capacity: i32,
    /// Type of compute roles configured.
    #[builder(into)]
    #[serde(rename = "configuredRoleTypes")]
    pub r#configured_role_types: Vec<String>,
    /// The Data Box Edge/Gateway device culture.
    #[builder(into)]
    #[serde(rename = "culture")]
    pub r#culture: String,
    /// The device software version number of the device (e.g. 1.2.18105.6).
    #[builder(into)]
    #[serde(rename = "hcsVersion")]
    pub r#hcs_version: String,
    /// The Data Box Edge/Gateway device model.
    #[builder(into)]
    #[serde(rename = "model")]
    pub r#model: String,
    /// The number of nodes in the cluster.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: i32,
    /// The Serial Number of Data Box Edge/Gateway device.
    #[builder(into)]
    #[serde(rename = "serialNumber")]
    pub r#serial_number: String,
    /// The Data Box Edge/Gateway device software version.
    #[builder(into)]
    #[serde(rename = "softwareVersion")]
    pub r#software_version: String,
    /// The status of the Data Box Edge/Gateway device.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
    /// The Data Box Edge/Gateway device timezone.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: String,
    /// The type of the Data Box Edge/Gateway device.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDeviceDeviceProperty {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "capacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#capacity,
                )
                .await,
            );
            map.insert(
                "configured_role_types".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#configured_role_types,
                )
                .await,
            );
            map.insert(
                "culture".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#culture,
                )
                .await,
            );
            map.insert(
                "hcs_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hcs_version,
                )
                .await,
            );
            map.insert(
                "model".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#model,
                )
                .await,
            );
            map.insert(
                "node_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_count,
                )
                .await,
            );
            map.insert(
                "serial_number".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#serial_number,
                )
                .await,
            );
            map.insert(
                "software_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#software_version,
                )
                .await,
            );
            map.insert(
                "status".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#status,
                )
                .await,
            );
            map.insert(
                "time_zone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#time_zone,
                )
                .await,
            );
            map.insert(
                "type_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDeviceDeviceProperty {
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
                    r#capacity: {
                        let field_value = match fields_map.get("capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#configured_role_types: {
                        let field_value = match fields_map.get("configured_role_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'configured_role_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#culture: {
                        let field_value = match fields_map.get("culture") {
                            Some(value) => value,
                            None => bail!("Missing field 'culture' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hcs_version: {
                        let field_value = match fields_map.get("hcs_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'hcs_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model: {
                        let field_value = match fields_map.get("model") {
                            Some(value) => value,
                            None => bail!("Missing field 'model' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_count: {
                        let field_value = match fields_map.get("node_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#serial_number: {
                        let field_value = match fields_map.get("serial_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'serial_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#software_version: {
                        let field_value = match fields_map.get("software_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'software_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_zone: {
                        let field_value = match fields_map.get("time_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IotHubDpsLinkedHub {
    /// The weight applied to the IoT Hub. Defaults to `1`.
    #[builder(into)]
    #[serde(rename = "allocationWeight")]
    pub r#allocation_weight: Option<i32>,
    /// Determines whether to apply allocation policies to the IoT Hub. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "applyAllocationPolicy")]
    pub r#apply_allocation_policy: Option<bool>,
    /// The connection string to connect to the IoT Hub.
    #[builder(into)]
    #[serde(rename = "connectionString")]
    pub r#connection_string: String,
    /// The IoT Hub hostname.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Option<String>,
    /// The location of the IoT hub.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IotHubDpsLinkedHub {
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
                "allocation_weight".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allocation_weight,
                )
                .await,
            );
            map.insert(
                "apply_allocation_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#apply_allocation_policy,
                )
                .await,
            );
            map.insert(
                "connection_string".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connection_string,
                )
                .await,
            );
            map.insert(
                "hostname".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hostname,
                )
                .await,
            );
            map.insert(
                "location".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#location,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IotHubDpsLinkedHub {
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
                    r#allocation_weight: {
                        let field_value = match fields_map.get("allocation_weight") {
                            Some(value) => value,
                            None => bail!("Missing field 'allocation_weight' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#apply_allocation_policy: {
                        let field_value = match fields_map.get("apply_allocation_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'apply_allocation_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connection_string: {
                        let field_value = match fields_map.get("connection_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hostname: {
                        let field_value = match fields_map.get("hostname") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostname' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#location: {
                        let field_value = match fields_map.get("location") {
                            Some(value) => value,
                            None => bail!("Missing field 'location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

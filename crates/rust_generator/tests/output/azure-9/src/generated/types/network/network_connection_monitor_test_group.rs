#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkConnectionMonitorTestGroup {
    /// A list of destination endpoint names.
    #[builder(into)]
    #[serde(rename = "destinationEndpoints")]
    pub r#destination_endpoints: Vec<String>,
    /// Should the test group be enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The name of the test group for the Network Connection Monitor.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A list of source endpoint names.
    #[builder(into)]
    #[serde(rename = "sourceEndpoints")]
    pub r#source_endpoints: Vec<String>,
    /// A list of test configuration names.
    #[builder(into)]
    #[serde(rename = "testConfigurationNames")]
    pub r#test_configuration_names: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkConnectionMonitorTestGroup {
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
                "destination_endpoints".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destination_endpoints,
                )
                .await,
            );
            map.insert(
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "source_endpoints".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_endpoints,
                )
                .await,
            );
            map.insert(
                "test_configuration_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#test_configuration_names,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkConnectionMonitorTestGroup {
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
                    r#destination_endpoints: {
                        let field_value = match fields_map.get("destination_endpoints") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_endpoints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_endpoints: {
                        let field_value = match fields_map.get("source_endpoints") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_endpoints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#test_configuration_names: {
                        let field_value = match fields_map.get("test_configuration_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'test_configuration_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

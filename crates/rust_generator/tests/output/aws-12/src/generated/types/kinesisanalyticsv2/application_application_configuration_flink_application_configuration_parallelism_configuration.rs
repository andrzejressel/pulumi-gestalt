#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationApplicationConfigurationFlinkApplicationConfigurationParallelismConfiguration {
    /// Describes whether the Kinesis Data Analytics service can increase the parallelism of the application in response to increased throughput.
    #[builder(into)]
    #[serde(rename = "autoScalingEnabled")]
    pub r#auto_scaling_enabled: Option<bool>,
    /// Describes whether the application uses the default parallelism for the Kinesis Data Analytics service. Valid values: `CUSTOM`, `DEFAULT`. Set this attribute to `CUSTOM` in order for any specified `auto_scaling_enabled`, `parallelism`, or `parallelism_per_kpu` attribute values to be effective.
    #[builder(into)]
    #[serde(rename = "configurationType")]
    pub r#configuration_type: String,
    /// Describes the initial number of parallel tasks that a Flink-based Kinesis Data Analytics application can perform.
    #[builder(into)]
    #[serde(rename = "parallelism")]
    pub r#parallelism: Option<i32>,
    /// Describes the number of parallel tasks that a Flink-based Kinesis Data Analytics application can perform per Kinesis Processing Unit (KPU) used by the application.
    #[builder(into)]
    #[serde(rename = "parallelismPerKpu")]
    pub r#parallelism_per_kpu: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationApplicationConfigurationFlinkApplicationConfigurationParallelismConfiguration {
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
                "auto_scaling_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auto_scaling_enabled,
                )
                .await,
            );
            map.insert(
                "configuration_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#configuration_type,
                )
                .await,
            );
            map.insert(
                "parallelism".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#parallelism,
                )
                .await,
            );
            map.insert(
                "parallelism_per_kpu".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#parallelism_per_kpu,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApplicationApplicationConfigurationFlinkApplicationConfigurationParallelismConfiguration {
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
                    r#auto_scaling_enabled: {
                        let field_value = match fields_map.get("auto_scaling_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_scaling_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#configuration_type: {
                        let field_value = match fields_map.get("configuration_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'configuration_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parallelism: {
                        let field_value = match fields_map.get("parallelism") {
                            Some(value) => value,
                            None => bail!("Missing field 'parallelism' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parallelism_per_kpu: {
                        let field_value = match fields_map.get("parallelism_per_kpu") {
                            Some(value) => value,
                            None => bail!("Missing field 'parallelism_per_kpu' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

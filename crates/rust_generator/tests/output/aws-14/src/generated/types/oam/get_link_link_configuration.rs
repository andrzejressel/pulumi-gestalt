#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLinkLinkConfiguration {
    /// Configuration for filtering which log groups are to send log events from the source account to the monitoring account. See `log_group_configuration` Block for details.
    #[builder(into)]
    #[serde(rename = "logGroupConfigurations")]
    pub r#log_group_configurations: Vec<super::super::types::oam::GetLinkLinkConfigurationLogGroupConfiguration>,
    /// Configuration for filtering which metric namespaces are to be shared from the source account to the monitoring account. See `metric_configuration` Block for details.
    #[builder(into)]
    #[serde(rename = "metricConfigurations")]
    pub r#metric_configurations: Vec<super::super::types::oam::GetLinkLinkConfigurationMetricConfiguration>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLinkLinkConfiguration {
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
                "log_group_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#log_group_configurations,
                )
                .await,
            );
            map.insert(
                "metric_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metric_configurations,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLinkLinkConfiguration {
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
                    r#log_group_configurations: {
                        let field_value = match fields_map.get("log_group_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_group_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric_configurations: {
                        let field_value = match fields_map.get("metric_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

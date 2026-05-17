#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataCollectionRuleDataFlow {
    /// The built-in transform to transform stream data.
    #[builder(into)]
    #[serde(rename = "builtInTransform")]
    pub r#built_in_transform: Option<String>,
    /// Specifies a list of destination names. A `azure_monitor_metrics` data source only allows for stream of kind `Microsoft-InsightsMetrics`.
    #[builder(into)]
    #[serde(rename = "destinations")]
    pub r#destinations: Vec<String>,
    /// The output stream of the transform. Only required if the data flow changes data to a different stream.
    #[builder(into)]
    #[serde(rename = "outputStream")]
    pub r#output_stream: Option<String>,
    /// Specifies a list of streams. Possible values include but not limited to `Microsoft-Event`, `Microsoft-InsightsMetrics`, `Microsoft-Perf`, `Microsoft-Syslog`, `Microsoft-WindowsEvent`, and `Microsoft-PrometheusMetrics`.
    #[builder(into)]
    #[serde(rename = "streams")]
    pub r#streams: Vec<String>,
    /// The KQL query to transform stream data.
    #[builder(into)]
    #[serde(rename = "transformKql")]
    pub r#transform_kql: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataCollectionRuleDataFlow {
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
                "built_in_transform".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#built_in_transform,
                )
                .await,
            );
            map.insert(
                "destinations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destinations,
                )
                .await,
            );
            map.insert(
                "output_stream".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#output_stream,
                )
                .await,
            );
            map.insert(
                "streams".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#streams,
                )
                .await,
            );
            map.insert(
                "transform_kql".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transform_kql,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataCollectionRuleDataFlow {
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
                    r#built_in_transform: {
                        let field_value = match fields_map.get("built_in_transform") {
                            Some(value) => value,
                            None => bail!("Missing field 'built_in_transform' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destinations: {
                        let field_value = match fields_map.get("destinations") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_stream: {
                        let field_value = match fields_map.get("output_stream") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_stream' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#streams: {
                        let field_value = match fields_map.get("streams") {
                            Some(value) => value,
                            None => bail!("Missing field 'streams' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transform_kql: {
                        let field_value = match fields_map.get("transform_kql") {
                            Some(value) => value,
                            None => bail!("Missing field 'transform_kql' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetMetastoreServiceScalingConfigAutoscalingConfigLimitConfig {
    /// The maximum scaling factor that the service will autoscale to. The default value is 6.0.
    #[builder(into)]
    #[serde(rename = "maxScalingFactor")]
    pub r#max_scaling_factor: f64,
    /// The minimum scaling factor that the service will autoscale to. The default value is 0.1.
    #[builder(into)]
    #[serde(rename = "minScalingFactor")]
    pub r#min_scaling_factor: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetMetastoreServiceScalingConfigAutoscalingConfigLimitConfig {
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
                "max_scaling_factor".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_scaling_factor,
                )
                .await,
            );
            map.insert(
                "min_scaling_factor".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_scaling_factor,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetMetastoreServiceScalingConfigAutoscalingConfigLimitConfig {
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
                    r#max_scaling_factor: {
                        let field_value = match fields_map.get("max_scaling_factor") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_scaling_factor' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_scaling_factor: {
                        let field_value = match fields_map.get("min_scaling_factor") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_scaling_factor' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

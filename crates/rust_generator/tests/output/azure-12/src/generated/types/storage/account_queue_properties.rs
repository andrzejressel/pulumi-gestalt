#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountQueueProperties {
    /// A `cors_rule` block as defined above.
    #[builder(into)]
    #[serde(rename = "corsRules")]
    pub r#cors_rules: Option<Vec<super::super::types::storage::AccountQueuePropertiesCorsRule>>,
    /// A `hour_metrics` block as defined below.
    #[builder(into)]
    #[serde(rename = "hourMetrics")]
    pub r#hour_metrics: Option<Box<super::super::types::storage::AccountQueuePropertiesHourMetrics>>,
    /// A `logging` block as defined below.
    #[builder(into)]
    #[serde(rename = "logging")]
    pub r#logging: Option<Box<super::super::types::storage::AccountQueuePropertiesLogging>>,
    /// A `minute_metrics` block as defined below.
    #[builder(into)]
    #[serde(rename = "minuteMetrics")]
    pub r#minute_metrics: Option<Box<super::super::types::storage::AccountQueuePropertiesMinuteMetrics>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccountQueueProperties {
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
                "cors_rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cors_rules,
                )
                .await,
            );
            map.insert(
                "hour_metrics".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hour_metrics,
                )
                .await,
            );
            map.insert(
                "logging".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#logging,
                )
                .await,
            );
            map.insert(
                "minute_metrics".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#minute_metrics,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccountQueueProperties {
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
                    r#cors_rules: {
                        let field_value = match fields_map.get("cors_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'cors_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hour_metrics: {
                        let field_value = match fields_map.get("hour_metrics") {
                            Some(value) => value,
                            None => bail!("Missing field 'hour_metrics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logging: {
                        let field_value = match fields_map.get("logging") {
                            Some(value) => value,
                            None => bail!("Missing field 'logging' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minute_metrics: {
                        let field_value = match fields_map.get("minute_metrics") {
                            Some(value) => value,
                            None => bail!("Missing field 'minute_metrics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

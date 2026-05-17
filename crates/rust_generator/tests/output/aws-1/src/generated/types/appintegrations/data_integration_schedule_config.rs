#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataIntegrationScheduleConfig {
    /// The start date for objects to import in the first flow run as an Unix/epoch timestamp in milliseconds or in ISO-8601 format. This needs to be a time in the past, meaning that the data created or updated before this given date will not be downloaded.
    #[builder(into)]
    #[serde(rename = "firstExecutionFrom")]
    pub r#first_execution_from: String,
    /// The name of the object to pull from the data source. Examples of objects in Salesforce include `Case`, `Account`, or `Lead`.
    #[builder(into)]
    #[serde(rename = "object")]
    pub r#object: String,
    /// How often the data should be pulled from data source. Examples include `rate(1 hour)`, `rate(3 hours)`, `rate(1 day)`.
    #[builder(into)]
    #[serde(rename = "scheduleExpression")]
    pub r#schedule_expression: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataIntegrationScheduleConfig {
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
                "first_execution_from".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#first_execution_from,
                )
                .await,
            );
            map.insert(
                "object".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#object,
                )
                .await,
            );
            map.insert(
                "schedule_expression".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schedule_expression,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataIntegrationScheduleConfig {
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
                    r#first_execution_from: {
                        let field_value = match fields_map.get("first_execution_from") {
                            Some(value) => value,
                            None => bail!("Missing field 'first_execution_from' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#object: {
                        let field_value = match fields_map.get("object") {
                            Some(value) => value,
                            None => bail!("Missing field 'object' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schedule_expression: {
                        let field_value = match fields_map.get("schedule_expression") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule_expression' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

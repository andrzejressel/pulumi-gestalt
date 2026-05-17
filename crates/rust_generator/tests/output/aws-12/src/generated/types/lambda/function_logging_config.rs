#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FunctionLoggingConfig {
    /// for JSON structured logs, choose the detail level of the logs your application sends to CloudWatch when using supported logging libraries.
    #[builder(into)]
    #[serde(rename = "applicationLogLevel")]
    pub r#application_log_level: Option<String>,
    /// select between `Text` and structured `JSON` format for your function's logs.
    #[builder(into)]
    #[serde(rename = "logFormat")]
    pub r#log_format: String,
    /// the CloudWatch log group your function sends logs to.
    #[builder(into)]
    #[serde(rename = "logGroup")]
    pub r#log_group: Option<String>,
    /// for JSON structured logs, choose the detail level of the Lambda platform event logs sent to CloudWatch, such as `ERROR`, `DEBUG`, or `INFO`.
    #[builder(into)]
    #[serde(rename = "systemLogLevel")]
    pub r#system_log_level: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FunctionLoggingConfig {
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
                "application_log_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#application_log_level,
                )
                .await,
            );
            map.insert(
                "log_format".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#log_format,
                )
                .await,
            );
            map.insert(
                "log_group".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#log_group,
                )
                .await,
            );
            map.insert(
                "system_log_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#system_log_level,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FunctionLoggingConfig {
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
                    r#application_log_level: {
                        let field_value = match fields_map.get("application_log_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_log_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_format: {
                        let field_value = match fields_map.get("log_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_group: {
                        let field_value = match fields_map.get("log_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#system_log_level: {
                        let field_value = match fields_map.get("system_log_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'system_log_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

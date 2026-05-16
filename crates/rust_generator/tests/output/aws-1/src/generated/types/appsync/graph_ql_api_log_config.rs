#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GraphQlApiLogConfig {
    /// Amazon Resource Name of the service role that AWS AppSync will assume to publish to Amazon CloudWatch logs in your account.
    #[builder(into)]
    #[serde(rename = "cloudwatchLogsRoleArn")]
    pub r#cloudwatch_logs_role_arn: String,
    /// Set to TRUE to exclude sections that contain information such as headers, context, and evaluated mapping templates, regardless of logging  level. Valid values: `true`, `false`. Default value: `false`
    #[builder(into)]
    #[serde(rename = "excludeVerboseContent")]
    pub r#exclude_verbose_content: Option<bool>,
    /// Field logging level. Valid values: `ALL`, `ERROR`, `NONE`.
    #[builder(into)]
    #[serde(rename = "fieldLogLevel")]
    pub r#field_log_level: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GraphQlApiLogConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("cloudwatch_logs_role_arn".to_string(), self.r#cloudwatch_logs_role_arn.to_pulumi_value().await);
            map.insert("exclude_verbose_content".to_string(), self.r#exclude_verbose_content.to_pulumi_value().await);
            map.insert("field_log_level".to_string(), self.r#field_log_level.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GraphQlApiLogConfig {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#cloudwatch_logs_role_arn: {
                        let field_value = match fields_map.get("cloudwatch_logs_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatch_logs_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#exclude_verbose_content: {
                        let field_value = match fields_map.get("exclude_verbose_content") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclude_verbose_content' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#field_log_level: {
                        let field_value = match fields_map.get("field_log_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'field_log_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

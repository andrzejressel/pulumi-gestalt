#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CanaryRunConfig {
    /// Whether this canary is to use active AWS X-Ray tracing when it runs. You can enable active tracing only for canaries that use version syn-nodejs-2.0 or later for their canary runtime.
    #[builder(into)]
    #[serde(rename = "activeTracing")]
    pub r#active_tracing: Option<bool>,
    /// Map of environment variables that are accessible from the canary during execution. Please see [AWS Docs](https://docs.aws.amazon.com/lambda/latest/dg/configuration-envvars.html#configuration-envvars-runtime) for variables reserved for Lambda.
    #[builder(into)]
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Option<std::collections::HashMap<String, String>>,
    /// Maximum amount of memory available to the canary while it is running, in MB. The value you specify must be a multiple of 64.
    #[builder(into)]
    #[serde(rename = "memoryInMb")]
    pub r#memory_in_mb: Option<i32>,
    /// Number of seconds the canary is allowed to run before it must stop. If you omit this field, the frequency of the canary is used, up to a maximum of 840 (14 minutes).
    #[builder(into)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CanaryRunConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("active_tracing".to_string(), self.r#active_tracing.to_pulumi_value().await);
            map.insert("environment_variables".to_string(), self.r#environment_variables.to_pulumi_value().await);
            map.insert("memory_in_mb".to_string(), self.r#memory_in_mb.to_pulumi_value().await);
            map.insert("timeout_in_seconds".to_string(), self.r#timeout_in_seconds.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CanaryRunConfig {
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
                    r#active_tracing: {
                        let field_value = match fields_map.get("active_tracing") {
                            Some(value) => value,
                            None => bail!("Missing field 'active_tracing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#environment_variables: {
                        let field_value = match fields_map.get("environment_variables") {
                            Some(value) => value,
                            None => bail!("Missing field 'environment_variables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<std::collections::HashMap<String, String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#memory_in_mb: {
                        let field_value = match fields_map.get("memory_in_mb") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory_in_mb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#timeout_in_seconds: {
                        let field_value = match fields_map.get("timeout_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

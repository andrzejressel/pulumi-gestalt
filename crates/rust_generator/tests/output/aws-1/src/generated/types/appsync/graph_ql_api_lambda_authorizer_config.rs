#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GraphQlApiLambdaAuthorizerConfig {
    /// Number of seconds a response should be cached for. The default is 5 minutes (300 seconds). The Lambda function can override this by returning a `ttlOverride` key in its response. A value of 0 disables caching of responses. Minimum value of 0. Maximum value of 3600.
    #[builder(into)]
    #[serde(rename = "authorizerResultTtlInSeconds")]
    pub r#authorizer_result_ttl_in_seconds: Option<i32>,
    /// ARN of the Lambda function to be called for authorization. Note: This Lambda function must have a resource-based policy assigned to it, to allow `lambda:InvokeFunction` from service principal `appsync.amazonaws.com`.
    #[builder(into)]
    #[serde(rename = "authorizerUri")]
    pub r#authorizer_uri: String,
    /// Regular expression for validation of tokens before the Lambda function is called.
    #[builder(into)]
    #[serde(rename = "identityValidationExpression")]
    pub r#identity_validation_expression: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GraphQlApiLambdaAuthorizerConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("authorizer_result_ttl_in_seconds".to_string(), self.r#authorizer_result_ttl_in_seconds.to_pulumi_value().await);
            map.insert("authorizer_uri".to_string(), self.r#authorizer_uri.to_pulumi_value().await);
            map.insert("identity_validation_expression".to_string(), self.r#identity_validation_expression.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GraphQlApiLambdaAuthorizerConfig {
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
                    r#authorizer_result_ttl_in_seconds: {
                        let field_value = match fields_map.get("authorizer_result_ttl_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorizer_result_ttl_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#authorizer_uri: {
                        let field_value = match fields_map.get("authorizer_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorizer_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#identity_validation_expression: {
                        let field_value = match fields_map.get("identity_validation_expression") {
                            Some(value) => value,
                            None => bail!("Missing field 'identity_validation_expression' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

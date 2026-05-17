#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GraphQlApiAdditionalAuthenticationProvider {
    /// Authentication type. Valid values: `API_KEY`, `AWS_IAM`, `AMAZON_COGNITO_USER_POOLS`, `OPENID_CONNECT`, `AWS_LAMBDA`
    #[builder(into)]
    #[serde(rename = "authenticationType")]
    pub r#authentication_type: String,
    /// Nested argument containing Lambda authorizer configuration. See `lambda_authorizer_config` Block for details.
    #[builder(into)]
    #[serde(rename = "lambdaAuthorizerConfig")]
    pub r#lambda_authorizer_config: Option<Box<super::super::types::appsync::GraphQlApiAdditionalAuthenticationProviderLambdaAuthorizerConfig>>,
    /// Nested argument containing OpenID Connect configuration. See `openid_connect_config` Block for details.
    #[builder(into)]
    #[serde(rename = "openidConnectConfig")]
    pub r#openid_connect_config: Option<Box<super::super::types::appsync::GraphQlApiAdditionalAuthenticationProviderOpenidConnectConfig>>,
    /// Amazon Cognito User Pool configuration. See `user_pool_config` Block for details.
    #[builder(into)]
    #[serde(rename = "userPoolConfig")]
    pub r#user_pool_config: Option<Box<super::super::types::appsync::GraphQlApiAdditionalAuthenticationProviderUserPoolConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GraphQlApiAdditionalAuthenticationProvider {
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
                "authentication_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authentication_type,
                )
                .await,
            );
            map.insert(
                "lambda_authorizer_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lambda_authorizer_config,
                )
                .await,
            );
            map.insert(
                "openid_connect_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#openid_connect_config,
                )
                .await,
            );
            map.insert(
                "user_pool_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_pool_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GraphQlApiAdditionalAuthenticationProvider {
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
                    r#authentication_type: {
                        let field_value = match fields_map.get("authentication_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'authentication_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lambda_authorizer_config: {
                        let field_value = match fields_map.get("lambda_authorizer_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'lambda_authorizer_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#openid_connect_config: {
                        let field_value = match fields_map.get("openid_connect_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'openid_connect_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_pool_config: {
                        let field_value = match fields_map.get("user_pool_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_pool_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

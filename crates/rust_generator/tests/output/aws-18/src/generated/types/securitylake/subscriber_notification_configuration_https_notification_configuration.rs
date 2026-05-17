#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SubscriberNotificationConfigurationHttpsNotificationConfiguration {
    /// The API key name for the notification subscription.
    #[builder(into)]
    #[serde(rename = "authorizationApiKeyName")]
    pub r#authorization_api_key_name: Option<String>,
    /// The API key value for the notification subscription.
    #[builder(into)]
    #[serde(rename = "authorizationApiKeyValue")]
    pub r#authorization_api_key_value: Option<String>,
    /// The subscription endpoint in Security Lake.
    /// If you prefer notification with an HTTPS endpoint, populate this field.
    #[builder(into)]
    #[serde(rename = "endpoint")]
    pub r#endpoint: String,
    /// The HTTP method used for the notification subscription.
    /// Valid values are `POST` and `PUT`.
    #[builder(into)]
    #[serde(rename = "httpMethod")]
    pub r#http_method: Option<String>,
    /// The Amazon Resource Name (ARN) of the EventBridge API destinations IAM role that you created.
    /// For more information about ARNs and how to use them in policies, see Managing data access and AWS Managed Policies in the Amazon Security Lake User Guide.
    #[builder(into)]
    #[serde(rename = "targetRoleArn")]
    pub r#target_role_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SubscriberNotificationConfigurationHttpsNotificationConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "authorization_api_key_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authorization_api_key_name,
                )
                .await,
            );
            map.insert(
                "authorization_api_key_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authorization_api_key_value,
                )
                .await,
            );
            map.insert(
                "endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#endpoint,
                )
                .await,
            );
            map.insert(
                "http_method".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_method,
                )
                .await,
            );
            map.insert(
                "target_role_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_role_arn,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SubscriberNotificationConfigurationHttpsNotificationConfiguration {
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
                    r#authorization_api_key_name: {
                        let field_value = match fields_map.get("authorization_api_key_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorization_api_key_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#authorization_api_key_value: {
                        let field_value = match fields_map.get("authorization_api_key_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorization_api_key_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#endpoint: {
                        let field_value = match fields_map.get("endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_method: {
                        let field_value = match fields_map.get("http_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_role_arn: {
                        let field_value = match fields_map.get("target_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

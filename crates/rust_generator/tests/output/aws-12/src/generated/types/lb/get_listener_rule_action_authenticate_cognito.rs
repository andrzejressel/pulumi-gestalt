#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetListenerRuleActionAuthenticateCognito {
    /// Set of additional parameters for the request.
    /// Detailed below.
    #[builder(into)]
    #[serde(rename = "authenticationRequestExtraParams")]
    pub r#authentication_request_extra_params: std::collections::HashMap<String, String>,
    /// Behavior when the client is not authenticated.
    #[builder(into)]
    #[serde(rename = "onUnauthenticatedRequest")]
    pub r#on_unauthenticated_request: String,
    /// Set of user claims requested.
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: String,
    /// Name of the cookie used to maintain session information.
    #[builder(into)]
    #[serde(rename = "sessionCookieName")]
    pub r#session_cookie_name: String,
    /// Maximum duration of the authentication session in seconds.
    #[builder(into)]
    #[serde(rename = "sessionTimeout")]
    pub r#session_timeout: i32,
    /// ARN of the Cognito user pool.
    #[builder(into)]
    #[serde(rename = "userPoolArn")]
    pub r#user_pool_arn: String,
    /// ID of the Cognito user pool client.
    #[builder(into)]
    #[serde(rename = "userPoolClientId")]
    pub r#user_pool_client_id: String,
    /// Domain prefix or fully-qualified domain name of the Cognito user pool.
    #[builder(into)]
    #[serde(rename = "userPoolDomain")]
    pub r#user_pool_domain: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetListenerRuleActionAuthenticateCognito {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("authentication_request_extra_params".to_string(), self.r#authentication_request_extra_params.to_pulumi_value().await);
            map.insert("on_unauthenticated_request".to_string(), self.r#on_unauthenticated_request.to_pulumi_value().await);
            map.insert("scope".to_string(), self.r#scope.to_pulumi_value().await);
            map.insert("session_cookie_name".to_string(), self.r#session_cookie_name.to_pulumi_value().await);
            map.insert("session_timeout".to_string(), self.r#session_timeout.to_pulumi_value().await);
            map.insert("user_pool_arn".to_string(), self.r#user_pool_arn.to_pulumi_value().await);
            map.insert("user_pool_client_id".to_string(), self.r#user_pool_client_id.to_pulumi_value().await);
            map.insert("user_pool_domain".to_string(), self.r#user_pool_domain.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetListenerRuleActionAuthenticateCognito {
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
                    r#authentication_request_extra_params: {
                        let field_value = match fields_map.get("authentication_request_extra_params") {
                            Some(value) => value,
                            None => bail!("Missing field 'authentication_request_extra_params' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <std::collections::HashMap<String, String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#on_unauthenticated_request: {
                        let field_value = match fields_map.get("on_unauthenticated_request") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_unauthenticated_request' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#scope: {
                        let field_value = match fields_map.get("scope") {
                            Some(value) => value,
                            None => bail!("Missing field 'scope' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#session_cookie_name: {
                        let field_value = match fields_map.get("session_cookie_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_cookie_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#session_timeout: {
                        let field_value = match fields_map.get("session_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#user_pool_arn: {
                        let field_value = match fields_map.get("user_pool_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_pool_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#user_pool_client_id: {
                        let field_value = match fields_map.get("user_pool_client_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_pool_client_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#user_pool_domain: {
                        let field_value = match fields_map.get("user_pool_domain") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_pool_domain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ListenerRuleActionAuthenticateCognito {
    /// The query parameters to include in the redirect request to the authorization endpoint. Max: 10.
    #[builder(into)]
    #[serde(rename = "authenticationRequestExtraParams")]
    pub r#authentication_request_extra_params: Option<std::collections::HashMap<String, String>>,
    /// The behavior if the user is not authenticated. Valid values: `deny`, `allow` and `authenticate`
    #[builder(into)]
    #[serde(rename = "onUnauthenticatedRequest")]
    pub r#on_unauthenticated_request: Option<String>,
    /// The set of user claims to be requested from the IdP.
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: Option<String>,
    /// The name of the cookie used to maintain session information.
    #[builder(into)]
    #[serde(rename = "sessionCookieName")]
    pub r#session_cookie_name: Option<String>,
    /// The maximum duration of the authentication session, in seconds.
    #[builder(into)]
    #[serde(rename = "sessionTimeout")]
    pub r#session_timeout: Option<i32>,
    /// The ARN of the Cognito user pool.
    #[builder(into)]
    #[serde(rename = "userPoolArn")]
    pub r#user_pool_arn: String,
    /// The ID of the Cognito user pool client.
    #[builder(into)]
    #[serde(rename = "userPoolClientId")]
    pub r#user_pool_client_id: String,
    /// The domain prefix or fully-qualified domain name of the Cognito user pool.
    #[builder(into)]
    #[serde(rename = "userPoolDomain")]
    pub r#user_pool_domain: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ListenerRuleActionAuthenticateCognito {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "authentication_request_extra_params",
                    &self.r#authentication_request_extra_params,
                ),
                to_pulumi_object_field(
                    "on_unauthenticated_request",
                    &self.r#on_unauthenticated_request,
                ),
                to_pulumi_object_field(
                    "scope",
                    &self.r#scope,
                ),
                to_pulumi_object_field(
                    "session_cookie_name",
                    &self.r#session_cookie_name,
                ),
                to_pulumi_object_field(
                    "session_timeout",
                    &self.r#session_timeout,
                ),
                to_pulumi_object_field(
                    "user_pool_arn",
                    &self.r#user_pool_arn,
                ),
                to_pulumi_object_field(
                    "user_pool_client_id",
                    &self.r#user_pool_client_id,
                ),
                to_pulumi_object_field(
                    "user_pool_domain",
                    &self.r#user_pool_domain,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ListenerRuleActionAuthenticateCognito {
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
                    r#authentication_request_extra_params: {
                        let field_value = match fields_map.get("authentication_request_extra_params") {
                            Some(value) => value,
                            None => bail!("Missing field 'authentication_request_extra_params' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#on_unauthenticated_request: {
                        let field_value = match fields_map.get("on_unauthenticated_request") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_unauthenticated_request' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scope: {
                        let field_value = match fields_map.get("scope") {
                            Some(value) => value,
                            None => bail!("Missing field 'scope' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_cookie_name: {
                        let field_value = match fields_map.get("session_cookie_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_cookie_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_timeout: {
                        let field_value = match fields_map.get("session_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_pool_arn: {
                        let field_value = match fields_map.get("user_pool_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_pool_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_pool_client_id: {
                        let field_value = match fields_map.get("user_pool_client_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_pool_client_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_pool_domain: {
                        let field_value = match fields_map.get("user_pool_domain") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_pool_domain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

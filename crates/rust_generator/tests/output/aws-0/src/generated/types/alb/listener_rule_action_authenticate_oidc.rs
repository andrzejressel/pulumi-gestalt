#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ListenerRuleActionAuthenticateOidc {
    /// The query parameters to include in the redirect request to the authorization endpoint. Max: 10.
    #[builder(into)]
    pub r#authentication_request_extra_params: Option<std::collections::HashMap<String, String>>,
    /// The authorization endpoint of the IdP.
    #[builder(into)]
    pub r#authorization_endpoint: String,
    /// The OAuth 2.0 client identifier.
    #[builder(into)]
    pub r#client_id: String,
    /// The OAuth 2.0 client secret.
    #[builder(into)]
    pub r#client_secret: String,
    /// The OIDC issuer identifier of the IdP.
    #[builder(into)]
    pub r#issuer: String,
    /// The behavior if the user is not authenticated. Valid values: `deny`, `allow` and `authenticate`
    #[builder(into)]
    pub r#on_unauthenticated_request: Option<String>,
    /// The set of user claims to be requested from the IdP.
    #[builder(into)]
    pub r#scope: Option<String>,
    /// The name of the cookie used to maintain session information.
    #[builder(into)]
    pub r#session_cookie_name: Option<String>,
    /// The maximum duration of the authentication session, in seconds.
    #[builder(into)]
    pub r#session_timeout: Option<i32>,
    /// The token endpoint of the IdP.
    #[builder(into)]
    pub r#token_endpoint: String,
    /// The user info endpoint of the IdP.
    #[builder(into)]
    pub r#user_info_endpoint: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ListenerRuleActionAuthenticateOidc {
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
                    "authenticationRequestExtraParams",
                    &self.r#authentication_request_extra_params,
                ),
                to_pulumi_object_field(
                    "authorizationEndpoint",
                    &self.r#authorization_endpoint,
                ),
                to_pulumi_object_field(
                    "clientId",
                    &self.r#client_id,
                ),
                to_pulumi_object_field(
                    "clientSecret",
                    &self.r#client_secret,
                ),
                to_pulumi_object_field(
                    "issuer",
                    &self.r#issuer,
                ),
                to_pulumi_object_field(
                    "onUnauthenticatedRequest",
                    &self.r#on_unauthenticated_request,
                ),
                to_pulumi_object_field(
                    "scope",
                    &self.r#scope,
                ),
                to_pulumi_object_field(
                    "sessionCookieName",
                    &self.r#session_cookie_name,
                ),
                to_pulumi_object_field(
                    "sessionTimeout",
                    &self.r#session_timeout,
                ),
                to_pulumi_object_field(
                    "tokenEndpoint",
                    &self.r#token_endpoint,
                ),
                to_pulumi_object_field(
                    "userInfoEndpoint",
                    &self.r#user_info_endpoint,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ListenerRuleActionAuthenticateOidc {
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
                        let field_value = match fields_map.get("authenticationRequestExtraParams") {
                            Some(value) => value,
                            None => bail!("Missing field 'authenticationRequestExtraParams' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#authorization_endpoint: {
                        let field_value = match fields_map.get("authorizationEndpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorizationEndpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_id: {
                        let field_value = match fields_map.get("clientId") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_secret: {
                        let field_value = match fields_map.get("clientSecret") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientSecret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#issuer: {
                        let field_value = match fields_map.get("issuer") {
                            Some(value) => value,
                            None => bail!("Missing field 'issuer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#on_unauthenticated_request: {
                        let field_value = match fields_map.get("onUnauthenticatedRequest") {
                            Some(value) => value,
                            None => bail!("Missing field 'onUnauthenticatedRequest' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("sessionCookieName") {
                            Some(value) => value,
                            None => bail!("Missing field 'sessionCookieName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_timeout: {
                        let field_value = match fields_map.get("sessionTimeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'sessionTimeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token_endpoint: {
                        let field_value = match fields_map.get("tokenEndpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'tokenEndpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_info_endpoint: {
                        let field_value = match fields_map.get("userInfoEndpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'userInfoEndpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

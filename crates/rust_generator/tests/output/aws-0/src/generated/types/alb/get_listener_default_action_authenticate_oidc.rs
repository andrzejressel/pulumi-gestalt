#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetListenerDefaultActionAuthenticateOidc {
    #[builder(into)]
    #[serde(rename = "authenticationRequestExtraParams")]
    pub r#authentication_request_extra_params: std::collections::HashMap<String, String>,
    #[builder(into)]
    #[serde(rename = "authorizationEndpoint")]
    pub r#authorization_endpoint: String,
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: String,
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: String,
    #[builder(into)]
    #[serde(rename = "onUnauthenticatedRequest")]
    pub r#on_unauthenticated_request: String,
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: String,
    #[builder(into)]
    #[serde(rename = "sessionCookieName")]
    pub r#session_cookie_name: String,
    #[builder(into)]
    #[serde(rename = "sessionTimeout")]
    pub r#session_timeout: i32,
    #[builder(into)]
    #[serde(rename = "tokenEndpoint")]
    pub r#token_endpoint: String,
    #[builder(into)]
    #[serde(rename = "userInfoEndpoint")]
    pub r#user_info_endpoint: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetListenerDefaultActionAuthenticateOidc {
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetListenerDefaultActionAuthenticateOidc {
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

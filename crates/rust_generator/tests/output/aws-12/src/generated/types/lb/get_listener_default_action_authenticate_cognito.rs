#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetListenerDefaultActionAuthenticateCognito {
    #[builder(into)]
    pub r#authentication_request_extra_params: std::collections::BTreeMap<String, String>,
    #[builder(into)]
    pub r#on_unauthenticated_request: String,
    #[builder(into)]
    pub r#scope: String,
    #[builder(into)]
    pub r#session_cookie_name: String,
    #[builder(into)]
    pub r#session_timeout: i32,
    #[builder(into)]
    pub r#user_pool_arn: String,
    #[builder(into)]
    pub r#user_pool_client_id: String,
    #[builder(into)]
    pub r#user_pool_domain: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetListenerDefaultActionAuthenticateCognito {
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
                    "userPoolArn",
                    &self.r#user_pool_arn,
                ),
                to_pulumi_object_field(
                    "userPoolClientId",
                    &self.r#user_pool_client_id,
                ),
                to_pulumi_object_field(
                    "userPoolDomain",
                    &self.r#user_pool_domain,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetListenerDefaultActionAuthenticateCognito {
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
                    r#user_pool_arn: {
                        let field_value = match fields_map.get("userPoolArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'userPoolArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_pool_client_id: {
                        let field_value = match fields_map.get("userPoolClientId") {
                            Some(value) => value,
                            None => bail!("Missing field 'userPoolClientId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_pool_domain: {
                        let field_value = match fields_map.get("userPoolDomain") {
                            Some(value) => value,
                            None => bail!("Missing field 'userPoolDomain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

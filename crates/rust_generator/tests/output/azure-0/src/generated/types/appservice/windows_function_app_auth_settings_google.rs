#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsFunctionAppAuthSettingsGoogle {
    /// The OpenID Connect Client ID for the Google web application.
    #[builder(into)]
    pub r#client_id: String,
    /// The client secret associated with the Google web application. Cannot be specified with `client_secret_setting_name`.
    #[builder(into)]
    pub r#client_secret: Option<String>,
    /// The app setting name that contains the `client_secret` value used for Google login. Cannot be specified with `client_secret`.
    #[builder(into)]
    pub r#client_secret_setting_name: Option<String>,
    /// Specifies a list of OAuth 2.0 scopes that will be requested as part of Google Sign-In authentication. If not specified, `openid`, `profile`, and `email` are used as default scopes.
    #[builder(into)]
    pub r#oauth_scopes: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WindowsFunctionAppAuthSettingsGoogle {
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
                    "clientId",
                    &self.r#client_id,
                ),
                to_pulumi_object_field(
                    "clientSecret",
                    &self.r#client_secret,
                ),
                to_pulumi_object_field(
                    "clientSecretSettingName",
                    &self.r#client_secret_setting_name,
                ),
                to_pulumi_object_field(
                    "oauthScopes",
                    &self.r#oauth_scopes,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WindowsFunctionAppAuthSettingsGoogle {
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
                    r#client_secret_setting_name: {
                        let field_value = match fields_map.get("clientSecretSettingName") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientSecretSettingName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_scopes: {
                        let field_value = match fields_map.get("oauthScopes") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauthScopes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsFunctionAppAuthSettingsFacebook {
    /// The App ID of the Facebook app used for login.
    #[builder(into)]
    #[serde(rename = "appId")]
    pub r#app_id: String,
    /// The App Secret of the Facebook app used for Facebook login. Cannot be specified with `app_secret_setting_name`.
    #[builder(into)]
    #[serde(rename = "appSecret")]
    pub r#app_secret: Option<String>,
    /// The app setting name that contains the `app_secret` value used for Facebook login. Cannot be specified with `app_secret`.
    #[builder(into)]
    #[serde(rename = "appSecretSettingName")]
    pub r#app_secret_setting_name: Option<String>,
    /// Specifies a list of OAuth 2.0 scopes to be requested as part of Facebook login authentication.
    #[builder(into)]
    #[serde(rename = "oauthScopes")]
    pub r#oauth_scopes: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WindowsFunctionAppAuthSettingsFacebook {
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
                    "appId",
                    &self.r#app_id,
                ),
                to_pulumi_object_field(
                    "appSecret",
                    &self.r#app_secret,
                ),
                to_pulumi_object_field(
                    "appSecretSettingName",
                    &self.r#app_secret_setting_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WindowsFunctionAppAuthSettingsFacebook {
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
                    r#app_id: {
                        let field_value = match fields_map.get("appId") {
                            Some(value) => value,
                            None => bail!("Missing field 'appId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#app_secret: {
                        let field_value = match fields_map.get("appSecret") {
                            Some(value) => value,
                            None => bail!("Missing field 'appSecret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#app_secret_setting_name: {
                        let field_value = match fields_map.get("appSecretSettingName") {
                            Some(value) => value,
                            None => bail!("Missing field 'appSecretSettingName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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

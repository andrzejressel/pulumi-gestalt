#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsWebAppSlotAuthSettingsV2FacebookV2 {
    /// The App ID of the Facebook app used for login.
    #[builder(into)]
    pub r#app_id: String,
    /// The app setting name that contains the `app_secret` value used for Facebook Login.
    /// 
    /// !> **NOTE:** A setting with this name must exist in `app_settings` to function correctly.
    #[builder(into)]
    pub r#app_secret_setting_name: String,
    /// The version of the Facebook API to be used while logging in.
    #[builder(into)]
    pub r#graph_api_version: Option<String>,
    /// The list of scopes that should be requested as part of Facebook Login authentication.
    #[builder(into)]
    pub r#login_scopes: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WindowsWebAppSlotAuthSettingsV2FacebookV2 {
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
                    "appSecretSettingName",
                    &self.r#app_secret_setting_name,
                ),
                to_pulumi_object_field(
                    "graphApiVersion",
                    &self.r#graph_api_version,
                ),
                to_pulumi_object_field(
                    "loginScopes",
                    &self.r#login_scopes,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WindowsWebAppSlotAuthSettingsV2FacebookV2 {
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
                    r#app_secret_setting_name: {
                        let field_value = match fields_map.get("appSecretSettingName") {
                            Some(value) => value,
                            None => bail!("Missing field 'appSecretSettingName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#graph_api_version: {
                        let field_value = match fields_map.get("graphApiVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'graphApiVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#login_scopes: {
                        let field_value = match fields_map.get("loginScopes") {
                            Some(value) => value,
                            None => bail!("Missing field 'loginScopes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

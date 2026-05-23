#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SettingsAccessSettings {
    /// Settings to configure and enable allowed domains.
    /// Structure is documented below.
    #[builder(into)]
    pub r#allowed_domains_settings: Option<Box<super::super::types::iap::SettingsAccessSettingsAllowedDomainsSettings>>,
    /// Configuration to allow cross-origin requests via IAP.
    /// Structure is documented below.
    #[builder(into)]
    pub r#cors_settings: Option<Box<super::super::types::iap::SettingsAccessSettingsCorsSettings>>,
    /// GCIP claims and endpoint configurations for 3p identity providers.
    /// Structure is documented below.
    #[builder(into)]
    pub r#gcip_settings: Option<Box<super::super::types::iap::SettingsAccessSettingsGcipSettings>>,
    /// Identity sources that IAP can use to authenticate the end user. Only one identity source
    /// can be configured. The possible values are:
    /// * `WORKFORCE_IDENTITY_FEDERATION`: Use external identities set up on Google Cloud Workforce
    /// Identity Federation.
    /// Each value may be one of: `WORKFORCE_IDENTITY_FEDERATION`.
    #[builder(into)]
    pub r#identity_sources: Option<Vec<String>>,
    /// Settings to configure IAP's OAuth behavior.
    /// Structure is documented below.
    #[builder(into)]
    pub r#oauth_settings: Option<Box<super::super::types::iap::SettingsAccessSettingsOauthSettings>>,
    /// Settings to configure reauthentication policies in IAP.
    /// Structure is documented below.
    #[builder(into)]
    pub r#reauth_settings: Option<Box<super::super::types::iap::SettingsAccessSettingsReauthSettings>>,
    /// Settings to configure the workforce identity federation, including workforce pools
    /// and OAuth 2.0 settings.
    /// Structure is documented below.
    #[builder(into)]
    pub r#workforce_identity_settings: Option<Box<super::super::types::iap::SettingsAccessSettingsWorkforceIdentitySettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SettingsAccessSettings {
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
                    "allowedDomainsSettings",
                    &self.r#allowed_domains_settings,
                ),
                to_pulumi_object_field(
                    "corsSettings",
                    &self.r#cors_settings,
                ),
                to_pulumi_object_field(
                    "gcipSettings",
                    &self.r#gcip_settings,
                ),
                to_pulumi_object_field(
                    "identitySources",
                    &self.r#identity_sources,
                ),
                to_pulumi_object_field(
                    "oauthSettings",
                    &self.r#oauth_settings,
                ),
                to_pulumi_object_field(
                    "reauthSettings",
                    &self.r#reauth_settings,
                ),
                to_pulumi_object_field(
                    "workforceIdentitySettings",
                    &self.r#workforce_identity_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SettingsAccessSettings {
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
                    r#allowed_domains_settings: {
                        let field_value = match fields_map.get("allowedDomainsSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowedDomainsSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cors_settings: {
                        let field_value = match fields_map.get("corsSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'corsSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcip_settings: {
                        let field_value = match fields_map.get("gcipSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcipSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#identity_sources: {
                        let field_value = match fields_map.get("identitySources") {
                            Some(value) => value,
                            None => bail!("Missing field 'identitySources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_settings: {
                        let field_value = match fields_map.get("oauthSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauthSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reauth_settings: {
                        let field_value = match fields_map.get("reauthSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'reauthSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workforce_identity_settings: {
                        let field_value = match fields_map.get("workforceIdentitySettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'workforceIdentitySettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SettingsAccessSettings {
    /// Settings to configure and enable allowed domains.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "allowedDomainsSettings")]
    pub r#allowed_domains_settings: Option<Box<super::super::types::iap::SettingsAccessSettingsAllowedDomainsSettings>>,
    /// Configuration to allow cross-origin requests via IAP.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "corsSettings")]
    pub r#cors_settings: Option<Box<super::super::types::iap::SettingsAccessSettingsCorsSettings>>,
    /// GCIP claims and endpoint configurations for 3p identity providers.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "gcipSettings")]
    pub r#gcip_settings: Option<Box<super::super::types::iap::SettingsAccessSettingsGcipSettings>>,
    /// Identity sources that IAP can use to authenticate the end user. Only one identity source
    /// can be configured. The possible values are:
    /// * `WORKFORCE_IDENTITY_FEDERATION`: Use external identities set up on Google Cloud Workforce
    /// Identity Federation.
    /// Each value may be one of: `WORKFORCE_IDENTITY_FEDERATION`.
    #[builder(into)]
    #[serde(rename = "identitySources")]
    pub r#identity_sources: Option<Vec<String>>,
    /// Settings to configure IAP's OAuth behavior.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oauthSettings")]
    pub r#oauth_settings: Option<Box<super::super::types::iap::SettingsAccessSettingsOauthSettings>>,
    /// Settings to configure reauthentication policies in IAP.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "reauthSettings")]
    pub r#reauth_settings: Option<Box<super::super::types::iap::SettingsAccessSettingsReauthSettings>>,
    /// Settings to configure the workforce identity federation, including workforce pools
    /// and OAuth 2.0 settings.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "workforceIdentitySettings")]
    pub r#workforce_identity_settings: Option<Box<super::super::types::iap::SettingsAccessSettingsWorkforceIdentitySettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SettingsAccessSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "allowed_domains_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_domains_settings,
                )
                .await,
            );
            map.insert(
                "cors_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cors_settings,
                )
                .await,
            );
            map.insert(
                "gcip_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gcip_settings,
                )
                .await,
            );
            map.insert(
                "identity_sources".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#identity_sources,
                )
                .await,
            );
            map.insert(
                "oauth_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#oauth_settings,
                )
                .await,
            );
            map.insert(
                "reauth_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reauth_settings,
                )
                .await,
            );
            map.insert(
                "workforce_identity_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#workforce_identity_settings,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
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
                        let field_value = match fields_map.get("allowed_domains_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_domains_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cors_settings: {
                        let field_value = match fields_map.get("cors_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'cors_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcip_settings: {
                        let field_value = match fields_map.get("gcip_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcip_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#identity_sources: {
                        let field_value = match fields_map.get("identity_sources") {
                            Some(value) => value,
                            None => bail!("Missing field 'identity_sources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_settings: {
                        let field_value = match fields_map.get("oauth_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reauth_settings: {
                        let field_value = match fields_map.get("reauth_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'reauth_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workforce_identity_settings: {
                        let field_value = match fields_map.get("workforce_identity_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'workforce_identity_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

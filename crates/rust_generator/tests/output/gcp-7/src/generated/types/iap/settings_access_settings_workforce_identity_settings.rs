#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SettingsAccessSettingsWorkforceIdentitySettings {
    /// OAuth 2.0 settings for IAP to perform OIDC flow with workforce identity
    /// federation services.
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_oauth2"></a>The `oauth2` block supports:
    #[builder(into)]
    #[serde(rename = "oauth2")]
    pub r#oauth_2: Option<Box<super::super::types::iap::SettingsAccessSettingsWorkforceIdentitySettingsOauth2>>,
    /// The workforce pool resources. Only one workforce pool is accepted.
    #[builder(into)]
    #[serde(rename = "workforcePools")]
    pub r#workforce_pools: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SettingsAccessSettingsWorkforceIdentitySettings {
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
                "oauth_2".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#oauth_2,
                )
                .await,
            );
            map.insert(
                "workforce_pools".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#workforce_pools,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SettingsAccessSettingsWorkforceIdentitySettings {
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
                    r#oauth_2: {
                        let field_value = match fields_map.get("oauth_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workforce_pools: {
                        let field_value = match fields_map.get("workforce_pools") {
                            Some(value) => value,
                            None => bail!("Missing field 'workforce_pools' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

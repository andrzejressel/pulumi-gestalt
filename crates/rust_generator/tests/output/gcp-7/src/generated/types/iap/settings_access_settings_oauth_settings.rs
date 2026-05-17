#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SettingsAccessSettingsOauthSettings {
    /// Domain hint to send as hd=? parameter in OAuth request flow.
    /// Enables redirect to primary IDP by skipping Google's login screen.
    /// (https://developers.google.com/identity/protocols/OpenIDConnect#hd-param)
    /// Note: IAP does not verify that the id token's hd claim matches this value
    /// since access behavior is managed by IAM policies.
    #[builder(into)]
    #[serde(rename = "loginHint")]
    pub r#login_hint: Option<String>,
    /// List of client ids allowed to use IAP programmatically.
    #[builder(into)]
    #[serde(rename = "programmaticClients")]
    pub r#programmatic_clients: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SettingsAccessSettingsOauthSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "login_hint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#login_hint,
                )
                .await,
            );
            map.insert(
                "programmatic_clients".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#programmatic_clients,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SettingsAccessSettingsOauthSettings {
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
                    r#login_hint: {
                        let field_value = match fields_map.get("login_hint") {
                            Some(value) => value,
                            None => bail!("Missing field 'login_hint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#programmatic_clients: {
                        let field_value = match fields_map.get("programmatic_clients") {
                            Some(value) => value,
                            None => bail!("Missing field 'programmatic_clients' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBackendServiceIap {
    /// Whether the serving infrastructure will authenticate and authorize all incoming requests.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// OAuth2 Client ID for IAP
    #[builder(into)]
    #[serde(rename = "oauth2ClientId")]
    pub r#oauth_2_client_id: String,
    /// OAuth2 Client Secret for IAP
    #[builder(into)]
    #[serde(rename = "oauth2ClientSecret")]
    pub r#oauth_2_client_secret: String,
    /// OAuth2 Client Secret SHA-256 for IAP
    #[builder(into)]
    #[serde(rename = "oauth2ClientSecretSha256")]
    pub r#oauth_2_client_secret_sha_256: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetBackendServiceIap {
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
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
                )
                .await,
            );
            map.insert(
                "oauth_2_client_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#oauth_2_client_id,
                )
                .await,
            );
            map.insert(
                "oauth_2_client_secret".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#oauth_2_client_secret,
                )
                .await,
            );
            map.insert(
                "oauth_2_client_secret_sha_256".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#oauth_2_client_secret_sha_256,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetBackendServiceIap {
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
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_2_client_id: {
                        let field_value = match fields_map.get("oauth_2_client_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth_2_client_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_2_client_secret: {
                        let field_value = match fields_map.get("oauth_2_client_secret") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth_2_client_secret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_2_client_secret_sha_256: {
                        let field_value = match fields_map.get("oauth_2_client_secret_sha_256") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth_2_client_secret_sha_256' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

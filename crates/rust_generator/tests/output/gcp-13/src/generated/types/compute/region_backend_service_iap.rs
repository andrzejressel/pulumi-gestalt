#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionBackendServiceIap {
    /// Whether the serving infrastructure will authenticate and authorize all incoming requests.
    #[builder(into)]
    pub r#enabled: bool,
    /// OAuth2 Client ID for IAP
    #[builder(into)]
    pub r#oauth_2_client_id: Option<String>,
    /// OAuth2 Client Secret for IAP
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    pub r#oauth_2_client_secret: Option<String>,
    /// (Output)
    /// OAuth2 Client Secret SHA-256 for IAP
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    pub r#oauth_2_client_secret_sha_256: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegionBackendServiceIap {
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
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "oauth2ClientId",
                    &self.r#oauth_2_client_id,
                ),
                to_pulumi_object_field(
                    "oauth2ClientSecret",
                    &self.r#oauth_2_client_secret,
                ),
                to_pulumi_object_field(
                    "oauth2ClientSecretSha256",
                    &self.r#oauth_2_client_secret_sha_256,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegionBackendServiceIap {
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
                        let field_value = match fields_map.get("oauth2ClientId") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth2ClientId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_2_client_secret: {
                        let field_value = match fields_map.get("oauth2ClientSecret") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth2ClientSecret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_2_client_secret_sha_256: {
                        let field_value = match fields_map.get("oauth2ClientSecretSha256") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth2ClientSecretSha256' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

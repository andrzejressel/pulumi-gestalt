#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelDirectLineSite {
    /// Enables/Disables this site. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Is the endpoint parameters enabled for this site?
    #[builder(into)]
    #[serde(rename = "endpointParametersEnabled")]
    pub r#endpoint_parameters_enabled: Option<bool>,
    /// Enables additional security measures for this site, see [Enhanced Directline Authentication Features](https://blog.botframework.com/2018/09/25/enhanced-direct-line-authentication-features). Disabled by default.
    #[builder(into)]
    #[serde(rename = "enhancedAuthenticationEnabled")]
    pub r#enhanced_authentication_enabled: Option<bool>,
    /// Id for the site
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Primary key for accessing this site
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// Secondary key for accessing this site
    #[builder(into)]
    #[serde(rename = "key2")]
    pub r#key_2: Option<String>,
    /// The name of the site
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Is the storage site enabled for detailed logging? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "storageEnabled")]
    pub r#storage_enabled: Option<bool>,
    /// This field is required when `is_secure_site_enabled` is enabled. Determines which origins can establish a Directline conversation for this site.
    #[builder(into)]
    #[serde(rename = "trustedOrigins")]
    pub r#trusted_origins: Option<Vec<String>>,
    /// Is the user upload enabled for this site? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "userUploadEnabled")]
    pub r#user_upload_enabled: Option<bool>,
    /// Enables v1 of the Directline protocol for this site. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "v1Allowed")]
    pub r#v_1_allowed: Option<bool>,
    /// Enables v3 of the Directline protocol for this site. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "v3Allowed")]
    pub r#v_3_allowed: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelDirectLineSite {
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
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
                )
                .await,
            );
            map.insert(
                "endpoint_parameters_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#endpoint_parameters_enabled,
                )
                .await,
            );
            map.insert(
                "enhanced_authentication_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enhanced_authentication_enabled,
                )
                .await,
            );
            map.insert(
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key,
                )
                .await,
            );
            map.insert(
                "key_2".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_2,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "storage_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_enabled,
                )
                .await,
            );
            map.insert(
                "trusted_origins".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#trusted_origins,
                )
                .await,
            );
            map.insert(
                "user_upload_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_upload_enabled,
                )
                .await,
            );
            map.insert(
                "v_1_allowed".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#v_1_allowed,
                )
                .await,
            );
            map.insert(
                "v_3_allowed".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#v_3_allowed,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelDirectLineSite {
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
                    r#endpoint_parameters_enabled: {
                        let field_value = match fields_map.get("endpoint_parameters_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpoint_parameters_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enhanced_authentication_enabled: {
                        let field_value = match fields_map.get("enhanced_authentication_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enhanced_authentication_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key: {
                        let field_value = match fields_map.get("key") {
                            Some(value) => value,
                            None => bail!("Missing field 'key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_2: {
                        let field_value = match fields_map.get("key_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_enabled: {
                        let field_value = match fields_map.get("storage_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trusted_origins: {
                        let field_value = match fields_map.get("trusted_origins") {
                            Some(value) => value,
                            None => bail!("Missing field 'trusted_origins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_upload_enabled: {
                        let field_value = match fields_map.get("user_upload_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_upload_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#v_1_allowed: {
                        let field_value = match fields_map.get("v_1_allowed") {
                            Some(value) => value,
                            None => bail!("Missing field 'v_1_allowed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#v_3_allowed: {
                        let field_value = match fields_map.get("v_3_allowed") {
                            Some(value) => value,
                            None => bail!("Missing field 'v_3_allowed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

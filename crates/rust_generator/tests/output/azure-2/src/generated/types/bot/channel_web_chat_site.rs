#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelWebChatSite {
    /// Is the endpoint parameters enabled for this site?
    #[builder(into)]
    #[serde(rename = "endpointParametersEnabled")]
    pub r#endpoint_parameters_enabled: Option<bool>,
    /// The name of the site.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Is the storage site enabled for detailed logging? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "storageEnabled")]
    pub r#storage_enabled: Option<bool>,
    /// Is the user upload enabled for this site? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "userUploadEnabled")]
    pub r#user_upload_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelWebChatSite {
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
                "endpoint_parameters_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#endpoint_parameters_enabled,
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
                "user_upload_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_upload_enabled,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelWebChatSite {
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
                    r#endpoint_parameters_enabled: {
                        let field_value = match fields_map.get("endpoint_parameters_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpoint_parameters_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#user_upload_enabled: {
                        let field_value = match fields_map.get("user_upload_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_upload_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

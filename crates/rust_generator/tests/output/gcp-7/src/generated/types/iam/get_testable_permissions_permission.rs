#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTestablePermissionsPermission {
    /// Whether the corresponding API has been enabled for the resource.
    #[builder(into)]
    #[serde(rename = "apiDisabled")]
    pub r#api_disabled: bool,
    /// The level of support for custom roles. Can be one of `"NOT_SUPPORTED"`, `"SUPPORTED"`, `"TESTING"`. Default is `"SUPPORTED"`
    #[builder(into)]
    #[serde(rename = "customSupportLevel")]
    pub r#custom_support_level: String,
    /// Name of the permission.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Release stage of the permission.
    #[builder(into)]
    #[serde(rename = "stage")]
    pub r#stage: String,
    /// Human readable title of the permission.
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTestablePermissionsPermission {
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
                "api_disabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#api_disabled,
                )
                .await,
            );
            map.insert(
                "custom_support_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_support_level,
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
                "stage".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#stage,
                )
                .await,
            );
            map.insert(
                "title".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#title,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTestablePermissionsPermission {
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
                    r#api_disabled: {
                        let field_value = match fields_map.get("api_disabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'api_disabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_support_level: {
                        let field_value = match fields_map.get("custom_support_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_support_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#stage: {
                        let field_value = match fields_map.get("stage") {
                            Some(value) => value,
                            None => bail!("Missing field 'stage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#title: {
                        let field_value = match fields_map.get("title") {
                            Some(value) => value,
                            None => bail!("Missing field 'title' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

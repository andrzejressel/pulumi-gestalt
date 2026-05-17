#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApiKeyRestrictions {
    /// The Android apps that are allowed to use the key.
    #[builder(into)]
    #[serde(rename = "androidKeyRestrictions")]
    pub r#android_key_restrictions: Option<Box<super::super::types::projects::ApiKeyRestrictionsAndroidKeyRestrictions>>,
    /// A restriction for a specific service and optionally one or more specific methods. Requests are allowed if they match any of these restrictions. If no restrictions are specified, all targets are allowed.
    #[builder(into)]
    #[serde(rename = "apiTargets")]
    pub r#api_targets: Option<Vec<super::super::types::projects::ApiKeyRestrictionsApiTarget>>,
    /// The HTTP referrers (websites) that are allowed to use the key.
    #[builder(into)]
    #[serde(rename = "browserKeyRestrictions")]
    pub r#browser_key_restrictions: Option<Box<super::super::types::projects::ApiKeyRestrictionsBrowserKeyRestrictions>>,
    /// The iOS apps that are allowed to use the key.
    #[builder(into)]
    #[serde(rename = "iosKeyRestrictions")]
    pub r#ios_key_restrictions: Option<Box<super::super::types::projects::ApiKeyRestrictionsIosKeyRestrictions>>,
    /// The IP addresses of callers that are allowed to use the key.
    #[builder(into)]
    #[serde(rename = "serverKeyRestrictions")]
    pub r#server_key_restrictions: Option<Box<super::super::types::projects::ApiKeyRestrictionsServerKeyRestrictions>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApiKeyRestrictions {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "android_key_restrictions",
                    &self.r#android_key_restrictions,
                ),
                to_pulumi_object_field(
                    "api_targets",
                    &self.r#api_targets,
                ),
                to_pulumi_object_field(
                    "browser_key_restrictions",
                    &self.r#browser_key_restrictions,
                ),
                to_pulumi_object_field(
                    "ios_key_restrictions",
                    &self.r#ios_key_restrictions,
                ),
                to_pulumi_object_field(
                    "server_key_restrictions",
                    &self.r#server_key_restrictions,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApiKeyRestrictions {
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
                    r#android_key_restrictions: {
                        let field_value = match fields_map.get("android_key_restrictions") {
                            Some(value) => value,
                            None => bail!("Missing field 'android_key_restrictions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#api_targets: {
                        let field_value = match fields_map.get("api_targets") {
                            Some(value) => value,
                            None => bail!("Missing field 'api_targets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#browser_key_restrictions: {
                        let field_value = match fields_map.get("browser_key_restrictions") {
                            Some(value) => value,
                            None => bail!("Missing field 'browser_key_restrictions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ios_key_restrictions: {
                        let field_value = match fields_map.get("ios_key_restrictions") {
                            Some(value) => value,
                            None => bail!("Missing field 'ios_key_restrictions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_key_restrictions: {
                        let field_value = match fields_map.get("server_key_restrictions") {
                            Some(value) => value,
                            None => bail!("Missing field 'server_key_restrictions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

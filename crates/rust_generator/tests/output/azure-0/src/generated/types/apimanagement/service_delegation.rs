#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceDelegation {
    /// Should subscription requests be delegated to an external url? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "subscriptionsEnabled")]
    pub r#subscriptions_enabled: Option<bool>,
    /// The delegation URL.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    /// Should user registration requests be delegated to an external url? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "userRegistrationEnabled")]
    pub r#user_registration_enabled: Option<bool>,
    /// A base64-encoded validation key to validate, that a request is coming from Azure API Management.
    #[builder(into)]
    #[serde(rename = "validationKey")]
    pub r#validation_key: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceDelegation {
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
                    "subscriptions_enabled",
                    &self.r#subscriptions_enabled,
                ),
                to_pulumi_object_field(
                    "url",
                    &self.r#url,
                ),
                to_pulumi_object_field(
                    "user_registration_enabled",
                    &self.r#user_registration_enabled,
                ),
                to_pulumi_object_field(
                    "validation_key",
                    &self.r#validation_key,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceDelegation {
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
                    r#subscriptions_enabled: {
                        let field_value = match fields_map.get("subscriptions_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'subscriptions_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url: {
                        let field_value = match fields_map.get("url") {
                            Some(value) => value,
                            None => bail!("Missing field 'url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_registration_enabled: {
                        let field_value = match fields_map.get("user_registration_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_registration_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#validation_key: {
                        let field_value = match fields_map.get("validation_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'validation_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

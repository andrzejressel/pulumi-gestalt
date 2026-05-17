#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionEventingConfig {
    /// List containing additional auth configs.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "additionalVariables")]
    pub r#additional_variables: Option<Vec<super::super::types::integrationconnectors::ConnectionEventingConfigAdditionalVariable>>,
    /// authConfig for Eventing Configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "authConfig")]
    pub r#auth_config: Option<Box<super::super::types::integrationconnectors::ConnectionEventingConfigAuthConfig>>,
    /// Enrichment Enabled.
    #[builder(into)]
    #[serde(rename = "enrichmentEnabled")]
    pub r#enrichment_enabled: Option<bool>,
    /// registrationDestinationConfig
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "registrationDestinationConfig")]
    pub r#registration_destination_config: Box<super::super::types::integrationconnectors::ConnectionEventingConfigRegistrationDestinationConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionEventingConfig {
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
                    "additional_variables",
                    &self.r#additional_variables,
                ),
                to_pulumi_object_field(
                    "auth_config",
                    &self.r#auth_config,
                ),
                to_pulumi_object_field(
                    "enrichment_enabled",
                    &self.r#enrichment_enabled,
                ),
                to_pulumi_object_field(
                    "registration_destination_config",
                    &self.r#registration_destination_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectionEventingConfig {
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
                    r#additional_variables: {
                        let field_value = match fields_map.get("additional_variables") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_variables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auth_config: {
                        let field_value = match fields_map.get("auth_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'auth_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enrichment_enabled: {
                        let field_value = match fields_map.get("enrichment_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enrichment_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#registration_destination_config: {
                        let field_value = match fields_map.get("registration_destination_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'registration_destination_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

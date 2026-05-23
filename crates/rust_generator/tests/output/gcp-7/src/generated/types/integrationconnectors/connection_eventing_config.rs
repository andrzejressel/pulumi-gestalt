#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionEventingConfig {
    /// List containing additional auth configs.
    /// Structure is documented below.
    #[builder(into)]
    pub r#additional_variables: Option<Vec<super::super::types::integrationconnectors::ConnectionEventingConfigAdditionalVariable>>,
    /// authConfig for Eventing Configuration.
    /// Structure is documented below.
    #[builder(into)]
    pub r#auth_config: Option<Box<super::super::types::integrationconnectors::ConnectionEventingConfigAuthConfig>>,
    /// Enrichment Enabled.
    #[builder(into)]
    pub r#enrichment_enabled: Option<bool>,
    /// registrationDestinationConfig
    /// Structure is documented below.
    #[builder(into)]
    pub r#registration_destination_config: Box<super::super::types::integrationconnectors::ConnectionEventingConfigRegistrationDestinationConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionEventingConfig {
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
                    "additionalVariables",
                    &self.r#additional_variables,
                ),
                to_pulumi_object_field(
                    "authConfig",
                    &self.r#auth_config,
                ),
                to_pulumi_object_field(
                    "enrichmentEnabled",
                    &self.r#enrichment_enabled,
                ),
                to_pulumi_object_field(
                    "registrationDestinationConfig",
                    &self.r#registration_destination_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("additionalVariables") {
                            Some(value) => value,
                            None => bail!("Missing field 'additionalVariables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auth_config: {
                        let field_value = match fields_map.get("authConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'authConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enrichment_enabled: {
                        let field_value = match fields_map.get("enrichmentEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enrichmentEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#registration_destination_config: {
                        let field_value = match fields_map.get("registrationDestinationConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'registrationDestinationConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

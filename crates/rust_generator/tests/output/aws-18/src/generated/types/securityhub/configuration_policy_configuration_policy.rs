#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigurationPolicyConfigurationPolicy {
    /// A list that defines which security standards are enabled in the configuration policy. It must be defined if `service_enabled` is set to true.
    #[builder(into)]
    pub r#enabled_standard_arns: Option<Vec<String>>,
    /// Defines which security controls are enabled in the configuration policy and any customizations to parameters affecting them. See below.
    #[builder(into)]
    pub r#security_controls_configuration: Option<Box<super::super::types::securityhub::ConfigurationPolicyConfigurationPolicySecurityControlsConfiguration>>,
    /// Indicates whether Security Hub is enabled in the policy.
    #[builder(into)]
    pub r#service_enabled: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConfigurationPolicyConfigurationPolicy {
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
                    "enabledStandardArns",
                    &self.r#enabled_standard_arns,
                ),
                to_pulumi_object_field(
                    "securityControlsConfiguration",
                    &self.r#security_controls_configuration,
                ),
                to_pulumi_object_field(
                    "serviceEnabled",
                    &self.r#service_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConfigurationPolicyConfigurationPolicy {
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
                    r#enabled_standard_arns: {
                        let field_value = match fields_map.get("enabledStandardArns") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabledStandardArns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_controls_configuration: {
                        let field_value = match fields_map.get("securityControlsConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'securityControlsConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_enabled: {
                        let field_value = match fields_map.get("serviceEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

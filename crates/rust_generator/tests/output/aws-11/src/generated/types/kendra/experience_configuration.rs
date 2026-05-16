#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ExperienceConfiguration {
    /// The identifiers of your data sources and FAQs. Or, you can specify that you want to use documents indexed via the `BatchPutDocument API`. The provider will only perform drift detection of its value when present in a configuration. Detailed below.
    #[builder(into)]
    #[serde(rename = "contentSourceConfiguration")]
    pub r#content_source_configuration: Option<Box<super::super::types::kendra::ExperienceConfigurationContentSourceConfiguration>>,
    /// The AWS SSO field name that contains the identifiers of your users, such as their emails. Detailed below.
    #[builder(into)]
    #[serde(rename = "userIdentityConfiguration")]
    pub r#user_identity_configuration: Option<Box<super::super::types::kendra::ExperienceConfigurationUserIdentityConfiguration>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ExperienceConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("content_source_configuration".to_string(), self.r#content_source_configuration.to_pulumi_value().await);
            map.insert("user_identity_configuration".to_string(), self.r#user_identity_configuration.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ExperienceConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#content_source_configuration: {
                        let field_value = match fields_map.get("content_source_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_source_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kendra::ExperienceConfigurationContentSourceConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#user_identity_configuration: {
                        let field_value = match fields_map.get("user_identity_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_identity_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kendra::ExperienceConfigurationUserIdentityConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomActionTypeSettings {
    /// The URL returned to the AWS CodePipeline console that provides a deep link to the resources of the external system.
    #[builder(into)]
    #[serde(rename = "entityUrlTemplate")]
    pub r#entity_url_template: Option<String>,
    /// The URL returned to the AWS CodePipeline console that contains a link to the top-level landing page for the external system.
    #[builder(into)]
    #[serde(rename = "executionUrlTemplate")]
    pub r#execution_url_template: Option<String>,
    /// The URL returned to the AWS CodePipeline console that contains a link to the page where customers can update or change the configuration of the external action.
    #[builder(into)]
    #[serde(rename = "revisionUrlTemplate")]
    pub r#revision_url_template: Option<String>,
    /// The URL of a sign-up page where users can sign up for an external service and perform initial configuration of the action provided by that service.
    #[builder(into)]
    #[serde(rename = "thirdPartyConfigurationUrl")]
    pub r#third_party_configuration_url: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CustomActionTypeSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("entity_url_template".to_string(), self.r#entity_url_template.to_pulumi_value().await);
            map.insert("execution_url_template".to_string(), self.r#execution_url_template.to_pulumi_value().await);
            map.insert("revision_url_template".to_string(), self.r#revision_url_template.to_pulumi_value().await);
            map.insert("third_party_configuration_url".to_string(), self.r#third_party_configuration_url.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CustomActionTypeSettings {
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
                    r#entity_url_template: {
                        let field_value = match fields_map.get("entity_url_template") {
                            Some(value) => value,
                            None => bail!("Missing field 'entity_url_template' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#execution_url_template: {
                        let field_value = match fields_map.get("execution_url_template") {
                            Some(value) => value,
                            None => bail!("Missing field 'execution_url_template' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#revision_url_template: {
                        let field_value = match fields_map.get("revision_url_template") {
                            Some(value) => value,
                            None => bail!("Missing field 'revision_url_template' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#third_party_configuration_url: {
                        let field_value = match fields_map.get("third_party_configuration_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'third_party_configuration_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

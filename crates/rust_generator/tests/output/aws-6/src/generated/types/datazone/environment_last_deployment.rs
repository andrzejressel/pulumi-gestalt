#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EnvironmentLastDeployment {
    #[builder(into)]
    #[serde(rename = "deploymentId")]
    pub r#deployment_id: String,
    #[builder(into)]
    #[serde(rename = "deploymentStatus")]
    pub r#deployment_status: String,
    #[builder(into)]
    #[serde(rename = "deploymentType")]
    pub r#deployment_type: String,
    #[builder(into)]
    #[serde(rename = "failureReasons")]
    pub r#failure_reasons: Vec<super::super::types::datazone::EnvironmentLastDeploymentFailureReason>,
    #[builder(into)]
    #[serde(rename = "isDeploymentComplete")]
    pub r#is_deployment_complete: bool,
    #[builder(into)]
    #[serde(rename = "messages")]
    pub r#messages: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EnvironmentLastDeployment {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("deployment_id".to_string(), self.r#deployment_id.to_pulumi_value().await);
            map.insert("deployment_status".to_string(), self.r#deployment_status.to_pulumi_value().await);
            map.insert("deployment_type".to_string(), self.r#deployment_type.to_pulumi_value().await);
            map.insert("failure_reasons".to_string(), self.r#failure_reasons.to_pulumi_value().await);
            map.insert("is_deployment_complete".to_string(), self.r#is_deployment_complete.to_pulumi_value().await);
            map.insert("messages".to_string(), self.r#messages.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EnvironmentLastDeployment {
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
                    r#deployment_id: {
                        let field_value = match fields_map.get("deployment_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'deployment_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#deployment_status: {
                        let field_value = match fields_map.get("deployment_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'deployment_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#deployment_type: {
                        let field_value = match fields_map.get("deployment_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'deployment_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#failure_reasons: {
                        let field_value = match fields_map.get("failure_reasons") {
                            Some(value) => value,
                            None => bail!("Missing field 'failure_reasons' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::datazone::EnvironmentLastDeploymentFailureReason> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#is_deployment_complete: {
                        let field_value = match fields_map.get("is_deployment_complete") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_deployment_complete' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#messages: {
                        let field_value = match fields_map.get("messages") {
                            Some(value) => value,
                            None => bail!("Missing field 'messages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

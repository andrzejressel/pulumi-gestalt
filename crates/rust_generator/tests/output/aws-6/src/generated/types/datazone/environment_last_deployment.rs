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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "deploymentId",
                    &self.r#deployment_id,
                ),
                to_pulumi_object_field(
                    "deploymentStatus",
                    &self.r#deployment_status,
                ),
                to_pulumi_object_field(
                    "deploymentType",
                    &self.r#deployment_type,
                ),
                to_pulumi_object_field(
                    "failureReasons",
                    &self.r#failure_reasons,
                ),
                to_pulumi_object_field(
                    "isDeploymentComplete",
                    &self.r#is_deployment_complete,
                ),
                to_pulumi_object_field(
                    "messages",
                    &self.r#messages,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EnvironmentLastDeployment {
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
                    r#deployment_id: {
                        let field_value = match fields_map.get("deploymentId") {
                            Some(value) => value,
                            None => bail!("Missing field 'deploymentId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deployment_status: {
                        let field_value = match fields_map.get("deploymentStatus") {
                            Some(value) => value,
                            None => bail!("Missing field 'deploymentStatus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deployment_type: {
                        let field_value = match fields_map.get("deploymentType") {
                            Some(value) => value,
                            None => bail!("Missing field 'deploymentType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#failure_reasons: {
                        let field_value = match fields_map.get("failureReasons") {
                            Some(value) => value,
                            None => bail!("Missing field 'failureReasons' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_deployment_complete: {
                        let field_value = match fields_map.get("isDeploymentComplete") {
                            Some(value) => value,
                            None => bail!("Missing field 'isDeploymentComplete' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#messages: {
                        let field_value = match fields_map.get("messages") {
                            Some(value) => value,
                            None => bail!("Missing field 'messages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAppTemplate {
    #[builder(into)]
    #[serde(rename = "azureQueueScaleRules")]
    pub r#azure_queue_scale_rules: Vec<super::super::types::containerapp::GetAppTemplateAzureQueueScaleRule>,
    /// One or more `container` blocks as detailed below.
    #[builder(into)]
    #[serde(rename = "containers")]
    pub r#containers: Vec<super::super::types::containerapp::GetAppTemplateContainer>,
    #[builder(into)]
    #[serde(rename = "customScaleRules")]
    pub r#custom_scale_rules: Option<Vec<super::super::types::containerapp::GetAppTemplateCustomScaleRule>>,
    #[builder(into)]
    #[serde(rename = "httpScaleRules")]
    pub r#http_scale_rules: Vec<super::super::types::containerapp::GetAppTemplateHttpScaleRule>,
    /// One or more `init_container` blocks as detailed below.
    #[builder(into)]
    #[serde(rename = "initContainers")]
    pub r#init_containers: Vec<super::super::types::containerapp::GetAppTemplateInitContainer>,
    /// The maximum number of replicas for this container.
    #[builder(into)]
    #[serde(rename = "maxReplicas")]
    pub r#max_replicas: i32,
    /// The minimum number of replicas for this container.
    #[builder(into)]
    #[serde(rename = "minReplicas")]
    pub r#min_replicas: i32,
    /// The suffix string to which this `traffic_weight` applies.
    #[builder(into)]
    #[serde(rename = "revisionSuffix")]
    pub r#revision_suffix: String,
    #[builder(into)]
    #[serde(rename = "tcpScaleRules")]
    pub r#tcp_scale_rules: Vec<super::super::types::containerapp::GetAppTemplateTcpScaleRule>,
    /// A `volume` block as detailed below.
    #[builder(into)]
    #[serde(rename = "volumes")]
    pub r#volumes: Vec<super::super::types::containerapp::GetAppTemplateVolume>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAppTemplate {
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
                    "azure_queue_scale_rules",
                    &self.r#azure_queue_scale_rules,
                ),
                to_pulumi_object_field(
                    "containers",
                    &self.r#containers,
                ),
                to_pulumi_object_field(
                    "custom_scale_rules",
                    &self.r#custom_scale_rules,
                ),
                to_pulumi_object_field(
                    "http_scale_rules",
                    &self.r#http_scale_rules,
                ),
                to_pulumi_object_field(
                    "init_containers",
                    &self.r#init_containers,
                ),
                to_pulumi_object_field(
                    "max_replicas",
                    &self.r#max_replicas,
                ),
                to_pulumi_object_field(
                    "min_replicas",
                    &self.r#min_replicas,
                ),
                to_pulumi_object_field(
                    "revision_suffix",
                    &self.r#revision_suffix,
                ),
                to_pulumi_object_field(
                    "tcp_scale_rules",
                    &self.r#tcp_scale_rules,
                ),
                to_pulumi_object_field(
                    "volumes",
                    &self.r#volumes,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAppTemplate {
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
                    r#azure_queue_scale_rules: {
                        let field_value = match fields_map.get("azure_queue_scale_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'azure_queue_scale_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#containers: {
                        let field_value = match fields_map.get("containers") {
                            Some(value) => value,
                            None => bail!("Missing field 'containers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_scale_rules: {
                        let field_value = match fields_map.get("custom_scale_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_scale_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_scale_rules: {
                        let field_value = match fields_map.get("http_scale_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_scale_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#init_containers: {
                        let field_value = match fields_map.get("init_containers") {
                            Some(value) => value,
                            None => bail!("Missing field 'init_containers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_replicas: {
                        let field_value = match fields_map.get("max_replicas") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_replicas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_replicas: {
                        let field_value = match fields_map.get("min_replicas") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_replicas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#revision_suffix: {
                        let field_value = match fields_map.get("revision_suffix") {
                            Some(value) => value,
                            None => bail!("Missing field 'revision_suffix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tcp_scale_rules: {
                        let field_value = match fields_map.get("tcp_scale_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'tcp_scale_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volumes: {
                        let field_value = match fields_map.get("volumes") {
                            Some(value) => value,
                            None => bail!("Missing field 'volumes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

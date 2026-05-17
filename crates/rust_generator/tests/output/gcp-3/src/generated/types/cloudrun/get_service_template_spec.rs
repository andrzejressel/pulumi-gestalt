#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceTemplateSpec {
    /// ContainerConcurrency specifies the maximum allowed in-flight (concurrent)
    /// requests per container of the Revision. If not specified or 0, defaults to 80 when
    /// requested CPU >= 1 and defaults to 1 when requested CPU < 1.
    #[builder(into)]
    #[serde(rename = "containerConcurrency")]
    pub r#container_concurrency: i32,
    /// Containers defines the unit of execution for this Revision.
    #[builder(into)]
    #[serde(rename = "containers")]
    pub r#containers: Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainer>,
    /// Node Selector describes the hardware requirements of the resources.
    /// Use the following node selector keys to configure features on a Revision:
    ///   - 'run.googleapis.com/accelerator' sets the [type of GPU](https://cloud.google.com/run/docs/configuring/services/gpu) required by the Revision to run.
    #[builder(into)]
    #[serde(rename = "nodeSelector")]
    pub r#node_selector: std::collections::HashMap<String, String>,
    /// Email address of the IAM service account associated with the revision of the
    /// service. The service account represents the identity of the running revision,
    /// and determines what permissions the revision has. If not provided, the revision
    /// will use the project's default service account.
    #[builder(into)]
    #[serde(rename = "serviceAccountName")]
    pub r#service_account_name: String,
    /// ServingState holds a value describing the state the resources
    /// are in for this Revision.
    /// It is expected
    /// that the system will manipulate this based on routability and load.
    #[builder(into)]
    #[serde(rename = "servingState")]
    pub r#serving_state: String,
    /// TimeoutSeconds holds the max duration the instance is allowed for responding to a request.
    #[builder(into)]
    #[serde(rename = "timeoutSeconds")]
    pub r#timeout_seconds: i32,
    /// Volume represents a named volume in a container.
    #[builder(into)]
    #[serde(rename = "volumes")]
    pub r#volumes: Vec<super::super::types::cloudrun::GetServiceTemplateSpecVolume>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetServiceTemplateSpec {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "container_concurrency".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container_concurrency,
                )
                .await,
            );
            map.insert(
                "containers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#containers,
                )
                .await,
            );
            map.insert(
                "node_selector".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_selector,
                )
                .await,
            );
            map.insert(
                "service_account_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_account_name,
                )
                .await,
            );
            map.insert(
                "serving_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#serving_state,
                )
                .await,
            );
            map.insert(
                "timeout_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timeout_seconds,
                )
                .await,
            );
            map.insert(
                "volumes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#volumes,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetServiceTemplateSpec {
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
                    r#container_concurrency: {
                        let field_value = match fields_map.get("container_concurrency") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_concurrency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#node_selector: {
                        let field_value = match fields_map.get("node_selector") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_selector' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_name: {
                        let field_value = match fields_map.get("service_account_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#serving_state: {
                        let field_value = match fields_map.get("serving_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'serving_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout_seconds: {
                        let field_value = match fields_map.get("timeout_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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

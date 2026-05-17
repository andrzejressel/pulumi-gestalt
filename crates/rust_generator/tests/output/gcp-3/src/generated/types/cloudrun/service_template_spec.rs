#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTemplateSpec {
    /// ContainerConcurrency specifies the maximum allowed in-flight (concurrent)
    /// requests per container of the Revision. If not specified or 0, defaults to 80 when
    /// requested CPU >= 1 and defaults to 1 when requested CPU < 1.
    #[builder(into)]
    #[serde(rename = "containerConcurrency")]
    pub r#container_concurrency: Option<i32>,
    /// Containers defines the unit of execution for this Revision.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "containers")]
    pub r#containers: Option<Vec<super::super::types::cloudrun::ServiceTemplateSpecContainer>>,
    /// Node Selector describes the hardware requirements of the resources.
    /// Use the following node selector keys to configure features on a Revision:
    /// - `run.googleapis.com/accelerator` sets the [type of GPU](https://cloud.google.com/run/docs/configuring/services/gpu) required by the Revision to run.
    #[builder(into)]
    #[serde(rename = "nodeSelector")]
    pub r#node_selector: Option<std::collections::HashMap<String, String>>,
    /// Email address of the IAM service account associated with the revision of the
    /// service. The service account represents the identity of the running revision,
    /// and determines what permissions the revision has. If not provided, the revision
    /// will use the project's default service account.
    #[builder(into)]
    #[serde(rename = "serviceAccountName")]
    pub r#service_account_name: Option<String>,
    /// (Output, Deprecated)
    /// ServingState holds a value describing the state the resources
    /// are in for this Revision.
    /// It is expected
    /// that the system will manipulate this based on routability and load.
    /// 
    /// > **Warning:** `serving_state` is deprecated and will be removed in a future major release. This field is not supported by the Cloud Run API.
    #[builder(into)]
    #[serde(rename = "servingState")]
    pub r#serving_state: Option<String>,
    /// TimeoutSeconds holds the max duration the instance is allowed for responding to a request.
    #[builder(into)]
    #[serde(rename = "timeoutSeconds")]
    pub r#timeout_seconds: Option<i32>,
    /// Volume represents a named volume in a container.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "volumes")]
    pub r#volumes: Option<Vec<super::super::types::cloudrun::ServiceTemplateSpecVolume>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceTemplateSpec {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "container_concurrency",
                    &self.r#container_concurrency,
                ),
                to_pulumi_object_field(
                    "containers",
                    &self.r#containers,
                ),
                to_pulumi_object_field(
                    "node_selector",
                    &self.r#node_selector,
                ),
                to_pulumi_object_field(
                    "service_account_name",
                    &self.r#service_account_name,
                ),
                to_pulumi_object_field(
                    "serving_state",
                    &self.r#serving_state,
                ),
                to_pulumi_object_field(
                    "timeout_seconds",
                    &self.r#timeout_seconds,
                ),
                to_pulumi_object_field(
                    "volumes",
                    &self.r#volumes,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceTemplateSpec {
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

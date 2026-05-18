#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TargetExecutionConfig {
    /// Optional. Cloud Storage location in which to store execution outputs. This can either be a bucket ("gs://my-bucket") or a path within a bucket ("gs://my-bucket/my-dir"). If unspecified, a default bucket located in the same region will be used.
    #[builder(into)]
    #[serde(rename = "artifactStorage")]
    pub r#artifact_storage: Option<String>,
    /// Optional. Execution timeout for a Cloud Build Execution. This must be between 10m and 24h in seconds format. If unspecified, a default timeout of 1h is used.
    #[builder(into)]
    #[serde(rename = "executionTimeout")]
    pub r#execution_timeout: Option<String>,
    /// Optional. Google service account to use for execution. If unspecified, the project execution service account (-compute@developer.gserviceaccount.com) is used.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Option<String>,
    /// Required. Usages when this configuration should be applied.
    #[builder(into)]
    #[serde(rename = "usages")]
    pub r#usages: Vec<String>,
    /// Optional. If true, additional logging will be enabled when running builds in this execution environment.
    #[builder(into)]
    #[serde(rename = "verbose")]
    pub r#verbose: Option<bool>,
    /// Optional. The resource name of the `WorkerPool`, with the format `projects/{project}/locations/{location}/workerPools/{worker_pool}`. If this optional field is unspecified, the default Cloud Build pool will be used.
    #[builder(into)]
    #[serde(rename = "workerPool")]
    pub r#worker_pool: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TargetExecutionConfig {
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
                    "artifact_storage",
                    &self.r#artifact_storage,
                ),
                to_pulumi_object_field(
                    "execution_timeout",
                    &self.r#execution_timeout,
                ),
                to_pulumi_object_field(
                    "service_account",
                    &self.r#service_account,
                ),
                to_pulumi_object_field(
                    "usages",
                    &self.r#usages,
                ),
                to_pulumi_object_field(
                    "verbose",
                    &self.r#verbose,
                ),
                to_pulumi_object_field(
                    "worker_pool",
                    &self.r#worker_pool,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TargetExecutionConfig {
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
                    r#artifact_storage: {
                        let field_value = match fields_map.get("artifact_storage") {
                            Some(value) => value,
                            None => bail!("Missing field 'artifact_storage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#execution_timeout: {
                        let field_value = match fields_map.get("execution_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'execution_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account: {
                        let field_value = match fields_map.get("service_account") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#usages: {
                        let field_value = match fields_map.get("usages") {
                            Some(value) => value,
                            None => bail!("Missing field 'usages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#verbose: {
                        let field_value = match fields_map.get("verbose") {
                            Some(value) => value,
                            None => bail!("Missing field 'verbose' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#worker_pool: {
                        let field_value = match fields_map.get("worker_pool") {
                            Some(value) => value,
                            None => bail!("Missing field 'worker_pool' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

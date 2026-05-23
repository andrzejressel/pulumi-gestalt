#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobTemplateTemplate {
    /// Holds the single container that defines the unit of execution for this task.
    #[builder(into)]
    pub r#containers: Vec<super::super::types::cloudrunv2::GetJobTemplateTemplateContainer>,
    /// A reference to a customer managed encryption key (CMEK) to use to encrypt this container image. For more information, go to https://cloud.google.com/run/docs/securing/using-cmek
    #[builder(into)]
    pub r#encryption_key: String,
    /// The execution environment being used to host this Task. Possible values: ["EXECUTION_ENVIRONMENT_GEN1", "EXECUTION_ENVIRONMENT_GEN2"]
    #[builder(into)]
    pub r#execution_environment: String,
    /// Number of retries allowed per Task, before marking this Task failed.
    #[builder(into)]
    pub r#max_retries: i32,
    /// Email address of the IAM service account associated with the Task of a Job. The service account represents the identity of the running task, and determines what permissions the task has. If not provided, the task will use the project's default service account.
    #[builder(into)]
    pub r#service_account: String,
    /// Max allowed time duration the Task may be active before the system will actively try to mark it failed and kill associated containers. This applies per attempt of a task, meaning each retry can run for the full timeout.
    /// 
    /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
    #[builder(into)]
    pub r#timeout: String,
    /// A list of Volumes to make available to containers.
    #[builder(into)]
    pub r#volumes: Vec<super::super::types::cloudrunv2::GetJobTemplateTemplateVolume>,
    /// VPC Access configuration to use for this Task. For more information, visit https://cloud.google.com/run/docs/configuring/connecting-vpc.
    #[builder(into)]
    pub r#vpc_accesses: Vec<super::super::types::cloudrunv2::GetJobTemplateTemplateVpcAccess>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetJobTemplateTemplate {
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
                    "containers",
                    &self.r#containers,
                ),
                to_pulumi_object_field(
                    "encryptionKey",
                    &self.r#encryption_key,
                ),
                to_pulumi_object_field(
                    "executionEnvironment",
                    &self.r#execution_environment,
                ),
                to_pulumi_object_field(
                    "maxRetries",
                    &self.r#max_retries,
                ),
                to_pulumi_object_field(
                    "serviceAccount",
                    &self.r#service_account,
                ),
                to_pulumi_object_field(
                    "timeout",
                    &self.r#timeout,
                ),
                to_pulumi_object_field(
                    "volumes",
                    &self.r#volumes,
                ),
                to_pulumi_object_field(
                    "vpcAccesses",
                    &self.r#vpc_accesses,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetJobTemplateTemplate {
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
                    r#containers: {
                        let field_value = match fields_map.get("containers") {
                            Some(value) => value,
                            None => bail!("Missing field 'containers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_key: {
                        let field_value = match fields_map.get("encryptionKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryptionKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#execution_environment: {
                        let field_value = match fields_map.get("executionEnvironment") {
                            Some(value) => value,
                            None => bail!("Missing field 'executionEnvironment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_retries: {
                        let field_value = match fields_map.get("maxRetries") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxRetries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account: {
                        let field_value = match fields_map.get("serviceAccount") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceAccount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout: {
                        let field_value = match fields_map.get("timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#vpc_accesses: {
                        let field_value = match fields_map.get("vpcAccesses") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpcAccesses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

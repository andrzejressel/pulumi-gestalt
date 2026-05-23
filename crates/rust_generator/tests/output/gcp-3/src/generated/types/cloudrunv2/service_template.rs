#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTemplate {
    /// Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.
    /// Cloud Run API v2 does not support annotations with `run.googleapis.com`, `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev` namespaces, and they will be rejected.
    /// All system annotations in v1 now have a corresponding field in v2 RevisionTemplate.
    /// This field follows Kubernetes annotations' namespacing, limits, and rules.
    #[builder(into)]
    #[serde(rename = "annotations")]
    pub r#annotations: Option<std::collections::HashMap<String, String>>,
    /// Holds the containers that define the unit of execution for this Service.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "containers")]
    pub r#containers: Option<Vec<super::super::types::cloudrunv2::ServiceTemplateContainer>>,
    /// A reference to a customer managed encryption key (CMEK) to use to encrypt this container image. For more information, go to https://cloud.google.com/run/docs/securing/using-cmek
    #[builder(into)]
    #[serde(rename = "encryptionKey")]
    pub r#encryption_key: Option<String>,
    /// The sandbox environment to host this Revision.
    /// Possible values are: `EXECUTION_ENVIRONMENT_GEN1`, `EXECUTION_ENVIRONMENT_GEN2`.
    #[builder(into)]
    #[serde(rename = "executionEnvironment")]
    pub r#execution_environment: Option<String>,
    /// Unstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment, state, etc.
    /// For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.
    /// Cloud Run API v2 does not support labels with `run.googleapis.com`, `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev` namespaces, and they will be rejected.
    /// All system labels in v1 now have a corresponding field in v2 RevisionTemplate.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// Sets the maximum number of requests that each serving instance can receive.
    /// If not specified or 0, defaults to 80 when requested CPU >= 1 and defaults to 1 when requested CPU < 1.
    #[builder(into)]
    #[serde(rename = "maxInstanceRequestConcurrency")]
    pub r#max_instance_request_concurrency: Option<i32>,
    /// Node Selector describes the hardware requirements of the resources.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "nodeSelector")]
    pub r#node_selector: Option<Box<super::super::types::cloudrunv2::ServiceTemplateNodeSelector>>,
    /// The unique name for the revision. If this field is omitted, it will be automatically generated based on the Service name.
    #[builder(into)]
    #[serde(rename = "revision")]
    pub r#revision: Option<String>,
    /// Scaling settings for this Revision.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "scaling")]
    pub r#scaling: Option<Box<super::super::types::cloudrunv2::ServiceTemplateScaling>>,
    /// Email address of the IAM service account associated with the revision of the service. The service account represents the identity of the running revision, and determines what permissions the revision has. If not provided, the revision will use the project's default service account.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Option<String>,
    /// Enables Cloud Service Mesh for this Revision.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "serviceMesh")]
    pub r#service_mesh: Option<Box<super::super::types::cloudrunv2::ServiceTemplateServiceMesh>>,
    /// Enables session affinity. For more information, go to https://cloud.google.com/run/docs/configuring/session-affinity
    #[builder(into)]
    #[serde(rename = "sessionAffinity")]
    pub r#session_affinity: Option<bool>,
    /// Max allowed time for an instance to respond to a request.
    /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<String>,
    /// A list of Volumes to make available to containers.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "volumes")]
    pub r#volumes: Option<Vec<super::super::types::cloudrunv2::ServiceTemplateVolume>>,
    /// VPC Access configuration to use for this Task. For more information, visit https://cloud.google.com/run/docs/configuring/connecting-vpc.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "vpcAccess")]
    pub r#vpc_access: Option<Box<super::super::types::cloudrunv2::ServiceTemplateVpcAccess>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceTemplate {
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
                    "annotations",
                    &self.r#annotations,
                ),
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
                    "labels",
                    &self.r#labels,
                ),
                to_pulumi_object_field(
                    "maxInstanceRequestConcurrency",
                    &self.r#max_instance_request_concurrency,
                ),
                to_pulumi_object_field(
                    "nodeSelector",
                    &self.r#node_selector,
                ),
                to_pulumi_object_field(
                    "revision",
                    &self.r#revision,
                ),
                to_pulumi_object_field(
                    "scaling",
                    &self.r#scaling,
                ),
                to_pulumi_object_field(
                    "serviceAccount",
                    &self.r#service_account,
                ),
                to_pulumi_object_field(
                    "serviceMesh",
                    &self.r#service_mesh,
                ),
                to_pulumi_object_field(
                    "sessionAffinity",
                    &self.r#session_affinity,
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
                    "vpcAccess",
                    &self.r#vpc_access,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceTemplate {
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
                    r#annotations: {
                        let field_value = match fields_map.get("annotations") {
                            Some(value) => value,
                            None => bail!("Missing field 'annotations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_instance_request_concurrency: {
                        let field_value = match fields_map.get("maxInstanceRequestConcurrency") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxInstanceRequestConcurrency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_selector: {
                        let field_value = match fields_map.get("nodeSelector") {
                            Some(value) => value,
                            None => bail!("Missing field 'nodeSelector' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#revision: {
                        let field_value = match fields_map.get("revision") {
                            Some(value) => value,
                            None => bail!("Missing field 'revision' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scaling: {
                        let field_value = match fields_map.get("scaling") {
                            Some(value) => value,
                            None => bail!("Missing field 'scaling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#service_mesh: {
                        let field_value = match fields_map.get("serviceMesh") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceMesh' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_affinity: {
                        let field_value = match fields_map.get("sessionAffinity") {
                            Some(value) => value,
                            None => bail!("Missing field 'sessionAffinity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#vpc_access: {
                        let field_value = match fields_map.get("vpcAccess") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpcAccess' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

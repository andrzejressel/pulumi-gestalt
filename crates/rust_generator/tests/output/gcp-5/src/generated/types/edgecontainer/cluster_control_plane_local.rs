#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterControlPlaneLocal {
    /// Only machines matching this filter will be allowed to host control
    /// plane nodes. The filtering language accepts strings like "name=<name>",
    /// and is documented here: [AIP-160](https://google.aip.dev/160).
    #[builder(into)]
    #[serde(rename = "machineFilter")]
    pub r#machine_filter: Option<String>,
    /// The number of nodes to serve as replicas of the Control Plane.
    /// Only 1 and 3 are supported.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Option<i32>,
    /// Name of the Google Distributed Cloud Edge zones where this node pool
    /// will be created. For example: `us-central1-edge-customer-a`.
    #[builder(into)]
    #[serde(rename = "nodeLocation")]
    pub r#node_location: Option<String>,
    /// Policy configuration about how user applications are deployed.
    /// Possible values are: `SHARED_DEPLOYMENT_POLICY_UNSPECIFIED`, `ALLOWED`, `DISALLOWED`.
    #[builder(into)]
    #[serde(rename = "sharedDeploymentPolicy")]
    pub r#shared_deployment_policy: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterControlPlaneLocal {
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
                    "machine_filter",
                    &self.r#machine_filter,
                ),
                to_pulumi_object_field(
                    "node_count",
                    &self.r#node_count,
                ),
                to_pulumi_object_field(
                    "node_location",
                    &self.r#node_location,
                ),
                to_pulumi_object_field(
                    "shared_deployment_policy",
                    &self.r#shared_deployment_policy,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterControlPlaneLocal {
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
                    r#machine_filter: {
                        let field_value = match fields_map.get("machine_filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'machine_filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_count: {
                        let field_value = match fields_map.get("node_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_location: {
                        let field_value = match fields_map.get("node_location") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shared_deployment_policy: {
                        let field_value = match fields_map.get("shared_deployment_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'shared_deployment_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

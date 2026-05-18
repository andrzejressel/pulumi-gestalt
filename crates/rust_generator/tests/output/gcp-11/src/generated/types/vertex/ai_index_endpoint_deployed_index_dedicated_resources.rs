#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AiIndexEndpointDeployedIndexDedicatedResources {
    /// The minimum number of replicas this DeployedModel will be always deployed on.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "machineSpec")]
    pub r#machine_spec: Box<super::super::types::vertex::AiIndexEndpointDeployedIndexDedicatedResourcesMachineSpec>,
    /// The maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If maxReplicaCount is not set, the default value is minReplicaCount
    #[builder(into)]
    #[serde(rename = "maxReplicaCount")]
    pub r#max_replica_count: Option<i32>,
    /// The minimum number of machine replicas this DeployedModel will be always deployed on. This value must be greater than or equal to 1.
    #[builder(into)]
    #[serde(rename = "minReplicaCount")]
    pub r#min_replica_count: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AiIndexEndpointDeployedIndexDedicatedResources {
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
                    "machine_spec",
                    &self.r#machine_spec,
                ),
                to_pulumi_object_field(
                    "max_replica_count",
                    &self.r#max_replica_count,
                ),
                to_pulumi_object_field(
                    "min_replica_count",
                    &self.r#min_replica_count,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AiIndexEndpointDeployedIndexDedicatedResources {
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
                    r#machine_spec: {
                        let field_value = match fields_map.get("machine_spec") {
                            Some(value) => value,
                            None => bail!("Missing field 'machine_spec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_replica_count: {
                        let field_value = match fields_map.get("max_replica_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_replica_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_replica_count: {
                        let field_value = match fields_map.get("min_replica_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_replica_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

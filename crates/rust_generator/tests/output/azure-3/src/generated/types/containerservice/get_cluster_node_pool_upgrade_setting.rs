#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterNodePoolUpgradeSetting {
    /// The amount of time in minutes to wait on eviction of pods and graceful termination per node. This eviction wait time honors waiting on pod disruption budgets. If this time is exceeded, the upgrade fails.
    #[builder(into)]
    #[serde(rename = "drainTimeoutInMinutes")]
    pub r#drain_timeout_in_minutes: i32,
    /// The maximum number or percentage of nodes which will be added to the Node Pool size during an upgrade.
    #[builder(into)]
    #[serde(rename = "maxSurge")]
    pub r#max_surge: String,
    /// The amount of time in minutes to wait after draining a node and before reimaging it and moving on to next node.
    #[builder(into)]
    #[serde(rename = "nodeSoakDurationInMinutes")]
    pub r#node_soak_duration_in_minutes: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterNodePoolUpgradeSetting {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "drain_timeout_in_minutes",
                    &self.r#drain_timeout_in_minutes,
                ),
                to_pulumi_object_field(
                    "max_surge",
                    &self.r#max_surge,
                ),
                to_pulumi_object_field(
                    "node_soak_duration_in_minutes",
                    &self.r#node_soak_duration_in_minutes,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterNodePoolUpgradeSetting {
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
                    r#drain_timeout_in_minutes: {
                        let field_value = match fields_map.get("drain_timeout_in_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'drain_timeout_in_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_surge: {
                        let field_value = match fields_map.get("max_surge") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_surge' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_soak_duration_in_minutes: {
                        let field_value = match fields_map.get("node_soak_duration_in_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_soak_duration_in_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

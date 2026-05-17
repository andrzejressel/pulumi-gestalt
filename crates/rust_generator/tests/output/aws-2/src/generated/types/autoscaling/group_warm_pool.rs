#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GroupWarmPool {
    /// Whether instances in the Auto Scaling group can be returned to the warm pool on scale in. The default is to terminate instances in the Auto Scaling group when the group scales in.
    #[builder(into)]
    #[serde(rename = "instanceReusePolicy")]
    pub r#instance_reuse_policy: Option<Box<super::super::types::autoscaling::GroupWarmPoolInstanceReusePolicy>>,
    /// Total maximum number of instances that are allowed to be in the warm pool or in any state except Terminated for the Auto Scaling group.
    #[builder(into)]
    #[serde(rename = "maxGroupPreparedCapacity")]
    pub r#max_group_prepared_capacity: Option<i32>,
    /// Minimum number of instances to maintain in the warm pool. This helps you to ensure that there is always a certain number of warmed instances available to handle traffic spikes. Defaults to 0 if not specified.
    #[builder(into)]
    #[serde(rename = "minSize")]
    pub r#min_size: Option<i32>,
    /// Sets the instance state to transition to after the lifecycle hooks finish. Valid values are: Stopped (default), Running or Hibernated.
    #[builder(into)]
    #[serde(rename = "poolState")]
    pub r#pool_state: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GroupWarmPool {
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
                "instance_reuse_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_reuse_policy,
                )
                .await,
            );
            map.insert(
                "max_group_prepared_capacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_group_prepared_capacity,
                )
                .await,
            );
            map.insert(
                "min_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_size,
                )
                .await,
            );
            map.insert(
                "pool_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pool_state,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GroupWarmPool {
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
                    r#instance_reuse_policy: {
                        let field_value = match fields_map.get("instance_reuse_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_reuse_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_group_prepared_capacity: {
                        let field_value = match fields_map.get("max_group_prepared_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_group_prepared_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_size: {
                        let field_value = match fields_map.get("min_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pool_state: {
                        let field_value = match fields_map.get("pool_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'pool_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

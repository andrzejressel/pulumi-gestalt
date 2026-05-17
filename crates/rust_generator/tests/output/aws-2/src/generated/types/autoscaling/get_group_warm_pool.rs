#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetGroupWarmPool {
    /// List of instance reuse policy objects.
    #[builder(into)]
    #[serde(rename = "instanceReusePolicies")]
    pub r#instance_reuse_policies: Vec<super::super::types::autoscaling::GetGroupWarmPoolInstanceReusePolicy>,
    #[builder(into)]
    #[serde(rename = "maxGroupPreparedCapacity")]
    pub r#max_group_prepared_capacity: i32,
    /// Minimum number of instances to maintain in the warm pool.
    #[builder(into)]
    #[serde(rename = "minSize")]
    pub r#min_size: i32,
    /// Instance state to transition to after the lifecycle actions are complete.
    #[builder(into)]
    #[serde(rename = "poolState")]
    pub r#pool_state: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetGroupWarmPool {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "instance_reuse_policies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_reuse_policies,
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
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetGroupWarmPool {
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
                    r#instance_reuse_policies: {
                        let field_value = match fields_map.get("instance_reuse_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_reuse_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NodePoolUpgradeSettingsBlueGreenSettings {
    /// Time needed after draining the entire blue pool.
    /// After this period, the blue pool will be cleaned up.
    #[builder(into)]
    #[serde(rename = "nodePoolSoakDuration")]
    pub r#node_pool_soak_duration: Option<String>,
    /// Specifies the standard policy settings for blue-green upgrades.
    #[builder(into)]
    #[serde(rename = "standardRolloutPolicy")]
    pub r#standard_rollout_policy: Box<super::super::types::container::NodePoolUpgradeSettingsBlueGreenSettingsStandardRolloutPolicy>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NodePoolUpgradeSettingsBlueGreenSettings {
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
                "node_pool_soak_duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_pool_soak_duration,
                )
                .await,
            );
            map.insert(
                "standard_rollout_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#standard_rollout_policy,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NodePoolUpgradeSettingsBlueGreenSettings {
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
                    r#node_pool_soak_duration: {
                        let field_value = match fields_map.get("node_pool_soak_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_pool_soak_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#standard_rollout_policy: {
                        let field_value = match fields_map.get("standard_rollout_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'standard_rollout_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

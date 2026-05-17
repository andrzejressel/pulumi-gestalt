#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NodeGroupAutoscalingPolicy {
    /// Maximum size of the node group. Set to a value less than or equal
    /// to 100 and greater than or equal to min-nodes.
    #[builder(into)]
    #[serde(rename = "maxNodes")]
    pub r#max_nodes: Option<i32>,
    /// Minimum size of the node group. Must be less
    /// than or equal to max-nodes. The default value is 0.
    #[builder(into)]
    #[serde(rename = "minNodes")]
    pub r#min_nodes: Option<i32>,
    /// The autoscaling mode. Set to one of the following:
    /// - OFF: Disables the autoscaler.
    /// - ON: Enables scaling in and scaling out.
    /// - ONLY_SCALE_OUT: Enables only scaling out.
    /// You must use this mode if your node groups are configured to
    /// restart their hosted VMs on minimal servers.
    /// Possible values are: `OFF`, `ON`, `ONLY_SCALE_OUT`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NodeGroupAutoscalingPolicy {
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
                "max_nodes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_nodes,
                )
                .await,
            );
            map.insert(
                "min_nodes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_nodes,
                )
                .await,
            );
            map.insert(
                "mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mode,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NodeGroupAutoscalingPolicy {
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
                    r#max_nodes: {
                        let field_value = match fields_map.get("max_nodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_nodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_nodes: {
                        let field_value = match fields_map.get("min_nodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_nodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mode: {
                        let field_value = match fields_map.get("mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

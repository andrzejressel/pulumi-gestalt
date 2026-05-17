#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterNodeTypeConfig {
    /// Customized number of cores available to each node of the type.
    /// This number must always be one of `nodeType.availableCustomCoreCounts`.
    /// If zero is provided max value from `nodeType.availableCustomCoreCounts` will be used.
    /// Once the customer is created then corecount cannot be changed.
    #[builder(into)]
    #[serde(rename = "customCoreCount")]
    pub r#custom_core_count: Option<i32>,
    /// The number of nodes of this type in the cluster.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: i32,
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "nodeTypeId")]
    pub r#node_type_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterNodeTypeConfig {
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
                "custom_core_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_core_count,
                )
                .await,
            );
            map.insert(
                "node_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_count,
                )
                .await,
            );
            map.insert(
                "node_type_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_type_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterNodeTypeConfig {
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
                    r#custom_core_count: {
                        let field_value = match fields_map.get("custom_core_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_core_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#node_type_id: {
                        let field_value = match fields_map.get("node_type_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_type_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

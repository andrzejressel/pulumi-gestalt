#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterNodePoolNetworkConfigAdditionalPodNetworkConfig {
    /// The maximum number of pods per node which use this pod network.
    #[builder(into)]
    #[serde(rename = "maxPodsPerNode")]
    pub r#max_pods_per_node: Option<i32>,
    /// The name of the secondary range on the subnet which provides IP address for this pod range.
    #[builder(into)]
    #[serde(rename = "secondaryPodRange")]
    pub r#secondary_pod_range: Option<String>,
    /// The name or self_link of the Google Compute Engine
    /// subnetwork in which the cluster's instances are launched.
    #[builder(into)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterNodePoolNetworkConfigAdditionalPodNetworkConfig {
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
                "max_pods_per_node".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_pods_per_node,
                )
                .await,
            );
            map.insert(
                "secondary_pod_range".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secondary_pod_range,
                )
                .await,
            );
            map.insert(
                "subnetwork".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subnetwork,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterNodePoolNetworkConfigAdditionalPodNetworkConfig {
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
                    r#max_pods_per_node: {
                        let field_value = match fields_map.get("max_pods_per_node") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_pods_per_node' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secondary_pod_range: {
                        let field_value = match fields_map.get("secondary_pod_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'secondary_pod_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnetwork: {
                        let field_value = match fields_map.get("subnetwork") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetwork' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

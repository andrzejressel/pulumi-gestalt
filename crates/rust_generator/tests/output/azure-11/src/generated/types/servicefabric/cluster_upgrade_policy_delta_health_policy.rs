#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterUpgradePolicyDeltaHealthPolicy {
    /// Specifies the maximum tolerated percentage of delta unhealthy applications that can have aggregated health states of error. If the current unhealthy applications do not respect the percentage relative to the state at the beginning of the upgrade, the cluster is unhealthy. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "maxDeltaUnhealthyApplicationsPercent")]
    pub r#max_delta_unhealthy_applications_percent: Option<i32>,
    /// Specifies the maximum tolerated percentage of delta unhealthy nodes that can have aggregated health states of error. If the current unhealthy nodes do not respect the percentage relative to the state at the beginning of the upgrade, the cluster is unhealthy. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "maxDeltaUnhealthyNodesPercent")]
    pub r#max_delta_unhealthy_nodes_percent: Option<i32>,
    /// Specifies the maximum tolerated percentage of upgrade domain delta unhealthy nodes that can have aggregated health state of error. If there is any upgrade domain where the current unhealthy nodes do not respect the percentage relative to the state at the beginning of the upgrade, the cluster is unhealthy. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "maxUpgradeDomainDeltaUnhealthyNodesPercent")]
    pub r#max_upgrade_domain_delta_unhealthy_nodes_percent: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterUpgradePolicyDeltaHealthPolicy {
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
                    "max_delta_unhealthy_applications_percent",
                    &self.r#max_delta_unhealthy_applications_percent,
                ),
                to_pulumi_object_field(
                    "max_delta_unhealthy_nodes_percent",
                    &self.r#max_delta_unhealthy_nodes_percent,
                ),
                to_pulumi_object_field(
                    "max_upgrade_domain_delta_unhealthy_nodes_percent",
                    &self.r#max_upgrade_domain_delta_unhealthy_nodes_percent,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterUpgradePolicyDeltaHealthPolicy {
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
                    r#max_delta_unhealthy_applications_percent: {
                        let field_value = match fields_map.get("max_delta_unhealthy_applications_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_delta_unhealthy_applications_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_delta_unhealthy_nodes_percent: {
                        let field_value = match fields_map.get("max_delta_unhealthy_nodes_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_delta_unhealthy_nodes_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_upgrade_domain_delta_unhealthy_nodes_percent: {
                        let field_value = match fields_map.get("max_upgrade_domain_delta_unhealthy_nodes_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_upgrade_domain_delta_unhealthy_nodes_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

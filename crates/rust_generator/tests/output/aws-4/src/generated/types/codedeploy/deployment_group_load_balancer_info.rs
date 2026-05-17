#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeploymentGroupLoadBalancerInfo {
    /// The Classic Elastic Load Balancer to use in a deployment. Conflicts with `target_group_info` and `target_group_pair_info`.
    #[builder(into)]
    #[serde(rename = "elbInfos")]
    pub r#elb_infos: Option<Vec<super::super::types::codedeploy::DeploymentGroupLoadBalancerInfoElbInfo>>,
    /// The (Application/Network Load Balancer) target group to use in a deployment. Conflicts with `elb_info` and `target_group_pair_info`.
    #[builder(into)]
    #[serde(rename = "targetGroupInfos")]
    pub r#target_group_infos: Option<Vec<super::super::types::codedeploy::DeploymentGroupLoadBalancerInfoTargetGroupInfo>>,
    /// The (Application/Network Load Balancer) target group pair to use in a deployment. Conflicts with `elb_info` and `target_group_info`.
    #[builder(into)]
    #[serde(rename = "targetGroupPairInfo")]
    pub r#target_group_pair_info: Option<Box<super::super::types::codedeploy::DeploymentGroupLoadBalancerInfoTargetGroupPairInfo>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DeploymentGroupLoadBalancerInfo {
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
                "elb_infos".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#elb_infos,
                )
                .await,
            );
            map.insert(
                "target_group_infos".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_group_infos,
                )
                .await,
            );
            map.insert(
                "target_group_pair_info".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_group_pair_info,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DeploymentGroupLoadBalancerInfo {
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
                    r#elb_infos: {
                        let field_value = match fields_map.get("elb_infos") {
                            Some(value) => value,
                            None => bail!("Missing field 'elb_infos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_group_infos: {
                        let field_value = match fields_map.get("target_group_infos") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_group_infos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_group_pair_info: {
                        let field_value = match fields_map.get("target_group_pair_info") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_group_pair_info' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeploymentGroupLoadBalancerInfoTargetGroupPairInfo {
    /// Configuration block for the production traffic route (documented below).
    #[builder(into)]
    #[serde(rename = "prodTrafficRoute")]
    pub r#prod_traffic_route: Box<super::super::types::codedeploy::DeploymentGroupLoadBalancerInfoTargetGroupPairInfoProdTrafficRoute>,
    /// Configuration blocks for a target group within a target group pair (documented below).
    #[builder(into)]
    #[serde(rename = "targetGroups")]
    pub r#target_groups: Vec<super::super::types::codedeploy::DeploymentGroupLoadBalancerInfoTargetGroupPairInfoTargetGroup>,
    /// Configuration block for the test traffic route (documented below).
    #[builder(into)]
    #[serde(rename = "testTrafficRoute")]
    pub r#test_traffic_route: Option<Box<super::super::types::codedeploy::DeploymentGroupLoadBalancerInfoTargetGroupPairInfoTestTrafficRoute>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DeploymentGroupLoadBalancerInfoTargetGroupPairInfo {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "prod_traffic_route",
                    &self.r#prod_traffic_route,
                ),
                to_pulumi_object_field(
                    "target_groups",
                    &self.r#target_groups,
                ),
                to_pulumi_object_field(
                    "test_traffic_route",
                    &self.r#test_traffic_route,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DeploymentGroupLoadBalancerInfoTargetGroupPairInfo {
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
                    r#prod_traffic_route: {
                        let field_value = match fields_map.get("prod_traffic_route") {
                            Some(value) => value,
                            None => bail!("Missing field 'prod_traffic_route' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_groups: {
                        let field_value = match fields_map.get("target_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#test_traffic_route: {
                        let field_value = match fields_map.get("test_traffic_route") {
                            Some(value) => value,
                            None => bail!("Missing field 'test_traffic_route' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

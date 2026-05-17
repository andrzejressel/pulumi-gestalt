#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TaskSetLoadBalancer {
    /// The name of the container to associate with the load balancer (as it appears in a container definition).
    #[builder(into)]
    #[serde(rename = "containerName")]
    pub r#container_name: String,
    /// The port on the container to associate with the load balancer. Defaults to `0` if not specified.
    /// 
    /// > **Note:** Specifying multiple `load_balancer` configurations is still not supported by AWS for ECS task set.
    #[builder(into)]
    #[serde(rename = "containerPort")]
    pub r#container_port: Option<i32>,
    /// The name of the ELB (Classic) to associate with the service.
    #[builder(into)]
    #[serde(rename = "loadBalancerName")]
    pub r#load_balancer_name: Option<String>,
    /// The ARN of the Load Balancer target group to associate with the service.
    #[builder(into)]
    #[serde(rename = "targetGroupArn")]
    pub r#target_group_arn: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TaskSetLoadBalancer {
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
                "container_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container_name,
                )
                .await,
            );
            map.insert(
                "container_port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container_port,
                )
                .await,
            );
            map.insert(
                "load_balancer_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#load_balancer_name,
                )
                .await,
            );
            map.insert(
                "target_group_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_group_arn,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TaskSetLoadBalancer {
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
                    r#container_name: {
                        let field_value = match fields_map.get("container_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_port: {
                        let field_value = match fields_map.get("container_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_name: {
                        let field_value = match fields_map.get("load_balancer_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_balancer_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_group_arn: {
                        let field_value = match fields_map.get("target_group_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_group_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeploymentGroupBlueGreenDeploymentConfigDeploymentReadyOption {
    /// When to reroute traffic from an original environment to a replacement environment in a blue/green deployment.
    /// * `CONTINUE_DEPLOYMENT`: Register new instances with the load balancer immediately after the new application revision is installed on the instances in the replacement environment.
    /// * `STOP_DEPLOYMENT`: Do not register new instances with load balancer unless traffic is rerouted manually. If traffic is not rerouted manually before the end of the specified wait period, the deployment status is changed to Stopped.
    #[builder(into)]
    #[serde(rename = "actionOnTimeout")]
    pub r#action_on_timeout: Option<String>,
    /// The number of minutes to wait before the status of a blue/green deployment changed to Stopped if rerouting is not started manually. Applies only to the `STOP_DEPLOYMENT` option for `action_on_timeout`.
    #[builder(into)]
    #[serde(rename = "waitTimeInMinutes")]
    pub r#wait_time_in_minutes: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DeploymentGroupBlueGreenDeploymentConfigDeploymentReadyOption {
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
                "action_on_timeout".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#action_on_timeout,
                )
                .await,
            );
            map.insert(
                "wait_time_in_minutes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#wait_time_in_minutes,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DeploymentGroupBlueGreenDeploymentConfigDeploymentReadyOption {
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
                    r#action_on_timeout: {
                        let field_value = match fields_map.get("action_on_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'action_on_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#wait_time_in_minutes: {
                        let field_value = match fields_map.get("wait_time_in_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'wait_time_in_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

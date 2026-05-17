#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GroupInstanceRefresh {
    /// Override default parameters for Instance Refresh.
    #[builder(into)]
    #[serde(rename = "preferences")]
    pub r#preferences: Option<Box<super::super::types::autoscaling::GroupInstanceRefreshPreferences>>,
    /// Strategy to use for instance refresh. The only allowed value is `Rolling`. See [StartInstanceRefresh Action](https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_StartInstanceRefresh.html#API_StartInstanceRefresh_RequestParameters) for more information.
    #[builder(into)]
    #[serde(rename = "strategy")]
    pub r#strategy: String,
    /// Set of additional property names that will trigger an Instance Refresh. A refresh will always be triggered by a change in any of `launch_configuration`, `launch_template`, or `mixed_instances_policy`.
    /// 
    /// > **NOTE:** A refresh is started when any of the following Auto Scaling Group properties change: `launch_configuration`, `launch_template`, `mixed_instances_policy`. Additional properties can be specified in the `triggers` property of `instance_refresh`.
    /// 
    /// > **NOTE:** A refresh will not start when `version = "$Latest"` is configured in the `launch_template` block. To trigger the instance refresh when a launch template is changed, configure `version` to use the `latest_version` attribute of the `aws.ec2.LaunchTemplate` resource.
    /// 
    /// > **NOTE:** Auto Scaling Groups support up to one active instance refresh at a time. When this resource is updated, any existing refresh is cancelled.
    /// 
    /// > **NOTE:** Depending on health check settings and group size, an instance refresh may take a long time or fail. This resource does not wait for the instance refresh to complete.
    #[builder(into)]
    #[serde(rename = "triggers")]
    pub r#triggers: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GroupInstanceRefresh {
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
                "preferences".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preferences,
                )
                .await,
            );
            map.insert(
                "strategy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#strategy,
                )
                .await,
            );
            map.insert(
                "triggers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#triggers,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GroupInstanceRefresh {
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
                    r#preferences: {
                        let field_value = match fields_map.get("preferences") {
                            Some(value) => value,
                            None => bail!("Missing field 'preferences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#strategy: {
                        let field_value = match fields_map.get("strategy") {
                            Some(value) => value,
                            None => bail!("Missing field 'strategy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#triggers: {
                        let field_value = match fields_map.get("triggers") {
                            Some(value) => value,
                            None => bail!("Missing field 'triggers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

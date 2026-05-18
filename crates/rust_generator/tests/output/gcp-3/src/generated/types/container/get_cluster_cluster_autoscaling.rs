#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterClusterAutoscaling {
    /// Contains defaults for a node pool created by NAP.
    #[builder(into)]
    #[serde(rename = "autoProvisioningDefaults")]
    pub r#auto_provisioning_defaults: Vec<super::super::types::container::GetClusterClusterAutoscalingAutoProvisioningDefault>,
    /// The list of Google Compute Engine zones in which the NodePool's nodes can be created by NAP.
    #[builder(into)]
    #[serde(rename = "autoProvisioningLocations")]
    pub r#auto_provisioning_locations: Vec<String>,
    /// Configuration options for the Autoscaling profile feature, which lets you choose whether the cluster autoscaler should optimize for resource utilization or resource availability when deciding to remove nodes from a cluster. Can be BALANCED or OPTIMIZE_UTILIZATION. Defaults to BALANCED.
    #[builder(into)]
    #[serde(rename = "autoscalingProfile")]
    pub r#autoscaling_profile: String,
    /// Whether node auto-provisioning is enabled. Resource limits for cpu and memory must be defined to enable node auto-provisioning.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Global constraints for machine resources in the cluster. Configuring the cpu and memory types is required if node auto-provisioning is enabled. These limits will apply to node pool autoscaling in addition to node auto-provisioning.
    #[builder(into)]
    #[serde(rename = "resourceLimits")]
    pub r#resource_limits: Vec<super::super::types::container::GetClusterClusterAutoscalingResourceLimit>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterClusterAutoscaling {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "auto_provisioning_defaults",
                    &self.r#auto_provisioning_defaults,
                ),
                to_pulumi_object_field(
                    "auto_provisioning_locations",
                    &self.r#auto_provisioning_locations,
                ),
                to_pulumi_object_field(
                    "autoscaling_profile",
                    &self.r#autoscaling_profile,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "resource_limits",
                    &self.r#resource_limits,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterClusterAutoscaling {
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
                    r#auto_provisioning_defaults: {
                        let field_value = match fields_map.get("auto_provisioning_defaults") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_provisioning_defaults' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auto_provisioning_locations: {
                        let field_value = match fields_map.get("auto_provisioning_locations") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_provisioning_locations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#autoscaling_profile: {
                        let field_value = match fields_map.get("autoscaling_profile") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoscaling_profile' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_limits: {
                        let field_value = match fields_map.get("resource_limits") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_limits' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

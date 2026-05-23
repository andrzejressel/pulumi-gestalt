#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureMembershipPolicycontrollerPolicyControllerHubConfigDeploymentConfig {
    /// The name of the component. One of `admission` `audit` or `mutation`
    #[builder(into)]
    pub r#component_name: String,
    /// Container resource requirements.
    #[builder(into)]
    pub r#container_resources: Option<Box<super::super::types::gkehub::FeatureMembershipPolicycontrollerPolicyControllerHubConfigDeploymentConfigContainerResources>>,
    /// Pod affinity configuration. Possible values: AFFINITY_UNSPECIFIED, NO_AFFINITY, ANTI_AFFINITY
    #[builder(into)]
    pub r#pod_affinity: Option<String>,
    /// Pod tolerations of node taints.
    #[builder(into)]
    pub r#pod_tolerations: Option<Vec<super::super::types::gkehub::FeatureMembershipPolicycontrollerPolicyControllerHubConfigDeploymentConfigPodToleration>>,
    /// Pod replica count.
    #[builder(into)]
    pub r#replica_count: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FeatureMembershipPolicycontrollerPolicyControllerHubConfigDeploymentConfig {
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
                    "componentName",
                    &self.r#component_name,
                ),
                to_pulumi_object_field(
                    "containerResources",
                    &self.r#container_resources,
                ),
                to_pulumi_object_field(
                    "podAffinity",
                    &self.r#pod_affinity,
                ),
                to_pulumi_object_field(
                    "podTolerations",
                    &self.r#pod_tolerations,
                ),
                to_pulumi_object_field(
                    "replicaCount",
                    &self.r#replica_count,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FeatureMembershipPolicycontrollerPolicyControllerHubConfigDeploymentConfig {
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
                    r#component_name: {
                        let field_value = match fields_map.get("componentName") {
                            Some(value) => value,
                            None => bail!("Missing field 'componentName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_resources: {
                        let field_value = match fields_map.get("containerResources") {
                            Some(value) => value,
                            None => bail!("Missing field 'containerResources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pod_affinity: {
                        let field_value = match fields_map.get("podAffinity") {
                            Some(value) => value,
                            None => bail!("Missing field 'podAffinity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pod_tolerations: {
                        let field_value = match fields_map.get("podTolerations") {
                            Some(value) => value,
                            None => bail!("Missing field 'podTolerations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replica_count: {
                        let field_value = match fields_map.get("replicaCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'replicaCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

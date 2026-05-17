#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfig {
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "component")]
    pub r#component: String,
    /// Container resource requirements.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "containerResources")]
    pub r#container_resources: Option<Box<super::super::types::gkehub::FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfigContainerResources>>,
    /// Pod affinity configuration.
    /// Possible values are: `AFFINITY_UNSPECIFIED`, `NO_AFFINITY`, `ANTI_AFFINITY`.
    #[builder(into)]
    #[serde(rename = "podAffinity")]
    pub r#pod_affinity: Option<String>,
    /// Pod tolerations of node taints.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "podTolerations")]
    pub r#pod_tolerations: Option<Vec<super::super::types::gkehub::FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfigPodToleration>>,
    /// Pod replica count.
    #[builder(into)]
    #[serde(rename = "replicaCount")]
    pub r#replica_count: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfig {
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
                "component".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#component,
                )
                .await,
            );
            map.insert(
                "container_resources".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container_resources,
                )
                .await,
            );
            map.insert(
                "pod_affinity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pod_affinity,
                )
                .await,
            );
            map.insert(
                "pod_tolerations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pod_tolerations,
                )
                .await,
            );
            map.insert(
                "replica_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#replica_count,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfig {
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
                    r#component: {
                        let field_value = match fields_map.get("component") {
                            Some(value) => value,
                            None => bail!("Missing field 'component' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_resources: {
                        let field_value = match fields_map.get("container_resources") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_resources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pod_affinity: {
                        let field_value = match fields_map.get("pod_affinity") {
                            Some(value) => value,
                            None => bail!("Missing field 'pod_affinity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pod_tolerations: {
                        let field_value = match fields_map.get("pod_tolerations") {
                            Some(value) => value,
                            None => bail!("Missing field 'pod_tolerations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replica_count: {
                        let field_value = match fields_map.get("replica_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'replica_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterClusterConfigPreemptibleWorkerConfigInstanceFlexibilityPolicy {
    /// List of instance selection options that the group will use when creating new VMs.
    #[builder(into)]
    #[serde(rename = "instanceSelectionLists")]
    pub r#instance_selection_lists: Option<Vec<super::super::types::dataproc::ClusterClusterConfigPreemptibleWorkerConfigInstanceFlexibilityPolicyInstanceSelectionList>>,
    /// A list of instance selection results in the group.
    #[builder(into)]
    #[serde(rename = "instanceSelectionResults")]
    pub r#instance_selection_results: Option<Vec<super::super::types::dataproc::ClusterClusterConfigPreemptibleWorkerConfigInstanceFlexibilityPolicyInstanceSelectionResult>>,
    /// Defines how Dataproc should create VMs with a mixture of provisioning models.
    #[builder(into)]
    #[serde(rename = "provisioningModelMix")]
    pub r#provisioning_model_mix: Option<Box<super::super::types::dataproc::ClusterClusterConfigPreemptibleWorkerConfigInstanceFlexibilityPolicyProvisioningModelMix>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterClusterConfigPreemptibleWorkerConfigInstanceFlexibilityPolicy {
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
                "instance_selection_lists".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_selection_lists,
                )
                .await,
            );
            map.insert(
                "instance_selection_results".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_selection_results,
                )
                .await,
            );
            map.insert(
                "provisioning_model_mix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#provisioning_model_mix,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterClusterConfigPreemptibleWorkerConfigInstanceFlexibilityPolicy {
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
                    r#instance_selection_lists: {
                        let field_value = match fields_map.get("instance_selection_lists") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_selection_lists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_selection_results: {
                        let field_value = match fields_map.get("instance_selection_results") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_selection_results' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioning_model_mix: {
                        let field_value = match fields_map.get("provisioning_model_mix") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioning_model_mix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

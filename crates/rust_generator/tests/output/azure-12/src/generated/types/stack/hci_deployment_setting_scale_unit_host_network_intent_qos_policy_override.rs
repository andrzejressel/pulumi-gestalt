#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HciDeploymentSettingScaleUnitHostNetworkIntentQosPolicyOverride {
    /// Specifies the percentage of the allocated storage traffic bandwidth. This parameter should only be modified based on your OEM guidance. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "bandwidthPercentageSmb")]
    pub r#bandwidth_percentage_smb: Option<String>,
    /// Specifies the Cluster traffic priority. This parameter should only be modified based on your OEM guidance. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "priorityValue8021ActionCluster")]
    pub r#priority_value_8021_action_cluster: Option<String>,
    /// Specifies the Priority Flow Control where Data Center Bridging (DCB) is used. This parameter should only be modified based on your OEM guidance. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "priorityValue8021ActionSmb")]
    pub r#priority_value_8021_action_smb: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HciDeploymentSettingScaleUnitHostNetworkIntentQosPolicyOverride {
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
                "bandwidth_percentage_smb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bandwidth_percentage_smb,
                )
                .await,
            );
            map.insert(
                "priority_value_8021_action_cluster".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#priority_value_8021_action_cluster,
                )
                .await,
            );
            map.insert(
                "priority_value_8021_action_smb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#priority_value_8021_action_smb,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HciDeploymentSettingScaleUnitHostNetworkIntentQosPolicyOverride {
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
                    r#bandwidth_percentage_smb: {
                        let field_value = match fields_map.get("bandwidth_percentage_smb") {
                            Some(value) => value,
                            None => bail!("Missing field 'bandwidth_percentage_smb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#priority_value_8021_action_cluster: {
                        let field_value = match fields_map.get("priority_value_8021_action_cluster") {
                            Some(value) => value,
                            None => bail!("Missing field 'priority_value_8021_action_cluster' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#priority_value_8021_action_smb: {
                        let field_value = match fields_map.get("priority_value_8021_action_smb") {
                            Some(value) => value,
                            None => bail!("Missing field 'priority_value_8021_action_smb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

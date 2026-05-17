#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecurityPolicyAdaptiveProtectionConfig {
    /// Configuration for [Automatically deploy Adaptive Protection suggested rules](https://cloud.google.com/armor/docs/adaptive-protection-auto-deploy?hl=en). Structure is documented below.
    /// 
    /// <a name="nested_layer_7_ddos_defense_config"></a>The `layer_7_ddos_defense_config` block supports:
    #[builder(into)]
    #[serde(rename = "autoDeployConfig")]
    pub r#auto_deploy_config: Option<Box<super::super::types::compute::SecurityPolicyAdaptiveProtectionConfigAutoDeployConfig>>,
    /// Configuration for [Google Cloud Armor Adaptive Protection Layer 7 DDoS Defense](https://cloud.google.com/armor/docs/adaptive-protection-overview?hl=en). Structure is documented below.
    #[builder(into)]
    #[serde(rename = "layer7DdosDefenseConfig")]
    pub r#layer_7_ddos_defense_config: Option<Box<super::super::types::compute::SecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SecurityPolicyAdaptiveProtectionConfig {
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
                "auto_deploy_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auto_deploy_config,
                )
                .await,
            );
            map.insert(
                "layer_7_ddos_defense_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#layer_7_ddos_defense_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SecurityPolicyAdaptiveProtectionConfig {
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
                    r#auto_deploy_config: {
                        let field_value = match fields_map.get("auto_deploy_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_deploy_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#layer_7_ddos_defense_config: {
                        let field_value = match fields_map.get("layer_7_ddos_defense_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'layer_7_ddos_defense_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

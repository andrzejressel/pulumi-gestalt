#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterClusterAutoscalingAutoProvisioningDefaultsUpgradeSettings {
    /// Settings for blue-green upgrade strategy. To be specified when strategy is set to BLUE_GREEN. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "blueGreenSettings")]
    pub r#blue_green_settings: Option<Box<super::super::types::container::ClusterClusterAutoscalingAutoProvisioningDefaultsUpgradeSettingsBlueGreenSettings>>,
    /// The maximum number of nodes that can be created beyond the current size of the node pool during the upgrade process. To be used when strategy is set to SURGE. Default is 0.
    #[builder(into)]
    #[serde(rename = "maxSurge")]
    pub r#max_surge: Option<i32>,
    /// The maximum number of nodes that can be simultaneously unavailable during the upgrade process. To be used when strategy is set to SURGE. Default is 0.
    #[builder(into)]
    #[serde(rename = "maxUnavailable")]
    pub r#max_unavailable: Option<i32>,
    /// Strategy used for node pool update. Strategy can only be one of BLUE_GREEN or SURGE. The default is value is SURGE.
    #[builder(into)]
    #[serde(rename = "strategy")]
    pub r#strategy: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterClusterAutoscalingAutoProvisioningDefaultsUpgradeSettings {
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
                "blue_green_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#blue_green_settings,
                )
                .await,
            );
            map.insert(
                "max_surge".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_surge,
                )
                .await,
            );
            map.insert(
                "max_unavailable".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_unavailable,
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

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterClusterAutoscalingAutoProvisioningDefaultsUpgradeSettings {
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
                    r#blue_green_settings: {
                        let field_value = match fields_map.get("blue_green_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'blue_green_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_surge: {
                        let field_value = match fields_map.get("max_surge") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_surge' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_unavailable: {
                        let field_value = match fields_map.get("max_unavailable") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_unavailable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

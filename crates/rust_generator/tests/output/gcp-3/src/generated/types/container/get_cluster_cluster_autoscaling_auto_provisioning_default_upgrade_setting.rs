#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterClusterAutoscalingAutoProvisioningDefaultUpgradeSetting {
    /// Settings for blue-green upgrade strategy.
    #[builder(into)]
    #[serde(rename = "blueGreenSettings")]
    pub r#blue_green_settings: Vec<super::super::types::container::GetClusterClusterAutoscalingAutoProvisioningDefaultUpgradeSettingBlueGreenSetting>,
    /// The maximum number of nodes that can be created beyond the current size of the node pool during the upgrade process.
    #[builder(into)]
    #[serde(rename = "maxSurge")]
    pub r#max_surge: i32,
    /// The maximum number of nodes that can be simultaneously unavailable during the upgrade process.
    #[builder(into)]
    #[serde(rename = "maxUnavailable")]
    pub r#max_unavailable: i32,
    /// Update strategy of the node pool.
    #[builder(into)]
    #[serde(rename = "strategy")]
    pub r#strategy: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterClusterAutoscalingAutoProvisioningDefaultUpgradeSetting {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "blue_green_settings",
                    &self.r#blue_green_settings,
                ),
                to_pulumi_object_field(
                    "max_surge",
                    &self.r#max_surge,
                ),
                to_pulumi_object_field(
                    "max_unavailable",
                    &self.r#max_unavailable,
                ),
                to_pulumi_object_field(
                    "strategy",
                    &self.r#strategy,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterClusterAutoscalingAutoProvisioningDefaultUpgradeSetting {
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

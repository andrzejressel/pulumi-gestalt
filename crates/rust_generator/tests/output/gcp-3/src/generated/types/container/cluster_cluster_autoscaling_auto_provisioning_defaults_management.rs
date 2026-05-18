#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterClusterAutoscalingAutoProvisioningDefaultsManagement {
    /// Specifies whether the node auto-repair is enabled for the node pool. If enabled, the nodes in this node pool will be monitored and, if they fail health checks too many times, an automatic repair action will be triggered.
    /// 
    /// This block also contains several computed attributes, documented below.
    #[builder(into)]
    #[serde(rename = "autoRepair")]
    pub r#auto_repair: Option<bool>,
    /// Specifies whether node auto-upgrade is enabled for the node pool. If enabled, node auto-upgrade helps keep the nodes in your node pool up to date with the latest release version of Kubernetes.
    #[builder(into)]
    #[serde(rename = "autoUpgrade")]
    pub r#auto_upgrade: Option<bool>,
    /// Specifies the [Auto Upgrade knobs](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1beta1/NodeManagement#AutoUpgradeOptions) for the node pool.
    #[builder(into)]
    #[serde(rename = "upgradeOptions")]
    pub r#upgrade_options: Option<Vec<super::super::types::container::ClusterClusterAutoscalingAutoProvisioningDefaultsManagementUpgradeOption>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterClusterAutoscalingAutoProvisioningDefaultsManagement {
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
                    "auto_repair",
                    &self.r#auto_repair,
                ),
                to_pulumi_object_field(
                    "auto_upgrade",
                    &self.r#auto_upgrade,
                ),
                to_pulumi_object_field(
                    "upgrade_options",
                    &self.r#upgrade_options,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterClusterAutoscalingAutoProvisioningDefaultsManagement {
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
                    r#auto_repair: {
                        let field_value = match fields_map.get("auto_repair") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_repair' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auto_upgrade: {
                        let field_value = match fields_map.get("auto_upgrade") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_upgrade' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upgrade_options: {
                        let field_value = match fields_map.get("upgrade_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'upgrade_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceUpgradeHistory {
    /// Optional. Action. Rolloback or Upgrade.
    #[builder(into)]
    pub r#action: Option<String>,
    /// Optional. The container image before this instance upgrade.
    #[builder(into)]
    pub r#container_image: Option<String>,
    /// An RFC3339 timestamp in UTC time. This in the format of yyyy-MM-ddTHH:mm:ss.SSSZ.
    /// The milliseconds portion (".SSS") is optional.
    #[builder(into)]
    pub r#create_time: Option<String>,
    /// Optional. The framework of this workbench instance.
    #[builder(into)]
    pub r#framework: Option<String>,
    /// Optional. The snapshot of the boot disk of this workbench instance before upgrade.
    #[builder(into)]
    pub r#snapshot: Option<String>,
    /// (Output)
    /// Output only. The state of this instance upgrade history entry.
    #[builder(into)]
    pub r#state: Option<String>,
    /// Optional. Target VM Version, like m63.
    #[builder(into)]
    pub r#target_version: Option<String>,
    /// Optional. The version of the workbench instance before this upgrade.
    #[builder(into)]
    pub r#version: Option<String>,
    /// Optional. The VM image before this instance upgrade.
    #[builder(into)]
    pub r#vm_image: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceUpgradeHistory {
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
                    "action",
                    &self.r#action,
                ),
                to_pulumi_object_field(
                    "containerImage",
                    &self.r#container_image,
                ),
                to_pulumi_object_field(
                    "createTime",
                    &self.r#create_time,
                ),
                to_pulumi_object_field(
                    "framework",
                    &self.r#framework,
                ),
                to_pulumi_object_field(
                    "snapshot",
                    &self.r#snapshot,
                ),
                to_pulumi_object_field(
                    "state",
                    &self.r#state,
                ),
                to_pulumi_object_field(
                    "targetVersion",
                    &self.r#target_version,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
                to_pulumi_object_field(
                    "vmImage",
                    &self.r#vm_image,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceUpgradeHistory {
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
                    r#action: {
                        let field_value = match fields_map.get("action") {
                            Some(value) => value,
                            None => bail!("Missing field 'action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_image: {
                        let field_value = match fields_map.get("containerImage") {
                            Some(value) => value,
                            None => bail!("Missing field 'containerImage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_time: {
                        let field_value = match fields_map.get("createTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'createTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#framework: {
                        let field_value = match fields_map.get("framework") {
                            Some(value) => value,
                            None => bail!("Missing field 'framework' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snapshot: {
                        let field_value = match fields_map.get("snapshot") {
                            Some(value) => value,
                            None => bail!("Missing field 'snapshot' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_version: {
                        let field_value = match fields_map.get("targetVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'targetVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_image: {
                        let field_value = match fields_map.get("vmImage") {
                            Some(value) => value,
                            None => bail!("Missing field 'vmImage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

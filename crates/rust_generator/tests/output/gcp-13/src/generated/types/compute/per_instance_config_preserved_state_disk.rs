#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PerInstanceConfigPreservedStateDisk {
    /// A value that prescribes what should happen to the stateful disk when the VM instance is deleted.
    /// The available options are `NEVER` and `ON_PERMANENT_INSTANCE_DELETION`.
    /// `NEVER` - detach the disk when the VM is deleted, but do not delete the disk.
    /// `ON_PERMANENT_INSTANCE_DELETION` will delete the stateful disk when the VM is permanently
    /// deleted from the instance group.
    /// Default value is `NEVER`.
    /// Possible values are: `NEVER`, `ON_PERMANENT_INSTANCE_DELETION`.
    #[builder(into)]
    #[serde(rename = "deleteRule")]
    pub r#delete_rule: Option<String>,
    /// A unique device name that is reflected into the /dev/ tree of a Linux operating system running within the instance.
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: String,
    /// The mode of the disk.
    /// Default value is `READ_WRITE`.
    /// Possible values are: `READ_ONLY`, `READ_WRITE`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    /// The URI of an existing persistent disk to attach under the specified device-name in the format
    /// `projects/project-id/zones/zone/disks/disk-name`.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PerInstanceConfigPreservedStateDisk {
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
                "delete_rule".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#delete_rule,
                )
                .await,
            );
            map.insert(
                "device_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#device_name,
                )
                .await,
            );
            map.insert(
                "mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mode,
                )
                .await,
            );
            map.insert(
                "source".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PerInstanceConfigPreservedStateDisk {
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
                    r#delete_rule: {
                        let field_value = match fields_map.get("delete_rule") {
                            Some(value) => value,
                            None => bail!("Missing field 'delete_rule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#device_name: {
                        let field_value = match fields_map.get("device_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mode: {
                        let field_value = match fields_map.get("mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source: {
                        let field_value = match fields_map.get("source") {
                            Some(value) => value,
                            None => bail!("Missing field 'source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuntimeVirtualMachine {
    /// (Output)
    /// The unique identifier of the Managed Compute Engine instance.
    #[builder(into)]
    #[serde(rename = "instanceId")]
    pub r#instance_id: Option<String>,
    /// (Output)
    /// The user-friendly name of the Managed Compute Engine instance.
    #[builder(into)]
    #[serde(rename = "instanceName")]
    pub r#instance_name: Option<String>,
    /// Virtual Machine configuration settings.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "virtualMachineConfig")]
    pub r#virtual_machine_config: Option<Box<super::super::types::notebooks::RuntimeVirtualMachineVirtualMachineConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RuntimeVirtualMachine {
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
                "instance_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_id,
                )
                .await,
            );
            map.insert(
                "instance_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_name,
                )
                .await,
            );
            map.insert(
                "virtual_machine_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#virtual_machine_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RuntimeVirtualMachine {
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
                    r#instance_id: {
                        let field_value = match fields_map.get("instance_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_name: {
                        let field_value = match fields_map.get("instance_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_machine_config: {
                        let field_value = match fields_map.get("virtual_machine_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_machine_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

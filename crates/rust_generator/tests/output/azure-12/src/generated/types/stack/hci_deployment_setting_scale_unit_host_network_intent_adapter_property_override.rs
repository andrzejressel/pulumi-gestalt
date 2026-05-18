#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HciDeploymentSettingScaleUnitHostNetworkIntentAdapterPropertyOverride {
    /// The jumbo frame size of the adapter. This parameter should only be modified based on your OEM guidance. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "jumboPacket")]
    pub r#jumbo_packet: Option<String>,
    /// The network direct of the adapter. This parameter should only be modified based on your OEM guidance. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "networkDirect")]
    pub r#network_direct: Option<String>,
    /// The network direct technology of the adapter. This parameter should only be modified based on your OEM guidance. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "networkDirectTechnology")]
    pub r#network_direct_technology: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HciDeploymentSettingScaleUnitHostNetworkIntentAdapterPropertyOverride {
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
                    "jumbo_packet",
                    &self.r#jumbo_packet,
                ),
                to_pulumi_object_field(
                    "network_direct",
                    &self.r#network_direct,
                ),
                to_pulumi_object_field(
                    "network_direct_technology",
                    &self.r#network_direct_technology,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HciDeploymentSettingScaleUnitHostNetworkIntentAdapterPropertyOverride {
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
                    r#jumbo_packet: {
                        let field_value = match fields_map.get("jumbo_packet") {
                            Some(value) => value,
                            None => bail!("Missing field 'jumbo_packet' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_direct: {
                        let field_value = match fields_map.get("network_direct") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_direct' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_direct_technology: {
                        let field_value = match fields_map.get("network_direct_technology") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_direct_technology' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

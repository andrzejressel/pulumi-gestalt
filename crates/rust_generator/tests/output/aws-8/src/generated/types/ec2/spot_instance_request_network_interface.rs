#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpotInstanceRequestNetworkInterface {
    /// Whether or not to delete the network interface on instance termination. Defaults to `false`. Currently, the only valid value is `false`, as this is only supported when creating new network interfaces when launching an instance.
    #[builder(into)]
    #[serde(rename = "deleteOnTermination")]
    pub r#delete_on_termination: Option<bool>,
    /// Integer index of the network interface attachment. Limited by instance type.
    #[builder(into)]
    #[serde(rename = "deviceIndex")]
    pub r#device_index: i32,
    /// Integer index of the network card. Limited by instance type. The default index is `0`.
    #[builder(into)]
    #[serde(rename = "networkCardIndex")]
    pub r#network_card_index: Option<i32>,
    /// ID of the network interface to attach.
    #[builder(into)]
    #[serde(rename = "networkInterfaceId")]
    pub r#network_interface_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpotInstanceRequestNetworkInterface {
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
                    "deleteOnTermination",
                    &self.r#delete_on_termination,
                ),
                to_pulumi_object_field(
                    "deviceIndex",
                    &self.r#device_index,
                ),
                to_pulumi_object_field(
                    "networkCardIndex",
                    &self.r#network_card_index,
                ),
                to_pulumi_object_field(
                    "networkInterfaceId",
                    &self.r#network_interface_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SpotInstanceRequestNetworkInterface {
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
                    r#delete_on_termination: {
                        let field_value = match fields_map.get("deleteOnTermination") {
                            Some(value) => value,
                            None => bail!("Missing field 'deleteOnTermination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#device_index: {
                        let field_value = match fields_map.get("deviceIndex") {
                            Some(value) => value,
                            None => bail!("Missing field 'deviceIndex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_card_index: {
                        let field_value = match fields_map.get("networkCardIndex") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkCardIndex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_interface_id: {
                        let field_value = match fields_map.get("networkInterfaceId") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkInterfaceId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

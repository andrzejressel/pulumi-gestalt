#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServicesCommunicationsGatewayServiceLocation {
    /// Specifies the allowed source IP address or CIDR ranges for media.
    #[builder(into)]
    #[serde(rename = "allowedMediaSourceAddressPrefixes")]
    pub r#allowed_media_source_address_prefixes: Option<Vec<String>>,
    /// Specifies the allowed source IP address or CIDR ranges for signaling.
    #[builder(into)]
    #[serde(rename = "allowedSignalingSourceAddressPrefixes")]
    pub r#allowed_signaling_source_address_prefixes: Option<Vec<String>>,
    /// IP address to use to contact the ESRP from this region.
    /// 
    /// !> **NOTE:** The `esrp_addresses` must be specified for each `service_location` when the`e911_type` is set to `DirectToEsrp`.  The `esrp_addresses` must not be specified for each `service_location` when the`e911_type` is set to `Standard`.
    #[builder(into)]
    #[serde(rename = "esrpAddresses")]
    pub r#esrp_addresses: Option<Vec<String>>,
    /// Specifies the region in which the resources needed for Teams Calling will be deployed.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
    /// IP address to use to contact the operator network from this region.
    #[builder(into)]
    #[serde(rename = "operatorAddresses")]
    pub r#operator_addresses: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServicesCommunicationsGatewayServiceLocation {
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
                    "allowedMediaSourceAddressPrefixes",
                    &self.r#allowed_media_source_address_prefixes,
                ),
                to_pulumi_object_field(
                    "allowedSignalingSourceAddressPrefixes",
                    &self.r#allowed_signaling_source_address_prefixes,
                ),
                to_pulumi_object_field(
                    "esrpAddresses",
                    &self.r#esrp_addresses,
                ),
                to_pulumi_object_field(
                    "location",
                    &self.r#location,
                ),
                to_pulumi_object_field(
                    "operatorAddresses",
                    &self.r#operator_addresses,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServicesCommunicationsGatewayServiceLocation {
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
                    r#allowed_media_source_address_prefixes: {
                        let field_value = match fields_map.get("allowedMediaSourceAddressPrefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowedMediaSourceAddressPrefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_signaling_source_address_prefixes: {
                        let field_value = match fields_map.get("allowedSignalingSourceAddressPrefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowedSignalingSourceAddressPrefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#esrp_addresses: {
                        let field_value = match fields_map.get("esrpAddresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'esrpAddresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#location: {
                        let field_value = match fields_map.get("location") {
                            Some(value) => value,
                            None => bail!("Missing field 'location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#operator_addresses: {
                        let field_value = match fields_map.get("operatorAddresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'operatorAddresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

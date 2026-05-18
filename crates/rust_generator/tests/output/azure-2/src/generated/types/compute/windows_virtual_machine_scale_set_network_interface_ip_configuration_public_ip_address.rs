#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsVirtualMachineScaleSetNetworkInterfaceIpConfigurationPublicIpAddress {
    /// The Prefix which should be used for the Domain Name Label for each Virtual Machine Instance. Azure concatenates the Domain Name Label and Virtual Machine Index to create a unique Domain Name Label for each Virtual Machine.
    #[builder(into)]
    #[serde(rename = "domainNameLabel")]
    pub r#domain_name_label: Option<String>,
    /// The Idle Timeout in Minutes for the Public IP Address. Possible values are in the range `4` to `32`.
    #[builder(into)]
    #[serde(rename = "idleTimeoutInMinutes")]
    pub r#idle_timeout_in_minutes: Option<i32>,
    /// One or more `ip_tag` blocks as defined above. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "ipTags")]
    pub r#ip_tags: Option<Vec<super::super::types::compute::WindowsVirtualMachineScaleSetNetworkInterfaceIpConfigurationPublicIpAddressIpTag>>,
    /// The Name of the Public IP Address Configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The ID of the Public IP Address Prefix from where Public IP Addresses should be allocated. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** This functionality is in Preview and must be opted into via `az feature register --namespace Microsoft.Network --name AllowBringYourOwnPublicIpAddress` and then `az provider register -n Microsoft.Network`.
    #[builder(into)]
    #[serde(rename = "publicIpPrefixId")]
    pub r#public_ip_prefix_id: Option<String>,
    /// The Internet Protocol Version which should be used for this public IP address. Possible values are `IPv4` and `IPv6`. Defaults to `IPv4`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WindowsVirtualMachineScaleSetNetworkInterfaceIpConfigurationPublicIpAddress {
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
                    "domain_name_label",
                    &self.r#domain_name_label,
                ),
                to_pulumi_object_field(
                    "idle_timeout_in_minutes",
                    &self.r#idle_timeout_in_minutes,
                ),
                to_pulumi_object_field(
                    "ip_tags",
                    &self.r#ip_tags,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "public_ip_prefix_id",
                    &self.r#public_ip_prefix_id,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WindowsVirtualMachineScaleSetNetworkInterfaceIpConfigurationPublicIpAddress {
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
                    r#domain_name_label: {
                        let field_value = match fields_map.get("domain_name_label") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_name_label' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#idle_timeout_in_minutes: {
                        let field_value = match fields_map.get("idle_timeout_in_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'idle_timeout_in_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_tags: {
                        let field_value = match fields_map.get("ip_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip_prefix_id: {
                        let field_value = match fields_map.get("public_ip_prefix_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_ip_prefix_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

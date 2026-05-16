#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VpcAttachmentOptions {
    /// Indicates whether appliance mode is supported.
    /// If enabled, traffic flow between a source and destination use the same Availability Zone for the VPC attachment for the lifetime of that flow.
    /// If the VPC attachment is pending acceptance, changing this value will recreate the resource.
    #[builder(into)]
    #[serde(rename = "applianceModeSupport")]
    pub r#appliance_mode_support: Option<bool>,
    /// Indicates whether IPv6 is supported.
    /// If the VPC attachment is pending acceptance, changing this value will recreate the resource.
    #[builder(into)]
    #[serde(rename = "ipv6Support")]
    pub r#ipv_6_support: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VpcAttachmentOptions {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("appliance_mode_support".to_string(), self.r#appliance_mode_support.to_pulumi_value().await);
            map.insert("ipv_6_support".to_string(), self.r#ipv_6_support.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VpcAttachmentOptions {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#appliance_mode_support: {
                        let field_value = match fields_map.get("appliance_mode_support") {
                            Some(value) => value,
                            None => bail!("Missing field 'appliance_mode_support' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_support: {
                        let field_value = match fields_map.get("ipv_6_support") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_6_support' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

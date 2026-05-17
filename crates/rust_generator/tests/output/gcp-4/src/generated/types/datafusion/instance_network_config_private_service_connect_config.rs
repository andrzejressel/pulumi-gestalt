#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceNetworkConfigPrivateServiceConnectConfig {
    /// (Output)
    /// Output only. The CIDR block to which the CDF instance can't route traffic to in the consumer project VPC.
    /// The size of this block is /25. The format of this field is governed by RFC 4632.
    #[builder(into)]
    #[serde(rename = "effectiveUnreachableCidrBlock")]
    pub r#effective_unreachable_cidr_block: Option<String>,
    /// Optional. The reference to the network attachment used to establish private connectivity.
    /// It will be of the form projects/{project-id}/regions/{region}/networkAttachments/{network-attachment-id}.
    /// This is required only when using connection type PRIVATE_SERVICE_CONNECT_INTERFACES.
    #[builder(into)]
    #[serde(rename = "networkAttachment")]
    pub r#network_attachment: Option<String>,
    /// Optional. Input only. The CIDR block to which the CDF instance can't route traffic to in the consumer project VPC.
    /// The size of this block should be at least /25. This range should not overlap with the primary address range of any subnetwork used by the network attachment.
    /// This range can be used for other purposes in the consumer VPC as long as there is no requirement for CDF to reach destinations using these addresses.
    /// If this value is not provided, the server chooses a non RFC 1918 address range. The format of this field is governed by RFC 4632.
    #[builder(into)]
    #[serde(rename = "unreachableCidrBlock")]
    pub r#unreachable_cidr_block: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceNetworkConfigPrivateServiceConnectConfig {
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
                "effective_unreachable_cidr_block".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#effective_unreachable_cidr_block,
                )
                .await,
            );
            map.insert(
                "network_attachment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network_attachment,
                )
                .await,
            );
            map.insert(
                "unreachable_cidr_block".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#unreachable_cidr_block,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceNetworkConfigPrivateServiceConnectConfig {
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
                    r#effective_unreachable_cidr_block: {
                        let field_value = match fields_map.get("effective_unreachable_cidr_block") {
                            Some(value) => value,
                            None => bail!("Missing field 'effective_unreachable_cidr_block' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_attachment: {
                        let field_value = match fields_map.get("network_attachment") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_attachment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unreachable_cidr_block: {
                        let field_value = match fields_map.get("unreachable_cidr_block") {
                            Some(value) => value,
                            None => bail!("Missing field 'unreachable_cidr_block' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

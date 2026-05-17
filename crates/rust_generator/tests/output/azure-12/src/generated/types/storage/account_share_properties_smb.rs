#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountSharePropertiesSmb {
    /// A set of SMB authentication methods. Possible values are `NTLMv2`, and `Kerberos`.
    #[builder(into)]
    #[serde(rename = "authenticationTypes")]
    pub r#authentication_types: Option<Vec<String>>,
    /// A set of SMB channel encryption. Possible values are `AES-128-CCM`, `AES-128-GCM`, and `AES-256-GCM`.
    #[builder(into)]
    #[serde(rename = "channelEncryptionTypes")]
    pub r#channel_encryption_types: Option<Vec<String>>,
    /// A set of Kerberos ticket encryption. Possible values are `RC4-HMAC`, and `AES-256`.
    #[builder(into)]
    #[serde(rename = "kerberosTicketEncryptionTypes")]
    pub r#kerberos_ticket_encryption_types: Option<Vec<String>>,
    /// Indicates whether multichannel is enabled. Defaults to `false`. This is only supported on Premium storage accounts.
    #[builder(into)]
    #[serde(rename = "multichannelEnabled")]
    pub r#multichannel_enabled: Option<bool>,
    /// A set of SMB protocol versions. Possible values are `SMB2.1`, `SMB3.0`, and `SMB3.1.1`.
    #[builder(into)]
    #[serde(rename = "versions")]
    pub r#versions: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccountSharePropertiesSmb {
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
                "authentication_types".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authentication_types,
                )
                .await,
            );
            map.insert(
                "channel_encryption_types".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#channel_encryption_types,
                )
                .await,
            );
            map.insert(
                "kerberos_ticket_encryption_types".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kerberos_ticket_encryption_types,
                )
                .await,
            );
            map.insert(
                "multichannel_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#multichannel_enabled,
                )
                .await,
            );
            map.insert(
                "versions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#versions,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccountSharePropertiesSmb {
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
                    r#authentication_types: {
                        let field_value = match fields_map.get("authentication_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'authentication_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#channel_encryption_types: {
                        let field_value = match fields_map.get("channel_encryption_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'channel_encryption_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_ticket_encryption_types: {
                        let field_value = match fields_map.get("kerberos_ticket_encryption_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos_ticket_encryption_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#multichannel_enabled: {
                        let field_value = match fields_map.get("multichannel_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'multichannel_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#versions: {
                        let field_value = match fields_map.get("versions") {
                            Some(value) => value,
                            None => bail!("Missing field 'versions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountSharePropertiesSmb {
    /// A set of SMB authentication methods. Possible values are `NTLMv2`, and `Kerberos`.
    #[builder(into)]
    pub r#authentication_types: Option<Vec<String>>,
    /// A set of SMB channel encryption. Possible values are `AES-128-CCM`, `AES-128-GCM`, and `AES-256-GCM`.
    #[builder(into)]
    pub r#channel_encryption_types: Option<Vec<String>>,
    /// A set of Kerberos ticket encryption. Possible values are `RC4-HMAC`, and `AES-256`.
    #[builder(into)]
    pub r#kerberos_ticket_encryption_types: Option<Vec<String>>,
    /// Indicates whether multichannel is enabled. Defaults to `false`. This is only supported on Premium storage accounts.
    #[builder(into)]
    pub r#multichannel_enabled: Option<bool>,
    /// A set of SMB protocol versions. Possible values are `SMB2.1`, `SMB3.0`, and `SMB3.1.1`.
    #[builder(into)]
    pub r#versions: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccountSharePropertiesSmb {
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
                    "authenticationTypes",
                    &self.r#authentication_types,
                ),
                to_pulumi_object_field(
                    "channelEncryptionTypes",
                    &self.r#channel_encryption_types,
                ),
                to_pulumi_object_field(
                    "kerberosTicketEncryptionTypes",
                    &self.r#kerberos_ticket_encryption_types,
                ),
                to_pulumi_object_field(
                    "multichannelEnabled",
                    &self.r#multichannel_enabled,
                ),
                to_pulumi_object_field(
                    "versions",
                    &self.r#versions,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("authenticationTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'authenticationTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#channel_encryption_types: {
                        let field_value = match fields_map.get("channelEncryptionTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'channelEncryptionTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_ticket_encryption_types: {
                        let field_value = match fields_map.get("kerberosTicketEncryptionTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberosTicketEncryptionTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#multichannel_enabled: {
                        let field_value = match fields_map.get("multichannelEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'multichannelEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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

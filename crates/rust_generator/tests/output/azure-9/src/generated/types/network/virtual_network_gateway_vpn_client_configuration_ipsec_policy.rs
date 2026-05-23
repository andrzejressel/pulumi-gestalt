#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualNetworkGatewayVpnClientConfigurationIpsecPolicy {
    /// The DH Group, used in IKE Phase 1. Possible values are `DHGroup1`, `DHGroup2`, `DHGroup14`, `DHGroup24`, `DHGroup2048`, `ECP256`, `ECP384` and `None`.
    #[builder(into)]
    pub r#dh_group: String,
    /// The IKE encryption algorithm, used for IKE Phase 2. Possible values are `AES128`, `AES192`, `AES256`, `DES`, `DES3`, `GCMAES128` and `GCMAES256`.
    #[builder(into)]
    pub r#ike_encryption: String,
    /// The IKE encryption integrity algorithm, used for IKE Phase 2. Possible values are `GCMAES128`, `GCMAES256`, `MD5`, `SHA1`, `SHA256` and `SHA384`.
    #[builder(into)]
    pub r#ike_integrity: String,
    /// The IPSec encryption algorithm, used for IKE phase 1. Possible values are `AES128`, `AES192`, `AES256`, `DES`, `DES3`, `GCMAES128`, `GCMAES192`, `GCMAES256` and `None`.
    #[builder(into)]
    pub r#ipsec_encryption: String,
    /// The IPSec integrity algorithm, used for IKE phase 1. Possible values are `GCMAES128`, `GCMAES192`, `GCMAES256`, `MD5`, `SHA1` and `SHA256`.
    #[builder(into)]
    pub r#ipsec_integrity: String,
    /// The Pfs Group, used in IKE Phase 2. Possible values are `ECP256`, `ECP384`, `PFS1`, `PFS2`, `PFS14`, `PFS24`, `PFS2048`, `PFSMM` and `None`.
    #[builder(into)]
    pub r#pfs_group: String,
    /// The IPSec Security Association payload size in KB for a Site-to-Site VPN tunnel. Possible values are between `1024` and `2147483647`.
    #[builder(into)]
    pub r#sa_data_size_in_kilobytes: i32,
    /// The IPSec Security Association lifetime in seconds for a Site-to-Site VPN tunnel. Possible values are between `300` and `172799`.
    #[builder(into)]
    pub r#sa_lifetime_in_seconds: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualNetworkGatewayVpnClientConfigurationIpsecPolicy {
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
                    "dhGroup",
                    &self.r#dh_group,
                ),
                to_pulumi_object_field(
                    "ikeEncryption",
                    &self.r#ike_encryption,
                ),
                to_pulumi_object_field(
                    "ikeIntegrity",
                    &self.r#ike_integrity,
                ),
                to_pulumi_object_field(
                    "ipsecEncryption",
                    &self.r#ipsec_encryption,
                ),
                to_pulumi_object_field(
                    "ipsecIntegrity",
                    &self.r#ipsec_integrity,
                ),
                to_pulumi_object_field(
                    "pfsGroup",
                    &self.r#pfs_group,
                ),
                to_pulumi_object_field(
                    "saDataSizeInKilobytes",
                    &self.r#sa_data_size_in_kilobytes,
                ),
                to_pulumi_object_field(
                    "saLifetimeInSeconds",
                    &self.r#sa_lifetime_in_seconds,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualNetworkGatewayVpnClientConfigurationIpsecPolicy {
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
                    r#dh_group: {
                        let field_value = match fields_map.get("dhGroup") {
                            Some(value) => value,
                            None => bail!("Missing field 'dhGroup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ike_encryption: {
                        let field_value = match fields_map.get("ikeEncryption") {
                            Some(value) => value,
                            None => bail!("Missing field 'ikeEncryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ike_integrity: {
                        let field_value = match fields_map.get("ikeIntegrity") {
                            Some(value) => value,
                            None => bail!("Missing field 'ikeIntegrity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipsec_encryption: {
                        let field_value = match fields_map.get("ipsecEncryption") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipsecEncryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipsec_integrity: {
                        let field_value = match fields_map.get("ipsecIntegrity") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipsecIntegrity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pfs_group: {
                        let field_value = match fields_map.get("pfsGroup") {
                            Some(value) => value,
                            None => bail!("Missing field 'pfsGroup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sa_data_size_in_kilobytes: {
                        let field_value = match fields_map.get("saDataSizeInKilobytes") {
                            Some(value) => value,
                            None => bail!("Missing field 'saDataSizeInKilobytes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sa_lifetime_in_seconds: {
                        let field_value = match fields_map.get("saLifetimeInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'saLifetimeInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

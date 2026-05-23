#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VpnGatewayConnectionVpnLinkIpsecPolicy {
    /// The DH Group used in IKE Phase 1 for initial SA. Possible values are `None`, `DHGroup1`, `DHGroup2`, `DHGroup14`, `DHGroup24`, `DHGroup2048`, `ECP256`, `ECP384`.
    #[builder(into)]
    pub r#dh_group: String,
    /// The IPSec encryption algorithm (IKE phase 1). Possible values are `AES128`, `AES192`, `AES256`, `DES`, `DES3`, `GCMAES128`, `GCMAES192`, `GCMAES256`, `None`.
    #[builder(into)]
    pub r#encryption_algorithm: String,
    /// The IKE encryption algorithm (IKE phase 2). Possible values are `DES`, `DES3`, `AES128`, `AES192`, `AES256`, `GCMAES128`, `GCMAES256`.
    #[builder(into)]
    pub r#ike_encryption_algorithm: String,
    /// The IKE integrity algorithm (IKE phase 2). Possible values are `MD5`, `SHA1`, `SHA256`, `SHA384`, `GCMAES128`, `GCMAES256`.
    #[builder(into)]
    pub r#ike_integrity_algorithm: String,
    /// The IPSec integrity algorithm (IKE phase 1). Possible values are `MD5`, `SHA1`, `SHA256`, `GCMAES128`, `GCMAES192`, `GCMAES256`.
    #[builder(into)]
    pub r#integrity_algorithm: String,
    /// The Pfs Group used in IKE Phase 2 for the new child SA. Possible values are `None`, `PFS1`, `PFS2`, `PFS14`, `PFS24`, `PFS2048`, `PFSMM`, `ECP256`, `ECP384`.
    #[builder(into)]
    pub r#pfs_group: String,
    /// The IPSec Security Association (also called Quick Mode or Phase 2 SA) payload size in KB for the site to site VPN tunnel.
    #[builder(into)]
    pub r#sa_data_size_kb: i32,
    /// The IPSec Security Association (also called Quick Mode or Phase 2 SA) lifetime in seconds for the site to site VPN tunnel.
    #[builder(into)]
    pub r#sa_lifetime_sec: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VpnGatewayConnectionVpnLinkIpsecPolicy {
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
                    "encryptionAlgorithm",
                    &self.r#encryption_algorithm,
                ),
                to_pulumi_object_field(
                    "ikeEncryptionAlgorithm",
                    &self.r#ike_encryption_algorithm,
                ),
                to_pulumi_object_field(
                    "ikeIntegrityAlgorithm",
                    &self.r#ike_integrity_algorithm,
                ),
                to_pulumi_object_field(
                    "integrityAlgorithm",
                    &self.r#integrity_algorithm,
                ),
                to_pulumi_object_field(
                    "pfsGroup",
                    &self.r#pfs_group,
                ),
                to_pulumi_object_field(
                    "saDataSizeKb",
                    &self.r#sa_data_size_kb,
                ),
                to_pulumi_object_field(
                    "saLifetimeSec",
                    &self.r#sa_lifetime_sec,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VpnGatewayConnectionVpnLinkIpsecPolicy {
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
                    r#encryption_algorithm: {
                        let field_value = match fields_map.get("encryptionAlgorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryptionAlgorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ike_encryption_algorithm: {
                        let field_value = match fields_map.get("ikeEncryptionAlgorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'ikeEncryptionAlgorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ike_integrity_algorithm: {
                        let field_value = match fields_map.get("ikeIntegrityAlgorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'ikeIntegrityAlgorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#integrity_algorithm: {
                        let field_value = match fields_map.get("integrityAlgorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'integrityAlgorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#sa_data_size_kb: {
                        let field_value = match fields_map.get("saDataSizeKb") {
                            Some(value) => value,
                            None => bail!("Missing field 'saDataSizeKb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sa_lifetime_sec: {
                        let field_value = match fields_map.get("saLifetimeSec") {
                            Some(value) => value,
                            None => bail!("Missing field 'saLifetimeSec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

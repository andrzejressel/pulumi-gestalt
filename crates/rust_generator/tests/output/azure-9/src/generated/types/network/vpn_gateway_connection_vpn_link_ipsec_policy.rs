#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VpnGatewayConnectionVpnLinkIpsecPolicy {
    /// The DH Group used in IKE Phase 1 for initial SA. Possible values are `None`, `DHGroup1`, `DHGroup2`, `DHGroup14`, `DHGroup24`, `DHGroup2048`, `ECP256`, `ECP384`.
    #[builder(into)]
    #[serde(rename = "dhGroup")]
    pub r#dh_group: String,
    /// The IPSec encryption algorithm (IKE phase 1). Possible values are `AES128`, `AES192`, `AES256`, `DES`, `DES3`, `GCMAES128`, `GCMAES192`, `GCMAES256`, `None`.
    #[builder(into)]
    #[serde(rename = "encryptionAlgorithm")]
    pub r#encryption_algorithm: String,
    /// The IKE encryption algorithm (IKE phase 2). Possible values are `DES`, `DES3`, `AES128`, `AES192`, `AES256`, `GCMAES128`, `GCMAES256`.
    #[builder(into)]
    #[serde(rename = "ikeEncryptionAlgorithm")]
    pub r#ike_encryption_algorithm: String,
    /// The IKE integrity algorithm (IKE phase 2). Possible values are `MD5`, `SHA1`, `SHA256`, `SHA384`, `GCMAES128`, `GCMAES256`.
    #[builder(into)]
    #[serde(rename = "ikeIntegrityAlgorithm")]
    pub r#ike_integrity_algorithm: String,
    /// The IPSec integrity algorithm (IKE phase 1). Possible values are `MD5`, `SHA1`, `SHA256`, `GCMAES128`, `GCMAES192`, `GCMAES256`.
    #[builder(into)]
    #[serde(rename = "integrityAlgorithm")]
    pub r#integrity_algorithm: String,
    /// The Pfs Group used in IKE Phase 2 for the new child SA. Possible values are `None`, `PFS1`, `PFS2`, `PFS14`, `PFS24`, `PFS2048`, `PFSMM`, `ECP256`, `ECP384`.
    #[builder(into)]
    #[serde(rename = "pfsGroup")]
    pub r#pfs_group: String,
    /// The IPSec Security Association (also called Quick Mode or Phase 2 SA) payload size in KB for the site to site VPN tunnel.
    #[builder(into)]
    #[serde(rename = "saDataSizeKb")]
    pub r#sa_data_size_kb: i32,
    /// The IPSec Security Association (also called Quick Mode or Phase 2 SA) lifetime in seconds for the site to site VPN tunnel.
    #[builder(into)]
    #[serde(rename = "saLifetimeSec")]
    pub r#sa_lifetime_sec: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VpnGatewayConnectionVpnLinkIpsecPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "dh_group",
                    &self.r#dh_group,
                ),
                to_pulumi_object_field(
                    "encryption_algorithm",
                    &self.r#encryption_algorithm,
                ),
                to_pulumi_object_field(
                    "ike_encryption_algorithm",
                    &self.r#ike_encryption_algorithm,
                ),
                to_pulumi_object_field(
                    "ike_integrity_algorithm",
                    &self.r#ike_integrity_algorithm,
                ),
                to_pulumi_object_field(
                    "integrity_algorithm",
                    &self.r#integrity_algorithm,
                ),
                to_pulumi_object_field(
                    "pfs_group",
                    &self.r#pfs_group,
                ),
                to_pulumi_object_field(
                    "sa_data_size_kb",
                    &self.r#sa_data_size_kb,
                ),
                to_pulumi_object_field(
                    "sa_lifetime_sec",
                    &self.r#sa_lifetime_sec,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
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
                        let field_value = match fields_map.get("dh_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'dh_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_algorithm: {
                        let field_value = match fields_map.get("encryption_algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ike_encryption_algorithm: {
                        let field_value = match fields_map.get("ike_encryption_algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'ike_encryption_algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ike_integrity_algorithm: {
                        let field_value = match fields_map.get("ike_integrity_algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'ike_integrity_algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#integrity_algorithm: {
                        let field_value = match fields_map.get("integrity_algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'integrity_algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pfs_group: {
                        let field_value = match fields_map.get("pfs_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'pfs_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sa_data_size_kb: {
                        let field_value = match fields_map.get("sa_data_size_kb") {
                            Some(value) => value,
                            None => bail!("Missing field 'sa_data_size_kb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sa_lifetime_sec: {
                        let field_value = match fields_map.get("sa_lifetime_sec") {
                            Some(value) => value,
                            None => bail!("Missing field 'sa_lifetime_sec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

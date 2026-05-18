#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetGatewayConnectionIpsecPolicy {
    /// The DH group used in IKE phase 1 for initial SA. Valid
    /// options are `DHGroup1`, `DHGroup14`, `DHGroup2`, `DHGroup2048`, `DHGroup24`,
    /// `ECP256`, `ECP384`, or `None`.
    #[builder(into)]
    #[serde(rename = "dhGroup")]
    pub r#dh_group: String,
    /// The IKE encryption algorithm. Valid
    /// options are `AES128`, `AES192`, `AES256`, `DES`, or `DES3`.
    #[builder(into)]
    #[serde(rename = "ikeEncryption")]
    pub r#ike_encryption: String,
    /// The IKE integrity algorithm. Valid
    /// options are `MD5`, `SHA1`, `SHA256`, or `SHA384`.
    #[builder(into)]
    #[serde(rename = "ikeIntegrity")]
    pub r#ike_integrity: String,
    /// The IPSec encryption algorithm. Valid
    /// options are `AES128`, `AES192`, `AES256`, `DES`, `DES3`, `GCMAES128`, `GCMAES192`, `GCMAES256`, or `None`.
    #[builder(into)]
    #[serde(rename = "ipsecEncryption")]
    pub r#ipsec_encryption: String,
    /// The IPSec integrity algorithm. Valid
    /// options are `GCMAES128`, `GCMAES192`, `GCMAES256`, `MD5`, `SHA1`, or `SHA256`.
    #[builder(into)]
    #[serde(rename = "ipsecIntegrity")]
    pub r#ipsec_integrity: String,
    /// The DH group used in IKE phase 2 for new child SA.
    /// Valid options are `ECP256`, `ECP384`, `PFS1`, `PFS2`, `PFS2048`, `PFS24`,
    /// or `None`.
    #[builder(into)]
    #[serde(rename = "pfsGroup")]
    pub r#pfs_group: String,
    /// The IPSec SA payload size in KB. Must be at least
    /// `1024` KB.
    #[builder(into)]
    #[serde(rename = "saDatasize")]
    pub r#sa_datasize: i32,
    /// The IPSec SA lifetime in seconds. Must be at least
    /// `300` seconds.
    #[builder(into)]
    #[serde(rename = "saLifetime")]
    pub r#sa_lifetime: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetGatewayConnectionIpsecPolicy {
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
                    "dh_group",
                    &self.r#dh_group,
                ),
                to_pulumi_object_field(
                    "ike_encryption",
                    &self.r#ike_encryption,
                ),
                to_pulumi_object_field(
                    "ike_integrity",
                    &self.r#ike_integrity,
                ),
                to_pulumi_object_field(
                    "ipsec_encryption",
                    &self.r#ipsec_encryption,
                ),
                to_pulumi_object_field(
                    "ipsec_integrity",
                    &self.r#ipsec_integrity,
                ),
                to_pulumi_object_field(
                    "pfs_group",
                    &self.r#pfs_group,
                ),
                to_pulumi_object_field(
                    "sa_datasize",
                    &self.r#sa_datasize,
                ),
                to_pulumi_object_field(
                    "sa_lifetime",
                    &self.r#sa_lifetime,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetGatewayConnectionIpsecPolicy {
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
                    r#ike_encryption: {
                        let field_value = match fields_map.get("ike_encryption") {
                            Some(value) => value,
                            None => bail!("Missing field 'ike_encryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ike_integrity: {
                        let field_value = match fields_map.get("ike_integrity") {
                            Some(value) => value,
                            None => bail!("Missing field 'ike_integrity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipsec_encryption: {
                        let field_value = match fields_map.get("ipsec_encryption") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipsec_encryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipsec_integrity: {
                        let field_value = match fields_map.get("ipsec_integrity") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipsec_integrity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#sa_datasize: {
                        let field_value = match fields_map.get("sa_datasize") {
                            Some(value) => value,
                            None => bail!("Missing field 'sa_datasize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sa_lifetime: {
                        let field_value = match fields_map.get("sa_lifetime") {
                            Some(value) => value,
                            None => bail!("Missing field 'sa_lifetime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

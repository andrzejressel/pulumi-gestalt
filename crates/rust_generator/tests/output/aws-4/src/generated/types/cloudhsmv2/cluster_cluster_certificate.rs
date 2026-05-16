#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterClusterCertificate {
    /// The HSM hardware certificate issued (signed) by AWS CloudHSM.
    #[builder(into)]
    #[serde(rename = "awsHardwareCertificate")]
    pub r#aws_hardware_certificate: Option<String>,
    /// The cluster certificate issued (signed) by the issuing certificate authority (CA) of the cluster's owner.
    #[builder(into)]
    #[serde(rename = "clusterCertificate")]
    pub r#cluster_certificate: Option<String>,
    /// The certificate signing request (CSR). Available only in `UNINITIALIZED` state after an HSM instance is added to the cluster.
    #[builder(into)]
    #[serde(rename = "clusterCsr")]
    pub r#cluster_csr: Option<String>,
    /// The HSM certificate issued (signed) by the HSM hardware.
    #[builder(into)]
    #[serde(rename = "hsmCertificate")]
    pub r#hsm_certificate: Option<String>,
    /// The HSM hardware certificate issued (signed) by the hardware manufacturer.
    #[builder(into)]
    #[serde(rename = "manufacturerHardwareCertificate")]
    pub r#manufacturer_hardware_certificate: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterClusterCertificate {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("aws_hardware_certificate".to_string(), self.r#aws_hardware_certificate.to_pulumi_value().await);
            map.insert("cluster_certificate".to_string(), self.r#cluster_certificate.to_pulumi_value().await);
            map.insert("cluster_csr".to_string(), self.r#cluster_csr.to_pulumi_value().await);
            map.insert("hsm_certificate".to_string(), self.r#hsm_certificate.to_pulumi_value().await);
            map.insert("manufacturer_hardware_certificate".to_string(), self.r#manufacturer_hardware_certificate.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterClusterCertificate {
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
                    r#aws_hardware_certificate: {
                        let field_value = match fields_map.get("aws_hardware_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_hardware_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#cluster_certificate: {
                        let field_value = match fields_map.get("cluster_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#cluster_csr: {
                        let field_value = match fields_map.get("cluster_csr") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_csr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#hsm_certificate: {
                        let field_value = match fields_map.get("hsm_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'hsm_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#manufacturer_hardware_certificate: {
                        let field_value = match fields_map.get("manufacturer_hardware_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'manufacturer_hardware_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

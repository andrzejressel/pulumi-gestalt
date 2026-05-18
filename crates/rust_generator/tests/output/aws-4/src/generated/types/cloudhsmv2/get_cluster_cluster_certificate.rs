#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterClusterCertificate {
    /// The HSM hardware certificate issued (signed) by AWS CloudHSM.
    #[builder(into)]
    #[serde(rename = "awsHardwareCertificate")]
    pub r#aws_hardware_certificate: String,
    /// The cluster certificate issued (signed) by the issuing certificate authority (CA) of the cluster's owner.
    #[builder(into)]
    #[serde(rename = "clusterCertificate")]
    pub r#cluster_certificate: String,
    /// The certificate signing request (CSR). Available only in UNINITIALIZED state.
    #[builder(into)]
    #[serde(rename = "clusterCsr")]
    pub r#cluster_csr: String,
    /// The HSM certificate issued (signed) by the HSM hardware.
    #[builder(into)]
    #[serde(rename = "hsmCertificate")]
    pub r#hsm_certificate: String,
    /// The HSM hardware certificate issued (signed) by the hardware manufacturer.
    /// The number of available cluster certificates may vary depending on state of the cluster.
    #[builder(into)]
    #[serde(rename = "manufacturerHardwareCertificate")]
    pub r#manufacturer_hardware_certificate: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterClusterCertificate {
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
                    "aws_hardware_certificate",
                    &self.r#aws_hardware_certificate,
                ),
                to_pulumi_object_field(
                    "cluster_certificate",
                    &self.r#cluster_certificate,
                ),
                to_pulumi_object_field(
                    "cluster_csr",
                    &self.r#cluster_csr,
                ),
                to_pulumi_object_field(
                    "hsm_certificate",
                    &self.r#hsm_certificate,
                ),
                to_pulumi_object_field(
                    "manufacturer_hardware_certificate",
                    &self.r#manufacturer_hardware_certificate,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterClusterCertificate {
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
                    r#aws_hardware_certificate: {
                        let field_value = match fields_map.get("aws_hardware_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_hardware_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_certificate: {
                        let field_value = match fields_map.get("cluster_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_csr: {
                        let field_value = match fields_map.get("cluster_csr") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_csr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hsm_certificate: {
                        let field_value = match fields_map.get("hsm_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'hsm_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#manufacturer_hardware_certificate: {
                        let field_value = match fields_map.get("manufacturer_hardware_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'manufacturer_hardware_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

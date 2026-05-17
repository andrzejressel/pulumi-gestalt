#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CryptoKeyVersionAttestationCertChains {
    /// Cavium certificate chain corresponding to the attestation.
    #[builder(into)]
    #[serde(rename = "caviumCerts")]
    pub r#cavium_certs: Option<Vec<String>>,
    /// Google card certificate chain corresponding to the attestation.
    #[builder(into)]
    #[serde(rename = "googleCardCerts")]
    pub r#google_card_certs: Option<Vec<String>>,
    /// Google partition certificate chain corresponding to the attestation.
    #[builder(into)]
    #[serde(rename = "googlePartitionCerts")]
    pub r#google_partition_certs: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CryptoKeyVersionAttestationCertChains {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "cavium_certs",
                    &self.r#cavium_certs,
                ),
                to_pulumi_object_field(
                    "google_card_certs",
                    &self.r#google_card_certs,
                ),
                to_pulumi_object_field(
                    "google_partition_certs",
                    &self.r#google_partition_certs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CryptoKeyVersionAttestationCertChains {
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
                    r#cavium_certs: {
                        let field_value = match fields_map.get("cavium_certs") {
                            Some(value) => value,
                            None => bail!("Missing field 'cavium_certs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#google_card_certs: {
                        let field_value = match fields_map.get("google_card_certs") {
                            Some(value) => value,
                            None => bail!("Missing field 'google_card_certs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#google_partition_certs: {
                        let field_value = match fields_map.get("google_partition_certs") {
                            Some(value) => value,
                            None => bail!("Missing field 'google_partition_certs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

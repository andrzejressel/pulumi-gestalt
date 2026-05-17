#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OccurenceAttestation {
    /// The serialized payload that is verified by one or
    /// more signatures. A base64-encoded string.
    #[builder(into)]
    #[serde(rename = "serializedPayload")]
    pub r#serialized_payload: String,
    /// One or more signatures over serializedPayload.
    /// Verifier implementations should consider this attestation
    /// message verified if at least one signature verifies
    /// serializedPayload. See Signature in common.proto for more
    /// details on signature structure and verification.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "signatures")]
    pub r#signatures: Vec<super::super::types::containeranalysis::OccurenceAttestationSignature>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OccurenceAttestation {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "serialized_payload".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#serialized_payload,
                )
                .await,
            );
            map.insert(
                "signatures".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#signatures,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OccurenceAttestation {
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
                    r#serialized_payload: {
                        let field_value = match fields_map.get("serialized_payload") {
                            Some(value) => value,
                            None => bail!("Missing field 'serialized_payload' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#signatures: {
                        let field_value = match fields_map.get("signatures") {
                            Some(value) => value,
                            None => bail!("Missing field 'signatures' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

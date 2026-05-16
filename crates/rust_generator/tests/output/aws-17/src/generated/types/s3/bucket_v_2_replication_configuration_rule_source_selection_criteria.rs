#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketV2ReplicationConfigurationRuleSourceSelectionCriteria {
    /// Match SSE-KMS encrypted objects (documented below). If specified, `replica_kms_key_id`
    /// in `destination` must be specified as well.
    #[builder(into)]
    #[serde(rename = "sseKmsEncryptedObjects")]
    pub r#sse_kms_encrypted_objects: Option<Vec<super::super::types::s3::BucketV2ReplicationConfigurationRuleSourceSelectionCriteriaSseKmsEncryptedObject>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BucketV2ReplicationConfigurationRuleSourceSelectionCriteria {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("sse_kms_encrypted_objects".to_string(), self.r#sse_kms_encrypted_objects.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BucketV2ReplicationConfigurationRuleSourceSelectionCriteria {
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
                    r#sse_kms_encrypted_objects: {
                        let field_value = match fields_map.get("sse_kms_encrypted_objects") {
                            Some(value) => value,
                            None => bail!("Missing field 'sse_kms_encrypted_objects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::s3::BucketV2ReplicationConfigurationRuleSourceSelectionCriteriaSseKmsEncryptedObject>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatabaseCmekConfig {
    /// (Output)
    /// Currently in-use KMS key versions (https://cloud.google.com/kms/docs/resource-hierarchy#key_versions).
    /// During key rotation (https://cloud.google.com/kms/docs/key-rotation), there can be
    /// multiple in-use key versions.
    /// The expected format is
    /// `projects/{project_id}/locations/{kms_location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}/cryptoKeyVersions/{key_version}`.
    #[builder(into)]
    #[serde(rename = "activeKeyVersions")]
    pub r#active_key_versions: Option<Vec<String>>,
    /// The resource ID of a Cloud KMS key. If set, the database created will
    /// be a Customer-managed Encryption Key (CMEK) database encrypted with
    /// this key. This feature is allowlist only in initial launch.
    /// Only keys in the same location as this database are allowed to be used
    /// for encryption. For Firestore's nam5 multi-region, this corresponds to Cloud KMS
    /// multi-region us. For Firestore's eur3 multi-region, this corresponds to
    /// Cloud KMS multi-region europe. See https://cloud.google.com/kms/docs/locations.
    /// This value should be the KMS key resource ID in the format of
    /// `projects/{project_id}/locations/{kms_location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`.
    /// How to retrive this resource ID is listed at
    /// https://cloud.google.com/kms/docs/getting-resource-ids#getting_the_id_for_a_key_and_version.
    #[builder(into)]
    #[serde(rename = "kmsKeyName")]
    pub r#kms_key_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DatabaseCmekConfig {
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
                "active_key_versions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#active_key_versions,
                )
                .await,
            );
            map.insert(
                "kms_key_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kms_key_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DatabaseCmekConfig {
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
                    r#active_key_versions: {
                        let field_value = match fields_map.get("active_key_versions") {
                            Some(value) => value,
                            None => bail!("Missing field 'active_key_versions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_key_name: {
                        let field_value = match fields_map.get("kms_key_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

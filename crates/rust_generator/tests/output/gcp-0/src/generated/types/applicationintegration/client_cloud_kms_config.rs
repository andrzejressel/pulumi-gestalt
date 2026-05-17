#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClientCloudKmsConfig {
    /// A Cloud KMS key is a named object containing one or more key versions, along
    /// with metadata for the key. A key exists on exactly one key ring tied to a
    /// specific location.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
    /// Each version of a key contains key material used for encryption or signing.
    /// A key's version is represented by an integer, starting at 1. To decrypt data
    /// or verify a signature, you must use the same key version that was used to
    /// encrypt or sign the data.
    #[builder(into)]
    #[serde(rename = "keyVersion")]
    pub r#key_version: Option<String>,
    /// Location name of the key ring, e.g. "us-west1".
    #[builder(into)]
    #[serde(rename = "kmsLocation")]
    pub r#kms_location: String,
    /// The Google Cloud project id of the project where the kms key stored. If empty,
    /// the kms key is stored at the same project as customer's project and ecrypted
    /// with CMEK, otherwise, the kms key is stored in the tenant project and
    /// encrypted with GMEK.
    #[builder(into)]
    #[serde(rename = "kmsProjectId")]
    pub r#kms_project_id: Option<String>,
    /// A key ring organizes keys in a specific Google Cloud location and allows you to
    /// manage access control on groups of keys. A key ring's name does not need to be
    /// unique across a Google Cloud project, but must be unique within a given location.
    #[builder(into)]
    #[serde(rename = "kmsRing")]
    pub r#kms_ring: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClientCloudKmsConfig {
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
                "key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key,
                )
                .await,
            );
            map.insert(
                "key_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_version,
                )
                .await,
            );
            map.insert(
                "kms_location".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kms_location,
                )
                .await,
            );
            map.insert(
                "kms_project_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kms_project_id,
                )
                .await,
            );
            map.insert(
                "kms_ring".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kms_ring,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClientCloudKmsConfig {
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
                    r#key: {
                        let field_value = match fields_map.get("key") {
                            Some(value) => value,
                            None => bail!("Missing field 'key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_version: {
                        let field_value = match fields_map.get("key_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_location: {
                        let field_value = match fields_map.get("kms_location") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_project_id: {
                        let field_value = match fields_map.get("kms_project_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_project_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_ring: {
                        let field_value = match fields_map.get("kms_ring") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_ring' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

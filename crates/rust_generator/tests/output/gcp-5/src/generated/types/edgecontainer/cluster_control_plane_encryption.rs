#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterControlPlaneEncryption {
    /// The Cloud KMS CryptoKey e.g.
    /// projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}
    /// to use for protecting control plane disks. If not specified, a
    /// Google-managed key will be used instead.
    #[builder(into)]
    #[serde(rename = "kmsKey")]
    pub r#kms_key: Option<String>,
    /// (Output)
    /// The Cloud KMS CryptoKeyVersion currently in use for protecting control
    /// plane disks. Only applicable if kms_key is set.
    #[builder(into)]
    #[serde(rename = "kmsKeyActiveVersion")]
    pub r#kms_key_active_version: Option<String>,
    /// (Output)
    /// Availability of the Cloud KMS CryptoKey. If not `KEY_AVAILABLE`, then
    /// nodes may go offline as they cannot access their local data. This can be
    /// caused by a lack of permissions to use the key, or if the key is disabled
    /// or deleted.
    #[builder(into)]
    #[serde(rename = "kmsKeyState")]
    pub r#kms_key_state: Option<String>,
    /// (Output)
    /// Error status returned by Cloud KMS when using this key. This field may be
    /// populated only if `kms_key_state` is not `KMS_KEY_STATE_KEY_AVAILABLE`.
    /// If populated, this field contains the error status reported by Cloud KMS.
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_kms_status"></a>The `kms_status` block contains:
    #[builder(into)]
    #[serde(rename = "kmsStatuses")]
    pub r#kms_statuses: Option<Vec<super::super::types::edgecontainer::ClusterControlPlaneEncryptionKmsStatus>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterControlPlaneEncryption {
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
                    "kms_key",
                    &self.r#kms_key,
                ),
                to_pulumi_object_field(
                    "kms_key_active_version",
                    &self.r#kms_key_active_version,
                ),
                to_pulumi_object_field(
                    "kms_key_state",
                    &self.r#kms_key_state,
                ),
                to_pulumi_object_field(
                    "kms_statuses",
                    &self.r#kms_statuses,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterControlPlaneEncryption {
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
                    r#kms_key: {
                        let field_value = match fields_map.get("kms_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_key_active_version: {
                        let field_value = match fields_map.get("kms_key_active_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key_active_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_key_state: {
                        let field_value = match fields_map.get("kms_key_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_statuses: {
                        let field_value = match fields_map.get("kms_statuses") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_statuses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCryptoKeysKey {
    /// The resource name of the backend environment associated with all CryptoKeyVersions within this CryptoKey.
    /// The resource name is in the format "projects/*/locations/*/ekmConnections/*" and only applies to "EXTERNAL_VPC" keys.
    #[builder(into)]
    #[serde(rename = "cryptoKeyBackend")]
    pub r#crypto_key_backend: String,
    /// The period of time that versions of this key spend in the DESTROY_SCHEDULED state before transitioning to DESTROYED.
    /// If not specified at creation time, the default duration is 30 days.
    #[builder(into)]
    #[serde(rename = "destroyScheduledDuration")]
    pub r#destroy_scheduled_duration: String,
    #[builder(into)]
    #[serde(rename = "effectiveLabels")]
    pub r#effective_labels: std::collections::HashMap<String, String>,
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Whether this key may contain imported versions only.
    #[builder(into)]
    #[serde(rename = "importOnly")]
    pub r#import_only: bool,
    /// The policy used for Key Access Justifications Policy Enforcement. If this
    /// field is present and this key is enrolled in Key Access Justifications
    /// Policy Enforcement, the policy will be evaluated in encrypt, decrypt, and
    /// sign operations, and the operation will fail if rejected by the policy. The
    /// policy is defined by specifying zero or more allowed justification codes.
    /// https://cloud.google.com/assured-workloads/key-access-justifications/docs/justification-codes
    /// By default, this field is absent, and all justification codes are allowed.
    /// This field is currently in beta and is subject to change.
    #[builder(into)]
    #[serde(rename = "keyAccessJustificationsPolicies")]
    pub r#key_access_justifications_policies: Vec<super::super::types::kms::GetCryptoKeysKeyKeyAccessJustificationsPolicy>,
    /// The key ring that the keys belongs to. Format: 'projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}'.,
    #[builder(into)]
    #[serde(rename = "keyRing")]
    pub r#key_ring: Option<String>,
    /// Labels with user-defined metadata to apply to this resource.
    /// 
    /// 
    /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
    /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: std::collections::HashMap<String, String>,
    /// The resource name for the CryptoKey.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// A copy of the primary CryptoKeyVersion that will be used by cryptoKeys.encrypt when this CryptoKey is given in EncryptRequest.name.
    /// Keys with purpose ENCRYPT_DECRYPT may have a primary. For other keys, this field will be unset.
    #[builder(into)]
    #[serde(rename = "primaries")]
    pub r#primaries: Vec<super::super::types::kms::GetCryptoKeysKeyPrimary>,
    /// The combination of labels configured directly on the resource
    ///  and default labels configured on the provider.
    #[builder(into)]
    #[serde(rename = "pulumiLabels")]
    pub r#pulumi_labels: std::collections::HashMap<String, String>,
    /// The immutable purpose of this CryptoKey. See the
    /// [purpose reference](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys#CryptoKeyPurpose)
    /// for possible inputs.
    /// Default value is "ENCRYPT_DECRYPT".
    #[builder(into)]
    #[serde(rename = "purpose")]
    pub r#purpose: String,
    /// Every time this period passes, generate a new CryptoKeyVersion and set it as the primary.
    /// The first rotation will take place after the specified period. The rotation period has
    /// the format of a decimal number with up to 9 fractional digits, followed by the
    /// letter 's' (seconds). It must be greater than a day (ie, 86400).
    #[builder(into)]
    #[serde(rename = "rotationPeriod")]
    pub r#rotation_period: String,
    /// If set to true, the request will create a CryptoKey without any CryptoKeyVersions.
    /// You must use the 'google_kms_crypto_key_version' resource to create a new CryptoKeyVersion
    /// or 'google_kms_key_ring_import_job' resource to import the CryptoKeyVersion.
    #[builder(into)]
    #[serde(rename = "skipInitialVersionCreation")]
    pub r#skip_initial_version_creation: bool,
    /// A template describing settings for new crypto key versions.
    #[builder(into)]
    #[serde(rename = "versionTemplates")]
    pub r#version_templates: Vec<super::super::types::kms::GetCryptoKeysKeyVersionTemplate>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCryptoKeysKey {
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
                    "crypto_key_backend",
                    &self.r#crypto_key_backend,
                ),
                to_pulumi_object_field(
                    "destroy_scheduled_duration",
                    &self.r#destroy_scheduled_duration,
                ),
                to_pulumi_object_field(
                    "effective_labels",
                    &self.r#effective_labels,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "import_only",
                    &self.r#import_only,
                ),
                to_pulumi_object_field(
                    "key_access_justifications_policies",
                    &self.r#key_access_justifications_policies,
                ),
                to_pulumi_object_field(
                    "key_ring",
                    &self.r#key_ring,
                ),
                to_pulumi_object_field(
                    "labels",
                    &self.r#labels,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "primaries",
                    &self.r#primaries,
                ),
                to_pulumi_object_field(
                    "pulumi_labels",
                    &self.r#pulumi_labels,
                ),
                to_pulumi_object_field(
                    "purpose",
                    &self.r#purpose,
                ),
                to_pulumi_object_field(
                    "rotation_period",
                    &self.r#rotation_period,
                ),
                to_pulumi_object_field(
                    "skip_initial_version_creation",
                    &self.r#skip_initial_version_creation,
                ),
                to_pulumi_object_field(
                    "version_templates",
                    &self.r#version_templates,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCryptoKeysKey {
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
                    r#crypto_key_backend: {
                        let field_value = match fields_map.get("crypto_key_backend") {
                            Some(value) => value,
                            None => bail!("Missing field 'crypto_key_backend' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destroy_scheduled_duration: {
                        let field_value = match fields_map.get("destroy_scheduled_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'destroy_scheduled_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#effective_labels: {
                        let field_value = match fields_map.get("effective_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'effective_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#import_only: {
                        let field_value = match fields_map.get("import_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'import_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_access_justifications_policies: {
                        let field_value = match fields_map.get("key_access_justifications_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_access_justifications_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_ring: {
                        let field_value = match fields_map.get("key_ring") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_ring' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primaries: {
                        let field_value = match fields_map.get("primaries") {
                            Some(value) => value,
                            None => bail!("Missing field 'primaries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pulumi_labels: {
                        let field_value = match fields_map.get("pulumi_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'pulumi_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#purpose: {
                        let field_value = match fields_map.get("purpose") {
                            Some(value) => value,
                            None => bail!("Missing field 'purpose' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rotation_period: {
                        let field_value = match fields_map.get("rotation_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'rotation_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#skip_initial_version_creation: {
                        let field_value = match fields_map.get("skip_initial_version_creation") {
                            Some(value) => value,
                            None => bail!("Missing field 'skip_initial_version_creation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version_templates: {
                        let field_value = match fields_map.get("version_templates") {
                            Some(value) => value,
                            None => bail!("Missing field 'version_templates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCertificateAuthorityRevocationConfigurationCrlConfiguration {
    /// Name inserted into the certificate CRL Distribution Points extension that enables the use of an alias for the CRL distribution point.
    #[builder(into)]
    #[serde(rename = "customCname")]
    pub r#custom_cname: String,
    /// Boolean value that specifies whether certificate revocation lists (CRLs) are enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Number of days until a certificate expires.
    #[builder(into)]
    #[serde(rename = "expirationInDays")]
    pub r#expiration_in_days: i32,
    /// Name of the S3 bucket that contains the CRL.
    #[builder(into)]
    #[serde(rename = "s3BucketName")]
    pub r#s_3_bucket_name: String,
    /// Whether the CRL is publicly readable or privately held in the CRL Amazon S3 bucket.
    #[builder(into)]
    #[serde(rename = "s3ObjectAcl")]
    pub r#s_3_object_acl: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCertificateAuthorityRevocationConfigurationCrlConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("custom_cname".to_string(), self.r#custom_cname.to_pulumi_value().await);
            map.insert("enabled".to_string(), self.r#enabled.to_pulumi_value().await);
            map.insert("expiration_in_days".to_string(), self.r#expiration_in_days.to_pulumi_value().await);
            map.insert("s_3_bucket_name".to_string(), self.r#s_3_bucket_name.to_pulumi_value().await);
            map.insert("s_3_object_acl".to_string(), self.r#s_3_object_acl.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCertificateAuthorityRevocationConfigurationCrlConfiguration {
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
                    r#custom_cname: {
                        let field_value = match fields_map.get("custom_cname") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_cname' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#expiration_in_days: {
                        let field_value = match fields_map.get("expiration_in_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'expiration_in_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_bucket_name: {
                        let field_value = match fields_map.get("s_3_bucket_name") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_bucket_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_object_acl: {
                        let field_value = match fields_map.get("s_3_object_acl") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_object_acl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

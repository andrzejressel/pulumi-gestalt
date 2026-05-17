#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateAuthorityRevocationConfigurationCrlConfiguration {
    /// Name inserted into the certificate CRL Distribution Points extension that enables the use of an alias for the CRL distribution point. Use this value if you don't want the name of your S3 bucket to be public. Must be less than or equal to 253 characters in length.
    #[builder(into)]
    #[serde(rename = "customCname")]
    pub r#custom_cname: Option<String>,
    /// Boolean value that specifies whether certificate revocation lists (CRLs) are enabled. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Number of days until a certificate expires. Must be between 1 and 5000.
    #[builder(into)]
    #[serde(rename = "expirationInDays")]
    pub r#expiration_in_days: Option<i32>,
    /// Name of the S3 bucket that contains the CRL. If you do not provide a value for the `custom_cname` argument, the name of your S3 bucket is placed into the CRL Distribution Points extension of the issued certificate. You must specify a bucket policy that allows ACM PCA to write the CRL to your bucket. Must be between 3 and 255 characters in length.
    #[builder(into)]
    #[serde(rename = "s3BucketName")]
    pub r#s_3_bucket_name: Option<String>,
    /// Determines whether the CRL will be publicly readable or privately held in the CRL Amazon S3 bucket. Defaults to `PUBLIC_READ`.
    #[builder(into)]
    #[serde(rename = "s3ObjectAcl")]
    pub r#s_3_object_acl: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateAuthorityRevocationConfigurationCrlConfiguration {
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
                "custom_cname".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_cname,
                )
                .await,
            );
            map.insert(
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
                )
                .await,
            );
            map.insert(
                "expiration_in_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expiration_in_days,
                )
                .await,
            );
            map.insert(
                "s_3_bucket_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_bucket_name,
                )
                .await,
            );
            map.insert(
                "s_3_object_acl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_object_acl,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateAuthorityRevocationConfigurationCrlConfiguration {
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
                    r#custom_cname: {
                        let field_value = match fields_map.get("custom_cname") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_cname' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expiration_in_days: {
                        let field_value = match fields_map.get("expiration_in_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'expiration_in_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_bucket_name: {
                        let field_value = match fields_map.get("s_3_bucket_name") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_bucket_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_object_acl: {
                        let field_value = match fields_map.get("s_3_object_acl") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_object_acl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

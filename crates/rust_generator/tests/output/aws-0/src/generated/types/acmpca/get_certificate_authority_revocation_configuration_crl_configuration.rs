#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCertificateAuthorityRevocationConfigurationCrlConfiguration {
    /// Name inserted into the certificate CRL Distribution Points extension that enables the use of an alias for the CRL distribution point.
    #[builder(into)]
    pub r#custom_cname: String,
    /// Boolean value that specifies whether certificate revocation lists (CRLs) are enabled.
    #[builder(into)]
    pub r#enabled: bool,
    /// Number of days until a certificate expires.
    #[builder(into)]
    pub r#expiration_in_days: i32,
    /// Name of the S3 bucket that contains the CRL.
    #[builder(into)]
    pub r#s_3_bucket_name: String,
    /// Whether the CRL is publicly readable or privately held in the CRL Amazon S3 bucket.
    #[builder(into)]
    pub r#s_3_object_acl: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCertificateAuthorityRevocationConfigurationCrlConfiguration {
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
                    "customCname",
                    &self.r#custom_cname,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "expirationInDays",
                    &self.r#expiration_in_days,
                ),
                to_pulumi_object_field(
                    "s3BucketName",
                    &self.r#s_3_bucket_name,
                ),
                to_pulumi_object_field(
                    "s3ObjectAcl",
                    &self.r#s_3_object_acl,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCertificateAuthorityRevocationConfigurationCrlConfiguration {
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
                        let field_value = match fields_map.get("customCname") {
                            Some(value) => value,
                            None => bail!("Missing field 'customCname' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("expirationInDays") {
                            Some(value) => value,
                            None => bail!("Missing field 'expirationInDays' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_bucket_name: {
                        let field_value = match fields_map.get("s3BucketName") {
                            Some(value) => value,
                            None => bail!("Missing field 's3BucketName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_object_acl: {
                        let field_value = match fields_map.get("s3ObjectAcl") {
                            Some(value) => value,
                            None => bail!("Missing field 's3ObjectAcl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

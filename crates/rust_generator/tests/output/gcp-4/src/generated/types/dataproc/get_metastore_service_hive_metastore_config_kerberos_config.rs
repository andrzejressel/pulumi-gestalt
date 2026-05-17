#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetMetastoreServiceHiveMetastoreConfigKerberosConfig {
    /// A Kerberos keytab file that can be used to authenticate a service principal with a Kerberos Key Distribution Center (KDC).
    #[builder(into)]
    #[serde(rename = "keytabs")]
    pub r#keytabs: Vec<super::super::types::dataproc::GetMetastoreServiceHiveMetastoreConfigKerberosConfigKeytab>,
    /// A Cloud Storage URI that specifies the path to a krb5.conf file. It is of the form gs://{bucket_name}/path/to/krb5.conf, although the file does not need to be named krb5.conf explicitly.
    #[builder(into)]
    #[serde(rename = "krb5ConfigGcsUri")]
    pub r#krb_5_config_gcs_uri: String,
    /// A Kerberos principal that exists in the both the keytab the KDC to authenticate as. A typical principal is of the form "primary/instance@REALM", but there is no exact format.
    #[builder(into)]
    #[serde(rename = "principal")]
    pub r#principal: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetMetastoreServiceHiveMetastoreConfigKerberosConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "keytabs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#keytabs,
                )
                .await,
            );
            map.insert(
                "krb_5_config_gcs_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#krb_5_config_gcs_uri,
                )
                .await,
            );
            map.insert(
                "principal".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#principal,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetMetastoreServiceHiveMetastoreConfigKerberosConfig {
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
                    r#keytabs: {
                        let field_value = match fields_map.get("keytabs") {
                            Some(value) => value,
                            None => bail!("Missing field 'keytabs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#krb_5_config_gcs_uri: {
                        let field_value = match fields_map.get("krb_5_config_gcs_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'krb_5_config_gcs_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#principal: {
                        let field_value = match fields_map.get("principal") {
                            Some(value) => value,
                            None => bail!("Missing field 'principal' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetMetastoreServiceHiveMetastoreConfig {
    /// A mapping of Hive metastore version to the auxiliary version configuration.
    /// When specified, a secondary Hive metastore service is created along with the primary service.
    /// All auxiliary versions must be less than the service's primary version.
    /// The key is the auxiliary service name and it must match the regular expression a-z?.
    /// This means that the first character must be a lowercase letter, and all the following characters must be hyphens, lowercase letters, or digits, except the last character, which cannot be a hyphen.
    #[builder(into)]
    #[serde(rename = "auxiliaryVersions")]
    pub r#auxiliary_versions: Vec<super::super::types::dataproc::GetMetastoreServiceHiveMetastoreConfigAuxiliaryVersion>,
    /// A mapping of Hive metastore configuration key-value pairs to apply to the Hive metastore (configured in hive-site.xml).
    /// The mappings override system defaults (some keys cannot be overridden)
    #[builder(into)]
    #[serde(rename = "configOverrides")]
    pub r#config_overrides: std::collections::HashMap<String, String>,
    /// The protocol to use for the metastore service endpoint. If unspecified, defaults to 'THRIFT'. Default value: "THRIFT" Possible values: ["THRIFT", "GRPC"]
    #[builder(into)]
    #[serde(rename = "endpointProtocol")]
    pub r#endpoint_protocol: String,
    /// Information used to configure the Hive metastore service as a service principal in a Kerberos realm.
    #[builder(into)]
    #[serde(rename = "kerberosConfigs")]
    pub r#kerberos_configs: Vec<super::super::types::dataproc::GetMetastoreServiceHiveMetastoreConfigKerberosConfig>,
    /// The Hive metastore schema version.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetMetastoreServiceHiveMetastoreConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "auxiliary_versions",
                    &self.r#auxiliary_versions,
                ),
                to_pulumi_object_field(
                    "config_overrides",
                    &self.r#config_overrides,
                ),
                to_pulumi_object_field(
                    "endpoint_protocol",
                    &self.r#endpoint_protocol,
                ),
                to_pulumi_object_field(
                    "kerberos_configs",
                    &self.r#kerberos_configs,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetMetastoreServiceHiveMetastoreConfig {
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
                    r#auxiliary_versions: {
                        let field_value = match fields_map.get("auxiliary_versions") {
                            Some(value) => value,
                            None => bail!("Missing field 'auxiliary_versions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#config_overrides: {
                        let field_value = match fields_map.get("config_overrides") {
                            Some(value) => value,
                            None => bail!("Missing field 'config_overrides' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#endpoint_protocol: {
                        let field_value = match fields_map.get("endpoint_protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpoint_protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_configs: {
                        let field_value = match fields_map.get("kerberos_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

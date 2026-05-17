#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VMwareClusterVcenter {
    /// (Output)
    /// The vCenter IP address.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Option<String>,
    /// Contains the vCenter CA certificate public key for SSL verification.
    #[builder(into)]
    #[serde(rename = "caCertData")]
    pub r#ca_cert_data: Option<String>,
    /// The name of the vCenter cluster for the user cluster.
    #[builder(into)]
    #[serde(rename = "cluster")]
    pub r#cluster: Option<String>,
    /// The name of the vCenter datacenter for the user cluster.
    #[builder(into)]
    #[serde(rename = "datacenter")]
    pub r#datacenter: Option<String>,
    /// The name of the vCenter datastore for the user cluster.
    #[builder(into)]
    #[serde(rename = "datastore")]
    pub r#datastore: Option<String>,
    /// The name of the vCenter folder for the user cluster.
    #[builder(into)]
    #[serde(rename = "folder")]
    pub r#folder: Option<String>,
    /// The name of the vCenter resource pool for the user cluster.
    #[builder(into)]
    #[serde(rename = "resourcePool")]
    pub r#resource_pool: Option<String>,
    /// The name of the vCenter storage policy for the user cluster.
    #[builder(into)]
    #[serde(rename = "storagePolicyName")]
    pub r#storage_policy_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VMwareClusterVcenter {
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
                    "address",
                    &self.r#address,
                ),
                to_pulumi_object_field(
                    "ca_cert_data",
                    &self.r#ca_cert_data,
                ),
                to_pulumi_object_field(
                    "cluster",
                    &self.r#cluster,
                ),
                to_pulumi_object_field(
                    "datacenter",
                    &self.r#datacenter,
                ),
                to_pulumi_object_field(
                    "datastore",
                    &self.r#datastore,
                ),
                to_pulumi_object_field(
                    "folder",
                    &self.r#folder,
                ),
                to_pulumi_object_field(
                    "resource_pool",
                    &self.r#resource_pool,
                ),
                to_pulumi_object_field(
                    "storage_policy_name",
                    &self.r#storage_policy_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VMwareClusterVcenter {
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
                    r#address: {
                        let field_value = match fields_map.get("address") {
                            Some(value) => value,
                            None => bail!("Missing field 'address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ca_cert_data: {
                        let field_value = match fields_map.get("ca_cert_data") {
                            Some(value) => value,
                            None => bail!("Missing field 'ca_cert_data' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster: {
                        let field_value = match fields_map.get("cluster") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#datacenter: {
                        let field_value = match fields_map.get("datacenter") {
                            Some(value) => value,
                            None => bail!("Missing field 'datacenter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#datastore: {
                        let field_value = match fields_map.get("datastore") {
                            Some(value) => value,
                            None => bail!("Missing field 'datastore' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#folder: {
                        let field_value = match fields_map.get("folder") {
                            Some(value) => value,
                            None => bail!("Missing field 'folder' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_pool: {
                        let field_value = match fields_map.get("resource_pool") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_pool' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_policy_name: {
                        let field_value = match fields_map.get("storage_policy_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_policy_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

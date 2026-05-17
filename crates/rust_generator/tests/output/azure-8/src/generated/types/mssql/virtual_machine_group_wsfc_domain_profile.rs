#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineGroupWsfcDomainProfile {
    /// The account name used for creating cluster. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "clusterBootstrapAccountName")]
    pub r#cluster_bootstrap_account_name: Option<String>,
    /// The account name used for operating cluster. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "clusterOperatorAccountName")]
    pub r#cluster_operator_account_name: Option<String>,
    /// The subnet type of the SQL Virtual Machine cluster. Possible values are `MultiSubnet` and `SingleSubnet`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "clusterSubnetType")]
    pub r#cluster_subnet_type: String,
    /// The fully qualified name of the domain. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: String,
    /// The organizational Unit path in which the nodes and cluster will be present. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "organizationalUnitPath")]
    pub r#organizational_unit_path: Option<String>,
    /// The account name under which SQL service will run on all participating SQL virtual machines in the cluster. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sqlServiceAccountName")]
    pub r#sql_service_account_name: Option<String>,
    /// The primary key of the Storage Account.
    #[builder(into)]
    #[serde(rename = "storageAccountPrimaryKey")]
    pub r#storage_account_primary_key: Option<String>,
    /// The SAS URL to the Storage Container of the witness storage account. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "storageAccountUrl")]
    pub r#storage_account_url: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualMachineGroupWsfcDomainProfile {
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
                "cluster_bootstrap_account_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cluster_bootstrap_account_name,
                )
                .await,
            );
            map.insert(
                "cluster_operator_account_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cluster_operator_account_name,
                )
                .await,
            );
            map.insert(
                "cluster_subnet_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cluster_subnet_type,
                )
                .await,
            );
            map.insert(
                "fqdn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fqdn,
                )
                .await,
            );
            map.insert(
                "organizational_unit_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#organizational_unit_path,
                )
                .await,
            );
            map.insert(
                "sql_service_account_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sql_service_account_name,
                )
                .await,
            );
            map.insert(
                "storage_account_primary_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_account_primary_key,
                )
                .await,
            );
            map.insert(
                "storage_account_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_account_url,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualMachineGroupWsfcDomainProfile {
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
                    r#cluster_bootstrap_account_name: {
                        let field_value = match fields_map.get("cluster_bootstrap_account_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_bootstrap_account_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_operator_account_name: {
                        let field_value = match fields_map.get("cluster_operator_account_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_operator_account_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_subnet_type: {
                        let field_value = match fields_map.get("cluster_subnet_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_subnet_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fqdn: {
                        let field_value = match fields_map.get("fqdn") {
                            Some(value) => value,
                            None => bail!("Missing field 'fqdn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#organizational_unit_path: {
                        let field_value = match fields_map.get("organizational_unit_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'organizational_unit_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sql_service_account_name: {
                        let field_value = match fields_map.get("sql_service_account_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'sql_service_account_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_primary_key: {
                        let field_value = match fields_map.get("storage_account_primary_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_account_primary_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_url: {
                        let field_value = match fields_map.get("storage_account_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_account_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

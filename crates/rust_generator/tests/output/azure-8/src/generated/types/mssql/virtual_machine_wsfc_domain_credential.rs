#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineWsfcDomainCredential {
    /// The account password used for creating cluster.
    #[builder(into)]
    #[serde(rename = "clusterBootstrapAccountPassword")]
    pub r#cluster_bootstrap_account_password: String,
    /// The account password used for operating cluster.
    #[builder(into)]
    #[serde(rename = "clusterOperatorAccountPassword")]
    pub r#cluster_operator_account_password: String,
    /// The account password under which SQL service will run on all participating SQL virtual machines in the cluster.
    #[builder(into)]
    #[serde(rename = "sqlServiceAccountPassword")]
    pub r#sql_service_account_password: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualMachineWsfcDomainCredential {
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
                "cluster_bootstrap_account_password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cluster_bootstrap_account_password,
                )
                .await,
            );
            map.insert(
                "cluster_operator_account_password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cluster_operator_account_password,
                )
                .await,
            );
            map.insert(
                "sql_service_account_password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sql_service_account_password,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualMachineWsfcDomainCredential {
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
                    r#cluster_bootstrap_account_password: {
                        let field_value = match fields_map.get("cluster_bootstrap_account_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_bootstrap_account_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_operator_account_password: {
                        let field_value = match fields_map.get("cluster_operator_account_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_operator_account_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sql_service_account_password: {
                        let field_value = match fields_map.get("sql_service_account_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'sql_service_account_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

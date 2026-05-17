#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IntegrationRuntimeSsisCatalogInfo {
    /// Administrator login name for the SQL Server.
    #[builder(into)]
    #[serde(rename = "administratorLogin")]
    pub r#administrator_login: Option<String>,
    /// Administrator login password for the SQL Server.
    #[builder(into)]
    #[serde(rename = "administratorPassword")]
    pub r#administrator_password: Option<String>,
    /// The dual standby Azure-SSIS Integration Runtime pair with SSISDB failover.
    #[builder(into)]
    #[serde(rename = "dualStandbyPairName")]
    pub r#dual_standby_pair_name: Option<String>,
    /// The name of SQL elastic pool where the database will be created for the SSIS catalog. Mutually exclusive with `pricing_tier`.
    #[builder(into)]
    #[serde(rename = "elasticPoolName")]
    pub r#elastic_pool_name: Option<String>,
    /// Pricing tier for the database that will be created for the SSIS catalog. Valid values are: `Basic`, `S0`, `S1`, `S2`, `S3`, `S4`, `S6`, `S7`, `S9`, `S12`, `P1`, `P2`, `P4`, `P6`, `P11`, `P15`, `GP_S_Gen5_1`, `GP_S_Gen5_2`, `GP_S_Gen5_4`, `GP_S_Gen5_6`, `GP_S_Gen5_8`, `GP_S_Gen5_10`, `GP_S_Gen5_12`, `GP_S_Gen5_14`, `GP_S_Gen5_16`, `GP_S_Gen5_18`, `GP_S_Gen5_20`, `GP_S_Gen5_24`, `GP_S_Gen5_32`, `GP_S_Gen5_40`, `GP_Gen5_2`, `GP_Gen5_4`, `GP_Gen5_6`, `GP_Gen5_8`, `GP_Gen5_10`, `GP_Gen5_12`, `GP_Gen5_14`, `GP_Gen5_16`, `GP_Gen5_18`, `GP_Gen5_20`, `GP_Gen5_24`, `GP_Gen5_32`, `GP_Gen5_40`, `GP_Gen5_80`, `BC_Gen5_2`, `BC_Gen5_4`, `BC_Gen5_6`, `BC_Gen5_8`, `BC_Gen5_10`, `BC_Gen5_12`, `BC_Gen5_14`, `BC_Gen5_16`, `BC_Gen5_18`, `BC_Gen5_20`, `BC_Gen5_24`, `BC_Gen5_32`, `BC_Gen5_40`, `BC_Gen5_80`, `HS_Gen5_2`, `HS_Gen5_4`, `HS_Gen5_6`, `HS_Gen5_8`, `HS_Gen5_10`, `HS_Gen5_12`, `HS_Gen5_14`, `HS_Gen5_16`, `HS_Gen5_18`, `HS_Gen5_20`, `HS_Gen5_24`, `HS_Gen5_32`, `HS_Gen5_40` and `HS_Gen5_80`. Mutually exclusive with `elastic_pool_name`.
    #[builder(into)]
    #[serde(rename = "pricingTier")]
    pub r#pricing_tier: Option<String>,
    /// The endpoint of an Azure SQL Server that will be used to host the SSIS catalog.
    #[builder(into)]
    #[serde(rename = "serverEndpoint")]
    pub r#server_endpoint: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IntegrationRuntimeSsisCatalogInfo {
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
                "administrator_login".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#administrator_login,
                )
                .await,
            );
            map.insert(
                "administrator_password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#administrator_password,
                )
                .await,
            );
            map.insert(
                "dual_standby_pair_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dual_standby_pair_name,
                )
                .await,
            );
            map.insert(
                "elastic_pool_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#elastic_pool_name,
                )
                .await,
            );
            map.insert(
                "pricing_tier".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pricing_tier,
                )
                .await,
            );
            map.insert(
                "server_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#server_endpoint,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IntegrationRuntimeSsisCatalogInfo {
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
                    r#administrator_login: {
                        let field_value = match fields_map.get("administrator_login") {
                            Some(value) => value,
                            None => bail!("Missing field 'administrator_login' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#administrator_password: {
                        let field_value = match fields_map.get("administrator_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'administrator_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dual_standby_pair_name: {
                        let field_value = match fields_map.get("dual_standby_pair_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'dual_standby_pair_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#elastic_pool_name: {
                        let field_value = match fields_map.get("elastic_pool_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'elastic_pool_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pricing_tier: {
                        let field_value = match fields_map.get("pricing_tier") {
                            Some(value) => value,
                            None => bail!("Missing field 'pricing_tier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_endpoint: {
                        let field_value = match fields_map.get("server_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'server_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

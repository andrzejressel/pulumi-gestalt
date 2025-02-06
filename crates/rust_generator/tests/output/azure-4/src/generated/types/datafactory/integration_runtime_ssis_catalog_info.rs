#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IntegrationRuntimeSsisCatalogInfo {
    /// Administrator login name for the SQL Server.
    #[builder(into, default)]
    #[serde(rename = "administratorLogin")]
    pub r#administrator_login: Box<Option<String>>,
    /// Administrator login password for the SQL Server.
    #[builder(into, default)]
    #[serde(rename = "administratorPassword")]
    pub r#administrator_password: Box<Option<String>>,
    /// The dual standby Azure-SSIS Integration Runtime pair with SSISDB failover.
    #[builder(into, default)]
    #[serde(rename = "dualStandbyPairName")]
    pub r#dual_standby_pair_name: Box<Option<String>>,
    /// The name of SQL elastic pool where the database will be created for the SSIS catalog. Mutually exclusive with `pricing_tier`.
    #[builder(into, default)]
    #[serde(rename = "elasticPoolName")]
    pub r#elastic_pool_name: Box<Option<String>>,
    /// Pricing tier for the database that will be created for the SSIS catalog. Valid values are: `Basic`, `S0`, `S1`, `S2`, `S3`, `S4`, `S6`, `S7`, `S9`, `S12`, `P1`, `P2`, `P4`, `P6`, `P11`, `P15`, `GP_S_Gen5_1`, `GP_S_Gen5_2`, `GP_S_Gen5_4`, `GP_S_Gen5_6`, `GP_S_Gen5_8`, `GP_S_Gen5_10`, `GP_S_Gen5_12`, `GP_S_Gen5_14`, `GP_S_Gen5_16`, `GP_S_Gen5_18`, `GP_S_Gen5_20`, `GP_S_Gen5_24`, `GP_S_Gen5_32`, `GP_S_Gen5_40`, `GP_Gen5_2`, `GP_Gen5_4`, `GP_Gen5_6`, `GP_Gen5_8`, `GP_Gen5_10`, `GP_Gen5_12`, `GP_Gen5_14`, `GP_Gen5_16`, `GP_Gen5_18`, `GP_Gen5_20`, `GP_Gen5_24`, `GP_Gen5_32`, `GP_Gen5_40`, `GP_Gen5_80`, `BC_Gen5_2`, `BC_Gen5_4`, `BC_Gen5_6`, `BC_Gen5_8`, `BC_Gen5_10`, `BC_Gen5_12`, `BC_Gen5_14`, `BC_Gen5_16`, `BC_Gen5_18`, `BC_Gen5_20`, `BC_Gen5_24`, `BC_Gen5_32`, `BC_Gen5_40`, `BC_Gen5_80`, `HS_Gen5_2`, `HS_Gen5_4`, `HS_Gen5_6`, `HS_Gen5_8`, `HS_Gen5_10`, `HS_Gen5_12`, `HS_Gen5_14`, `HS_Gen5_16`, `HS_Gen5_18`, `HS_Gen5_20`, `HS_Gen5_24`, `HS_Gen5_32`, `HS_Gen5_40` and `HS_Gen5_80`. Mutually exclusive with `elastic_pool_name`.
    #[builder(into, default)]
    #[serde(rename = "pricingTier")]
    pub r#pricing_tier: Box<Option<String>>,
    /// The endpoint of an Azure SQL Server that will be used to host the SSIS catalog.
    #[builder(into)]
    #[serde(rename = "serverEndpoint")]
    pub r#server_endpoint: Box<String>,
}

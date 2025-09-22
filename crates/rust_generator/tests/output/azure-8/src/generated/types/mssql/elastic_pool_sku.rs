#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ElasticPoolSku {
    /// The scale up/out capacity, representing server's compute units. For more information see the documentation for your Elasticpool configuration: [vCore-based](https://docs.microsoft.com/azure/sql-database/sql-database-vcore-resource-limits-elastic-pools) or [DTU-based](https://docs.microsoft.com/azure/sql-database/sql-database-dtu-resource-limits-elastic-pools).
    #[builder(into)]
    #[serde(rename = "capacity")]
    pub r#capacity: i32,
    /// The `family` of hardware `Gen4`, `Gen5`, `Fsv2`, `MOPRMS`, `PRMS`, or `DC`.
    #[builder(into)]
    #[serde(rename = "family")]
    pub r#family: Option<String>,
    /// Specifies the SKU Name for this Elasticpool. The name of the SKU, will be either `vCore` based or `DTU` based. Possible `DTU` based values are `BasicPool`, `StandardPool`, `PremiumPool` while possible `vCore` based values are `GP_Gen4`, `GP_Gen5`, `GP_Fsv2`, `GP_DC`, `BC_Gen4`, `BC_Gen5`, `BC_DC`, `HS_PRMS`, `HS_MOPRMS`, or `HS_Gen5`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The tier of the particular SKU. Possible values are `GeneralPurpose`, `BusinessCritical`, `Basic`, `Standard`, `Premium`, or `HyperScale`. For more information see the documentation for your Elasticpool configuration: [vCore-based](https://docs.microsoft.com/azure/sql-database/sql-database-vcore-resource-limits-elastic-pools) or [DTU-based](https://docs.microsoft.com/azure/sql-database/sql-database-dtu-resource-limits-elastic-pools).
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: String,
}

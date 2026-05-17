#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineAvailabilityGroupListenerReplica {
    /// The replica commit mode for the availability group. Possible values are `Synchronous_Commit` and `Asynchronous_Commit`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "commit")]
    pub r#commit: String,
    /// The replica failover mode for the availability group. Possible values are `Manual` and `Automatic`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "failoverMode")]
    pub r#failover_mode: String,
    /// The replica readable secondary mode for the availability group. Possible values are `No`, `Read_Only` and `All`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "readableSecondary")]
    pub r#readable_secondary: String,
    /// The replica role for the availability group. Possible values are `Primary` and `Secondary`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "role")]
    pub r#role: String,
    /// The ID of the SQL Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sqlVirtualMachineId")]
    pub r#sql_virtual_machine_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualMachineAvailabilityGroupListenerReplica {
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
                    "commit",
                    &self.r#commit,
                ),
                to_pulumi_object_field(
                    "failover_mode",
                    &self.r#failover_mode,
                ),
                to_pulumi_object_field(
                    "readable_secondary",
                    &self.r#readable_secondary,
                ),
                to_pulumi_object_field(
                    "role",
                    &self.r#role,
                ),
                to_pulumi_object_field(
                    "sql_virtual_machine_id",
                    &self.r#sql_virtual_machine_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualMachineAvailabilityGroupListenerReplica {
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
                    r#commit: {
                        let field_value = match fields_map.get("commit") {
                            Some(value) => value,
                            None => bail!("Missing field 'commit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#failover_mode: {
                        let field_value = match fields_map.get("failover_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'failover_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#readable_secondary: {
                        let field_value = match fields_map.get("readable_secondary") {
                            Some(value) => value,
                            None => bail!("Missing field 'readable_secondary' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role: {
                        let field_value = match fields_map.get("role") {
                            Some(value) => value,
                            None => bail!("Missing field 'role' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sql_virtual_machine_id: {
                        let field_value = match fields_map.get("sql_virtual_machine_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'sql_virtual_machine_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

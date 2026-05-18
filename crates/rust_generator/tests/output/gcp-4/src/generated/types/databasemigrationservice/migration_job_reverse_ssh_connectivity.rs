#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MigrationJobReverseSshConnectivity {
    /// The name of the virtual machine (Compute Engine) used as the bastion server
    /// for the SSH tunnel.
    #[builder(into)]
    #[serde(rename = "vm")]
    pub r#vm: Option<String>,
    /// The IP of the virtual machine (Compute Engine) used as the bastion server
    /// for the SSH tunnel.
    #[builder(into)]
    #[serde(rename = "vmIp")]
    pub r#vm_ip: Option<String>,
    /// The forwarding port of the virtual machine (Compute Engine) used as the
    /// bastion server for the SSH tunnel.
    #[builder(into)]
    #[serde(rename = "vmPort")]
    pub r#vm_port: Option<i32>,
    /// The name of the VPC to peer with the Cloud SQL private network.
    #[builder(into)]
    #[serde(rename = "vpc")]
    pub r#vpc: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MigrationJobReverseSshConnectivity {
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
                    "vm",
                    &self.r#vm,
                ),
                to_pulumi_object_field(
                    "vm_ip",
                    &self.r#vm_ip,
                ),
                to_pulumi_object_field(
                    "vm_port",
                    &self.r#vm_port,
                ),
                to_pulumi_object_field(
                    "vpc",
                    &self.r#vpc,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MigrationJobReverseSshConnectivity {
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
                    r#vm: {
                        let field_value = match fields_map.get("vm") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_ip: {
                        let field_value = match fields_map.get("vm_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_port: {
                        let field_value = match fields_map.get("vm_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc: {
                        let field_value = match fields_map.get("vpc") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

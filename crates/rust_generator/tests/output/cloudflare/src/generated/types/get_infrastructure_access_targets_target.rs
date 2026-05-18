#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetInfrastructureAccessTargetsTarget {
    /// The account identifier to target for the resource.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: String,
    /// The date and time at which the target was created.
    #[builder(into)]
    #[serde(rename = "createdAt")]
    pub r#created_at: String,
    /// A non-unique field that refers to a target.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: String,
    /// The identifier of this resource. This is target's unique identifier.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The IPv4/IPv6 address that identifies where to reach a target.
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Box<super::types::GetInfrastructureAccessTargetsTargetIp>,
    /// The date and time at which the target was last modified.
    #[builder(into)]
    #[serde(rename = "modifiedAt")]
    pub r#modified_at: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetInfrastructureAccessTargetsTarget {
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
                    "account_id",
                    &self.r#account_id,
                ),
                to_pulumi_object_field(
                    "created_at",
                    &self.r#created_at,
                ),
                to_pulumi_object_field(
                    "hostname",
                    &self.r#hostname,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "ip",
                    &self.r#ip,
                ),
                to_pulumi_object_field(
                    "modified_at",
                    &self.r#modified_at,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetInfrastructureAccessTargetsTarget {
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
                    r#account_id: {
                        let field_value = match fields_map.get("account_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'account_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#created_at: {
                        let field_value = match fields_map.get("created_at") {
                            Some(value) => value,
                            None => bail!("Missing field 'created_at' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hostname: {
                        let field_value = match fields_map.get("hostname") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostname' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip: {
                        let field_value = match fields_map.get("ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#modified_at: {
                        let field_value = match fields_map.get("modified_at") {
                            Some(value) => value,
                            None => bail!("Missing field 'modified_at' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

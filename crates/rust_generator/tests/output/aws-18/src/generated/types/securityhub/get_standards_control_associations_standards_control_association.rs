#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetStandardsControlAssociationsStandardsControlAssociation {
    /// Enablement status of a control in a specific standard.
    #[builder(into)]
    #[serde(rename = "associationStatus")]
    pub r#association_status: String,
    /// List of underlying requirements in the compliance framework related to the standard.
    #[builder(into)]
    #[serde(rename = "relatedRequirements")]
    pub r#related_requirements: Vec<String>,
    /// ARN of the security control.
    #[builder(into)]
    #[serde(rename = "securityControlArn")]
    pub r#security_control_arn: String,
    /// The identifier of the control (identified with `SecurityControlId`, `SecurityControlArn`, or a mix of both parameters).
    #[builder(into)]
    #[serde(rename = "securityControlId")]
    pub r#security_control_id: String,
    /// ARN of the standard.
    #[builder(into)]
    #[serde(rename = "standardsArn")]
    pub r#standards_arn: String,
    /// Description of the standard.
    #[builder(into)]
    #[serde(rename = "standardsControlDescription")]
    pub r#standards_control_description: String,
    /// Title of the standard.
    #[builder(into)]
    #[serde(rename = "standardsControlTitle")]
    pub r#standards_control_title: String,
    /// Last time that a control's enablement status in a specified standard was updated.
    #[builder(into)]
    #[serde(rename = "updatedAt")]
    pub r#updated_at: String,
    /// Reason for updating a control's enablement status in a specified standard.
    #[builder(into)]
    #[serde(rename = "updatedReason")]
    pub r#updated_reason: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetStandardsControlAssociationsStandardsControlAssociation {
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
                    "association_status",
                    &self.r#association_status,
                ),
                to_pulumi_object_field(
                    "related_requirements",
                    &self.r#related_requirements,
                ),
                to_pulumi_object_field(
                    "security_control_arn",
                    &self.r#security_control_arn,
                ),
                to_pulumi_object_field(
                    "security_control_id",
                    &self.r#security_control_id,
                ),
                to_pulumi_object_field(
                    "standards_arn",
                    &self.r#standards_arn,
                ),
                to_pulumi_object_field(
                    "standards_control_description",
                    &self.r#standards_control_description,
                ),
                to_pulumi_object_field(
                    "standards_control_title",
                    &self.r#standards_control_title,
                ),
                to_pulumi_object_field(
                    "updated_at",
                    &self.r#updated_at,
                ),
                to_pulumi_object_field(
                    "updated_reason",
                    &self.r#updated_reason,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetStandardsControlAssociationsStandardsControlAssociation {
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
                    r#association_status: {
                        let field_value = match fields_map.get("association_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'association_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#related_requirements: {
                        let field_value = match fields_map.get("related_requirements") {
                            Some(value) => value,
                            None => bail!("Missing field 'related_requirements' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_control_arn: {
                        let field_value = match fields_map.get("security_control_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_control_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_control_id: {
                        let field_value = match fields_map.get("security_control_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_control_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#standards_arn: {
                        let field_value = match fields_map.get("standards_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'standards_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#standards_control_description: {
                        let field_value = match fields_map.get("standards_control_description") {
                            Some(value) => value,
                            None => bail!("Missing field 'standards_control_description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#standards_control_title: {
                        let field_value = match fields_map.get("standards_control_title") {
                            Some(value) => value,
                            None => bail!("Missing field 'standards_control_title' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#updated_at: {
                        let field_value = match fields_map.get("updated_at") {
                            Some(value) => value,
                            None => bail!("Missing field 'updated_at' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#updated_reason: {
                        let field_value = match fields_map.get("updated_reason") {
                            Some(value) => value,
                            None => bail!("Missing field 'updated_reason' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

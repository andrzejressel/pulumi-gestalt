#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RestorePlanRestoreConfigTransformationRuleResourceFilter {
    /// (Filtering parameter) Any resource subject to transformation must
    /// belong to one of the listed "types". If this field is not provided,
    /// no type filtering will be performed
    /// (all resources of all types matching previous filtering parameters
    /// will be candidates for transformation).
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "groupKinds")]
    pub r#group_kinds: Option<Vec<super::super::types::gkebackup::RestorePlanRestoreConfigTransformationRuleResourceFilterGroupKind>>,
    /// This is a JSONPath expression that matches specific fields of
    /// candidate resources and it operates as a filtering parameter
    /// (resources that are not matched with this expression will not
    /// be candidates for transformation).
    #[builder(into)]
    #[serde(rename = "jsonPath")]
    pub r#json_path: Option<String>,
    /// (Filtering parameter) Any resource subject to transformation must
    /// be contained within one of the listed Kubernetes Namespace in the
    /// Backup. If this field is not provided, no namespace filtering will
    /// be performed (all resources in all Namespaces, including all
    /// cluster-scoped resources, will be candidates for transformation).
    /// To mix cluster-scoped and namespaced resources in the same rule,
    /// use an empty string ("") as one of the target namespaces.
    #[builder(into)]
    #[serde(rename = "namespaces")]
    pub r#namespaces: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RestorePlanRestoreConfigTransformationRuleResourceFilter {
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
                "group_kinds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#group_kinds,
                )
                .await,
            );
            map.insert(
                "json_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#json_path,
                )
                .await,
            );
            map.insert(
                "namespaces".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#namespaces,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RestorePlanRestoreConfigTransformationRuleResourceFilter {
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
                    r#group_kinds: {
                        let field_value = match fields_map.get("group_kinds") {
                            Some(value) => value,
                            None => bail!("Missing field 'group_kinds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#json_path: {
                        let field_value = match fields_map.get("json_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'json_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#namespaces: {
                        let field_value = match fields_map.get("namespaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'namespaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

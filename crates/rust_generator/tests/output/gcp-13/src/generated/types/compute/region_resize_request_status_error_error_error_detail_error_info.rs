#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionResizeRequestStatusErrorErrorErrorDetailErrorInfo {
    /// (Output)
    /// The logical grouping to which the "reason" belongs. The error domain is typically the registered service name of the tool or product that generates the error. Example: "pubsub.googleapis.com".
    #[builder(into)]
    #[serde(rename = "domain")]
    pub r#domain: Option<String>,
    /// (Output)
    /// Additional structured details about this error.
    #[builder(into)]
    #[serde(rename = "metadatas")]
    pub r#metadatas: Option<std::collections::HashMap<String, String>>,
    /// (Output)
    /// The reason of the error. This is a constant value that identifies the proximate cause of the error. Error reasons are unique within a particular domain of errors.
    #[builder(into)]
    #[serde(rename = "reason")]
    pub r#reason: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegionResizeRequestStatusErrorErrorErrorDetailErrorInfo {
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
                "domain".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#domain,
                )
                .await,
            );
            map.insert(
                "metadatas".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metadatas,
                )
                .await,
            );
            map.insert(
                "reason".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reason,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegionResizeRequestStatusErrorErrorErrorDetailErrorInfo {
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
                    r#domain: {
                        let field_value = match fields_map.get("domain") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metadatas: {
                        let field_value = match fields_map.get("metadatas") {
                            Some(value) => value,
                            None => bail!("Missing field 'metadatas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reason: {
                        let field_value = match fields_map.get("reason") {
                            Some(value) => value,
                            None => bail!("Missing field 'reason' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

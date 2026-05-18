#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SubscriberSourceCustomLogSourceResource {
    /// The attributes of the third-party custom source. See `attributes` Block below.
    #[builder(into)]
    #[serde(rename = "attributes")]
    pub r#attributes: Option<Vec<super::super::types::securitylake::SubscriberSourceCustomLogSourceResourceAttribute>>,
    /// The details of the log provider for the third-party custom source. See `provider` Block below.
    #[builder(into)]
    #[serde(rename = "providers")]
    pub r#providers: Option<Vec<super::super::types::securitylake::SubscriberSourceCustomLogSourceResourceProvider>>,
    /// The name for a third-party custom source. This must be a Regionally unique value.
    #[builder(into)]
    #[serde(rename = "sourceName")]
    pub r#source_name: String,
    /// The version for a third-party custom source. This must be a Regionally unique value.
    #[builder(into)]
    #[serde(rename = "sourceVersion")]
    pub r#source_version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SubscriberSourceCustomLogSourceResource {
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
                    "attributes",
                    &self.r#attributes,
                ),
                to_pulumi_object_field(
                    "providers",
                    &self.r#providers,
                ),
                to_pulumi_object_field(
                    "source_name",
                    &self.r#source_name,
                ),
                to_pulumi_object_field(
                    "source_version",
                    &self.r#source_version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SubscriberSourceCustomLogSourceResource {
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
                    r#attributes: {
                        let field_value = match fields_map.get("attributes") {
                            Some(value) => value,
                            None => bail!("Missing field 'attributes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#providers: {
                        let field_value = match fields_map.get("providers") {
                            Some(value) => value,
                            None => bail!("Missing field 'providers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_name: {
                        let field_value = match fields_map.get("source_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_version: {
                        let field_value = match fields_map.get("source_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

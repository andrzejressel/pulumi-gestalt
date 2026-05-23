#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CrawlerDeltaTarget {
    /// The name of the connection to use to connect to the Delta table target.
    #[builder(into)]
    pub r#connection_name: Option<String>,
    /// Specifies whether the crawler will create native tables, to allow integration with query engines that support querying of the Delta transaction log directly.
    #[builder(into)]
    pub r#create_native_delta_table: Option<bool>,
    /// A list of the Amazon S3 paths to the Delta tables.
    #[builder(into)]
    pub r#delta_tables: Vec<String>,
    /// Specifies whether to write the manifest files to the Delta table path.
    #[builder(into)]
    pub r#write_manifest: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CrawlerDeltaTarget {
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
                    "connectionName",
                    &self.r#connection_name,
                ),
                to_pulumi_object_field(
                    "createNativeDeltaTable",
                    &self.r#create_native_delta_table,
                ),
                to_pulumi_object_field(
                    "deltaTables",
                    &self.r#delta_tables,
                ),
                to_pulumi_object_field(
                    "writeManifest",
                    &self.r#write_manifest,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CrawlerDeltaTarget {
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
                    r#connection_name: {
                        let field_value = match fields_map.get("connectionName") {
                            Some(value) => value,
                            None => bail!("Missing field 'connectionName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_native_delta_table: {
                        let field_value = match fields_map.get("createNativeDeltaTable") {
                            Some(value) => value,
                            None => bail!("Missing field 'createNativeDeltaTable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#delta_tables: {
                        let field_value = match fields_map.get("deltaTables") {
                            Some(value) => value,
                            None => bail!("Missing field 'deltaTables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#write_manifest: {
                        let field_value = match fields_map.get("writeManifest") {
                            Some(value) => value,
                            None => bail!("Missing field 'writeManifest' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

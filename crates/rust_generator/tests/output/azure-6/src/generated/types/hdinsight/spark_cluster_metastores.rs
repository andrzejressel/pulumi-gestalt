#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SparkClusterMetastores {
    /// An `ambari` block as defined below.
    #[builder(into)]
    #[serde(rename = "ambari")]
    pub r#ambari: Option<Box<super::super::types::hdinsight::SparkClusterMetastoresAmbari>>,
    /// A `hive` block as defined below.
    #[builder(into)]
    #[serde(rename = "hive")]
    pub r#hive: Option<Box<super::super::types::hdinsight::SparkClusterMetastoresHive>>,
    /// An `oozie` block as defined below.
    #[builder(into)]
    #[serde(rename = "oozie")]
    pub r#oozie: Option<Box<super::super::types::hdinsight::SparkClusterMetastoresOozie>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SparkClusterMetastores {
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
                    "ambari",
                    &self.r#ambari,
                ),
                to_pulumi_object_field(
                    "hive",
                    &self.r#hive,
                ),
                to_pulumi_object_field(
                    "oozie",
                    &self.r#oozie,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SparkClusterMetastores {
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
                    r#ambari: {
                        let field_value = match fields_map.get("ambari") {
                            Some(value) => value,
                            None => bail!("Missing field 'ambari' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hive: {
                        let field_value = match fields_map.get("hive") {
                            Some(value) => value,
                            None => bail!("Missing field 'hive' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oozie: {
                        let field_value = match fields_map.get("oozie") {
                            Some(value) => value,
                            None => bail!("Missing field 'oozie' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

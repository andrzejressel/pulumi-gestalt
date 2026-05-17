#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MlTransformParameters {
    /// The parameters for the find matches algorithm. see Find Matches Parameters.
    #[builder(into)]
    #[serde(rename = "findMatchesParameters")]
    pub r#find_matches_parameters: Box<super::super::types::glue::MlTransformParametersFindMatchesParameters>,
    /// The type of machine learning transform. For information about the types of machine learning transforms, see [Creating Machine Learning Transforms](http://docs.aws.amazon.com/glue/latest/dg/add-job-machine-learning-transform.html).
    #[builder(into)]
    #[serde(rename = "transformType")]
    pub r#transform_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MlTransformParameters {
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
                "find_matches_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#find_matches_parameters,
                )
                .await,
            );
            map.insert(
                "transform_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transform_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MlTransformParameters {
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
                    r#find_matches_parameters: {
                        let field_value = match fields_map.get("find_matches_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'find_matches_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transform_type: {
                        let field_value = match fields_map.get("transform_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'transform_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

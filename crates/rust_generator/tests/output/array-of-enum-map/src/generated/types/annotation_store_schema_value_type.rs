#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub enum AnnotationStoreSchemaValueType {
    #[serde(rename = "LONG")]
    Long,
    #[serde(rename = "INT")]
    Int,
    #[serde(rename = "STRING")]
    String,
    #[serde(rename = "FLOAT")]
    Float,
    #[serde(rename = "DOUBLE")]
    Double,
    #[serde(rename = "BOOLEAN")]
    Boolean,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AnnotationStoreSchemaValueType {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        let value = match self {
            AnnotationStoreSchemaValueType::Long => "LONG".to_string(),
            AnnotationStoreSchemaValueType::Int => "INT".to_string(),
            AnnotationStoreSchemaValueType::String => "STRING".to_string(),
            AnnotationStoreSchemaValueType::Float => "FLOAT".to_string(),
            AnnotationStoreSchemaValueType::Double => "DOUBLE".to_string(),
            AnnotationStoreSchemaValueType::Boolean => "BOOLEAN".to_string(),
        };
        std::future::ready(pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue {
            content: pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent::String(value),
            secret: false,
            dependencies: std::collections::HashSet::new(),
        })
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AnnotationStoreSchemaValueType {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;

        match &value.content {
            PulumiValueContent::String(s) => match s.as_str() {
                "LONG" => Ok(AnnotationStoreSchemaValueType::Long),
                "INT" => Ok(AnnotationStoreSchemaValueType::Int),
                "STRING" => Ok(AnnotationStoreSchemaValueType::String),
                "FLOAT" => Ok(AnnotationStoreSchemaValueType::Float),
                "DOUBLE" => Ok(AnnotationStoreSchemaValueType::Double),
                "BOOLEAN" => Ok(AnnotationStoreSchemaValueType::Boolean),
                _ => bail!("Invalid string enum value: {}", s),
            },
            _ => bail!("Expected String, got {:?}", value.content),
        }
    }
}

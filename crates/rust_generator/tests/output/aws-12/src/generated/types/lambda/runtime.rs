#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub enum Runtime {
    #[serde(rename = "dotnet6")]
    Dotnet6,
    #[serde(rename = "dotnet8")]
    Dotnet8,
    #[serde(rename = "java11")]
    Java11,
    #[serde(rename = "java17")]
    Java17,
    #[serde(rename = "java21")]
    Java21,
    #[serde(rename = "java8.al2")]
    Java8Al2,
    #[serde(rename = "nodejs18.x")]
    NodeJs18DX,
    #[serde(rename = "nodejs20.x")]
    NodeJs20DX,
    #[serde(rename = "nodejs22.x")]
    NodeJs22DX,
    #[serde(rename = "provided.al2")]
    CustomAl2,
    #[serde(rename = "provided.al2023")]
    CustomAl2023,
    #[serde(rename = "python3.10")]
    Python3D10,
    #[serde(rename = "python3.11")]
    Python3D11,
    #[serde(rename = "python3.12")]
    Python3D12,
    #[serde(rename = "python3.13")]
    Python3D13,
    #[serde(rename = "python3.9")]
    Python3D9,
    #[serde(rename = "ruby3.2")]
    Ruby3D2,
    #[serde(rename = "ruby3.3")]
    Ruby3D3,
    #[serde(rename = "dotnet5.0")]
    Dotnet5D0,
    #[serde(rename = "dotnet7")]
    Dotnet7,
    #[serde(rename = "dotnetcore2.1")]
    DotnetCore2D1,
    #[serde(rename = "dotnetcore3.1")]
    DotnetCore3D1,
    #[serde(rename = "go1.x")]
    Go1Dx,
    #[serde(rename = "java8")]
    Java8,
    #[serde(rename = "nodejs10.x")]
    NodeJs10DX,
    #[serde(rename = "nodejs12.x")]
    NodeJs12DX,
    #[serde(rename = "nodejs14.x")]
    NodeJs14DX,
    #[serde(rename = "nodejs16.x")]
    NodeJs16DX,
    #[serde(rename = "provided")]
    Custom,
    #[serde(rename = "python2.7")]
    Python2D7,
    #[serde(rename = "python3.6")]
    Python3D6,
    #[serde(rename = "python3.7")]
    Python3D7,
    #[serde(rename = "python3.8")]
    Python3D8,
    #[serde(rename = "ruby2.5")]
    Ruby2D5,
    #[serde(rename = "ruby2.7")]
    Ruby2D7,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for Runtime {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        let value = match self {
            Runtime::Dotnet6 => "dotnet6".to_string(),
            Runtime::Dotnet8 => "dotnet8".to_string(),
            Runtime::Java11 => "java11".to_string(),
            Runtime::Java17 => "java17".to_string(),
            Runtime::Java21 => "java21".to_string(),
            Runtime::Java8Al2 => "java8.al2".to_string(),
            Runtime::NodeJs18DX => "nodejs18.x".to_string(),
            Runtime::NodeJs20DX => "nodejs20.x".to_string(),
            Runtime::NodeJs22DX => "nodejs22.x".to_string(),
            Runtime::CustomAl2 => "provided.al2".to_string(),
            Runtime::CustomAl2023 => "provided.al2023".to_string(),
            Runtime::Python3D10 => "python3.10".to_string(),
            Runtime::Python3D11 => "python3.11".to_string(),
            Runtime::Python3D12 => "python3.12".to_string(),
            Runtime::Python3D13 => "python3.13".to_string(),
            Runtime::Python3D9 => "python3.9".to_string(),
            Runtime::Ruby3D2 => "ruby3.2".to_string(),
            Runtime::Ruby3D3 => "ruby3.3".to_string(),
            Runtime::Dotnet5D0 => "dotnet5.0".to_string(),
            Runtime::Dotnet7 => "dotnet7".to_string(),
            Runtime::DotnetCore2D1 => "dotnetcore2.1".to_string(),
            Runtime::DotnetCore3D1 => "dotnetcore3.1".to_string(),
            Runtime::Go1Dx => "go1.x".to_string(),
            Runtime::Java8 => "java8".to_string(),
            Runtime::NodeJs10DX => "nodejs10.x".to_string(),
            Runtime::NodeJs12DX => "nodejs12.x".to_string(),
            Runtime::NodeJs14DX => "nodejs14.x".to_string(),
            Runtime::NodeJs16DX => "nodejs16.x".to_string(),
            Runtime::Custom => "provided".to_string(),
            Runtime::Python2D7 => "python2.7".to_string(),
            Runtime::Python3D6 => "python3.6".to_string(),
            Runtime::Python3D7 => "python3.7".to_string(),
            Runtime::Python3D8 => "python3.8".to_string(),
            Runtime::Ruby2D5 => "ruby2.5".to_string(),
            Runtime::Ruby2D7 => "ruby2.7".to_string(),
        };
        std::future::ready(pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue {
            content: pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent::String(value),
            secret: false,
            dependencies: std::collections::HashSet::new(),
        })
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for Runtime {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;

        match &value.content {
            PulumiValueContent::String(s) => match s.as_str() {
                "dotnet6" => Ok(Runtime::Dotnet6),
                "dotnet8" => Ok(Runtime::Dotnet8),
                "java11" => Ok(Runtime::Java11),
                "java17" => Ok(Runtime::Java17),
                "java21" => Ok(Runtime::Java21),
                "java8.al2" => Ok(Runtime::Java8Al2),
                "nodejs18.x" => Ok(Runtime::NodeJs18DX),
                "nodejs20.x" => Ok(Runtime::NodeJs20DX),
                "nodejs22.x" => Ok(Runtime::NodeJs22DX),
                "provided.al2" => Ok(Runtime::CustomAl2),
                "provided.al2023" => Ok(Runtime::CustomAl2023),
                "python3.10" => Ok(Runtime::Python3D10),
                "python3.11" => Ok(Runtime::Python3D11),
                "python3.12" => Ok(Runtime::Python3D12),
                "python3.13" => Ok(Runtime::Python3D13),
                "python3.9" => Ok(Runtime::Python3D9),
                "ruby3.2" => Ok(Runtime::Ruby3D2),
                "ruby3.3" => Ok(Runtime::Ruby3D3),
                "dotnet5.0" => Ok(Runtime::Dotnet5D0),
                "dotnet7" => Ok(Runtime::Dotnet7),
                "dotnetcore2.1" => Ok(Runtime::DotnetCore2D1),
                "dotnetcore3.1" => Ok(Runtime::DotnetCore3D1),
                "go1.x" => Ok(Runtime::Go1Dx),
                "java8" => Ok(Runtime::Java8),
                "nodejs10.x" => Ok(Runtime::NodeJs10DX),
                "nodejs12.x" => Ok(Runtime::NodeJs12DX),
                "nodejs14.x" => Ok(Runtime::NodeJs14DX),
                "nodejs16.x" => Ok(Runtime::NodeJs16DX),
                "provided" => Ok(Runtime::Custom),
                "python2.7" => Ok(Runtime::Python2D7),
                "python3.6" => Ok(Runtime::Python3D6),
                "python3.7" => Ok(Runtime::Python3D7),
                "python3.8" => Ok(Runtime::Python3D8),
                "ruby2.5" => Ok(Runtime::Ruby2D5),
                "ruby2.7" => Ok(Runtime::Ruby2D7),
                _ => bail!("Invalid string enum value: {}", s),
            },
            _ => bail!("Expected String, got {:?}", value.content),
        }
    }
}

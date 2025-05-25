#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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

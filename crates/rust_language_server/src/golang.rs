#[derive(rust2go::R2G, Clone)]
pub struct GeneratePackageRequest {
    pub protobuf: Vec<u8>,
    pub directory: String,
}

#[derive(rust2go::R2G, Clone)]
pub struct GenerateProjectRequest {
    pub protobuf: Vec<u8>,
    pub directory: String,
    pub local_dependencies: Vec<LocalDependency>,
    pub testing: bool,
}

#[derive(rust2go::R2G, Clone)]
pub struct GenerateProgramRequest {
    pub protobuf: Vec<u8>,
    pub testing: bool,
}

#[derive(rust2go::R2G, Clone)]
pub struct GenerateProgramResult {
    pub files_content: Vec<FileWithContent>,
    pub error: String,
}

#[derive(rust2go::R2G, Clone)]
pub struct GeneratePackageResult {
    pub error: String,
}

#[derive(rust2go::R2G, Clone)]
pub struct GenerateProjectResult {
    pub error: String,
}

#[derive(rust2go::R2G, Clone)]
pub struct FileWithContent {
    pub path: String,
    pub content: Vec<u8>,
}

#[derive(rust2go::R2G, Clone)]
pub struct LocalDependency {
    pub name: String,
    pub path: String,
}

#[rust2go::g2r]
pub trait G2RCall {
    fn generate_package(req: GeneratePackageRequest) -> GeneratePackageResult;
    fn generate_program(req: GenerateProgramRequest) -> GenerateProgramResult;
    fn generate_project(req: GenerateProjectRequest) -> GenerateProjectResult;
}

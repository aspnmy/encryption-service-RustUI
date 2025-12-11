use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// 业务组状态枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum GroupStatus {
    Running,
    Stopped,
    Starting,
    Stopping,
    Error,
}

/// 容器状态枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ContainerStatus {
    Running,
    Stopped,
    Starting,
    Stopping,
    Error,
}

/// 健康状态枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
    Unknown,
    Checking,
}

/// 调度策略枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum SchedulerStrategy {
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "read_write_split")]
    ReadWriteSplit,
    #[serde(rename = "load_balance")]
    LoadBalance,
}

/// CRUD API实例配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrudApiInstance {
    pub id: String,
    pub url: String,
    pub instance_type: String,
    pub timeout: u64,
    pub retries: u32,
}

/// 服务器配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub https: bool,
}

/// JWT配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expires_in: i64,
    pub refresh_in: i64,
}

/// 加密配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptionConfig {
    pub algorithm: String,
    pub key_length: u32,
    pub iterations: u32,
    pub salt: String,
}

/// 服务角色配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceRoleConfig {
    pub role: String,
    pub id: String,
}

/// CRUD API服务配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrudApiConfig {
    pub instances: Vec<CrudApiInstance>,
    pub strategy: SchedulerStrategy,
    pub health_check_interval: u64,
    pub timeout: u64,
    pub retries: u32,
}

/// 应用配置结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub jwt: JwtConfig,
    pub encryption: EncryptionConfig,
    pub service: ServiceRoleConfig,
    pub crud_api: CrudApiConfig,
}

/// 后端容器模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackendContainer {
    pub id: String,
    pub name: String,
    pub url: String,
    pub instance_type: String,
    pub timeout: u64,
    pub retries: u32,
    pub status: ContainerStatus,
    pub health: HealthStatus,
}

impl Default for BackendContainer {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "新后端容器".to_string(),
            url: "http://localhost:8000".to_string(),
            instance_type: "mixed".to_string(),
            timeout: 5000,
            retries: 3,
            status: ContainerStatus::Stopped,
            health: HealthStatus::Unknown,
        }
    }
}

/// 中间层容器模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MiddlewareContainer {
    pub id: String,
    pub name: String,
    pub url: String,
    pub docker_run_params: String,
    pub config: AppConfig,
    pub backend_containers: Vec<BackendContainer>,
    pub status: ContainerStatus,
    pub health: HealthStatus,
    pub logs: Vec<String>,
    pub agent_installed: bool,
}

impl Default for MiddlewareContainer {
    fn default() -> Self {
        let default_config = AppConfig {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 9999,
                https: false,
            },
            jwt: JwtConfig {
                secret: "default_jwt_secret_123456".to_string(),
                expires_in: 3600,
                refresh_in: 86400,
            },
            encryption: EncryptionConfig {
                algorithm: "aes-256-gcm".to_string(),
                key_length: 32,
                iterations: 100000,
                salt: "default_salt".to_string(),
            },
            service: ServiceRoleConfig {
                role: "mixed".to_string(),
                id: "encryption-01".to_string(),
            },
            crud_api: CrudApiConfig {
                instances: Vec::new(),
                strategy: SchedulerStrategy::ReadWriteSplit,
                health_check_interval: 30,
                timeout: 5000,
                retries: 3,
            },
        };
        
        Self {
            id: Uuid::new_v4().to_string(),
            name: "新中间层容器".to_string(),
            url: "http://localhost:9999".to_string(),
            docker_run_params: "".to_string(),
            config: default_config,
            backend_containers: Vec::new(),
            status: ContainerStatus::Stopped,
            health: HealthStatus::Unknown,
            logs: Vec::new(),
            agent_installed: false,
        }
    }
}

/// 业务组模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BusinessGroup {
    pub id: String,
    pub name: String,
    pub description: String,
    pub middlewares: Vec<MiddlewareContainer>,
    pub backend_containers: Vec<BackendContainer>,
    pub status: GroupStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for BusinessGroup {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: "新业务组".to_string(),
            description: "".to_string(),
            middlewares: Vec::new(),
            backend_containers: Vec::new(),
            status: GroupStatus::Stopped,
            created_at: now,
            updated_at: now,
        }
    }
}

/// 应用状态模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppState {
    pub business_groups: Vec<BusinessGroup>,
    pub selected_group_id: Option<String>,
    pub selected_middleware_id: Option<String>,
    pub selected_backend_id: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            business_groups: Vec::new(),
            selected_group_id: None,
            selected_middleware_id: None,
            selected_backend_id: None,
        }
    }
}

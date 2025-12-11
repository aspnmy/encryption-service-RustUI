use anyhow::Result;
use reqwest::{blocking::Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::models::{AppConfig, HealthStatus};

/// API客户端配置
#[derive(Debug, Clone)]
pub struct ApiClientConfig {
    pub base_url: String,
    pub timeout: u64,
}

/// API客户端
#[derive(Debug, Clone)]
pub struct ApiClient {
    client: Client,
    config: ApiClientConfig,
}

/// 健康检查响应
#[derive(Debug, Deserialize, Serialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub timestamp: String,
    pub service_id: String,
    pub service_role: String,
}

/// 加密请求
#[derive(Debug, Deserialize, Serialize)]
pub struct EncryptRequest {
    pub data: String,
}

/// 加密响应
#[derive(Debug, Deserialize, Serialize)]
pub struct EncryptResponse {
    pub encrypted_data: String,
}

/// 解密请求
#[derive(Debug, Deserialize, Serialize)]
pub struct DecryptRequest {
    pub encrypted_data: String,
}

/// 解密响应
#[derive(Debug, Deserialize, Serialize)]
pub struct DecryptResponse {
    pub data: String,
}

impl ApiClient {
    /// 创建新的API客户端
    pub fn new(config: ApiClientConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_millis(config.timeout))
            .build()?;
        
        Ok(Self {
            client,
            config,
        })
    }
    
    /// 获取配置
    pub fn get_config(&self) -> Result<AppConfig> {
        let url = format!("{}/config", self.config.base_url);
        
        let response = self.client
            .get(&url)
            .send()?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("获取配置失败: {} {}", response.status(), response.text()?);
        }
        
        let config = response.json()?;
        Ok(config)
    }
    
    /// 更新配置
    pub fn update_config(&self, config: &AppConfig) -> Result<()> {
        let url = format!("{}/config", self.config.base_url);
        
        let response = self.client
            .put(&url)
            .json(config)
            .send()?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("更新配置失败: {} {}", response.status(), response.text()?);
        }
        
        Ok(())
    }
    
    /// 健康检查
    pub fn health_check(&self) -> Result<HealthStatus> {
        let url = format!("{}/health", self.config.base_url);
        
        let response = self.client
            .get(&url)
            .send()?;
        
        if response.status() == StatusCode::OK {
            Ok(HealthStatus::Healthy)
        } else {
            Ok(HealthStatus::Unhealthy)
        }
    }
    
    /// 获取状态
    pub fn get_status(&self) -> Result<HealthCheckResponse> {
        let url = format!("{}/health", self.config.base_url);
        
        let response = self.client
            .get(&url)
            .send()?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("获取状态失败: {} {}", response.status(), response.text()?);
        }
        
        let status = response.json()?;
        Ok(status)
    }
    
    /// 重启服务
    pub fn restart(&self) -> Result<()> {
        let url = format!("{}/restart", self.config.base_url);
        
        let response = self.client
            .post(&url)
            .send()?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("重启服务失败: {} {}", response.status(), response.text()?);
        }
        
        Ok(())
    }
    
    /// 加密数据
    pub fn encrypt(&self, data: &str) -> Result<String> {
        let url = format!("{}/encrypt", self.config.base_url);
        
        let request = EncryptRequest {
            data: data.to_string(),
        };
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("加密失败: {} {}", response.status(), response.text()?);
        }
        
        let result: EncryptResponse = response.json()?;
        Ok(result.encrypted_data)
    }
    
    /// 解密数据
    pub fn decrypt(&self, encrypted_data: &str) -> Result<String> {
        let url = format!("{}/decrypt", self.config.base_url);
        
        let request = DecryptRequest {
            encrypted_data: encrypted_data.to_string(),
        };
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("解密失败: {} {}", response.status(), response.text()?);
        }
        
        let result: DecryptResponse = response.json()?;
        Ok(result.data)
    }
    
    /// 获取日志
    pub fn get_logs(&self, limit: u32) -> Result<Vec<String>> {
        let url = format!("{}/logs?limit={}", self.config.base_url, limit);
        
        let response = self.client
            .get(&url)
            .send()?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("获取日志失败: {} {}", response.status(), response.text()?);
        }
        
        let logs: Vec<String> = response.json()?;
        Ok(logs)
    }
}

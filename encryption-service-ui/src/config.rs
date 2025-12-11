use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

use crate::models::AppState;

/// 应用配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub app_state: AppState,
    pub last_opened: String,
    pub theme: String,
    pub auto_save: bool,
    pub save_interval: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_state: AppState::default(),
            last_opened: chrono::Utc::now().to_string(),
            theme: "dark".to_string(),
            auto_save: true,
            save_interval: 30,
        }
    }
}

/// 配置管理器
#[derive(Clone)]
pub struct ConfigManager {
    config_path: String,
}

impl ConfigManager {
    /// 创建新的配置管理器
    pub fn new(config_path: String) -> Self {
        Self {
            config_path,
        }
    }
    
    /// 获取默认配置路径
    pub fn default_config_path() -> String {
        let mut path = std::env::current_dir().expect("无法获取当前目录");
        path.push("config.json");
        path.to_string_lossy().to_string()
    }
    
    /// 加载配置
    pub fn load_config(&self) -> Result<Config> {
        let path = Path::new(&self.config_path);
        
        // 如果配置文件不存在，返回默认配置
        if !path.exists() {
            return Ok(Config::default());
        }
        
        let mut file = File::open(path)
            .context(format!("无法打开配置文件: {}", self.config_path))?;
        
        let mut content = String::new();
        file.read_to_string(&mut content)
            .context(format!("无法读取配置文件: {}", self.config_path))?;
        
        let config: Config = serde_json::from_str(&content)
            .context(format!("无法解析配置文件: {}", self.config_path))?;
        
        Ok(config)
    }
    
    /// 保存配置
    pub fn save_config(&self, config: &Config) -> Result<()> {
        let path = Path::new(&self.config_path);
        
        // 如果目录不存在，创建目录
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .context(format!("无法创建配置目录: {:?}", parent))?;
            }
        }
        
        let content = serde_json::to_string_pretty(config)
            .context("无法序列化配置")?;
        
        let mut file = File::create(path)
            .context(format!("无法创建配置文件: {}", self.config_path))?;
        
        file.write_all(content.as_bytes())
            .context(format!("无法写入配置文件: {}", self.config_path))?;
        
        Ok(())
    }
    
    /// 导入配置
    pub fn import_config(&self, import_path: &str) -> Result<Config> {
        let path = Path::new(import_path);
        
        let mut file = File::open(path)
            .context(format!("无法打开导入文件: {}", import_path))?;
        
        let mut content = String::new();
        file.read_to_string(&mut content)
            .context(format!("无法读取导入文件: {}", import_path))?;
        
        let config: Config = serde_json::from_str(&content)
            .context(format!("无法解析导入文件: {}", import_path))?;
        
        Ok(config)
    }
    
    /// 导出配置
    pub fn export_config(&self, config: &Config, export_path: &str) -> Result<()> {
        let path = Path::new(export_path);
        
        let content = serde_json::to_string_pretty(config)
            .context("无法序列化配置")?;
        
        let mut file = File::create(path)
            .context(format!("无法创建导出文件: {}", export_path))?;
        
        file.write_all(content.as_bytes())
            .context(format!("无法写入导出文件: {}", export_path))?;
        
        Ok(())
    }
    
    /// 备份配置
    pub fn backup_config(&self, config: &Config) -> Result<String> {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_path = format!("config_backup_{}.json", timestamp);
        
        self.export_config(config, &backup_path)?;
        
        Ok(backup_path)
    }
    
    /// 恢复配置
    pub fn restore_config(&self, backup_path: &str) -> Result<Config> {
        let config = self.import_config(backup_path)?;
        self.save_config(&config)?;
        Ok(config)
    }
}

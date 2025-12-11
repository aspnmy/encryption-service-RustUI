use anyhow::{Context, Result};

use crate::models::{BusinessGroup, MiddlewareContainer, BackendContainer, GroupStatus, ContainerStatus};
use crate::api::{ApiClient, ApiClientConfig};
use crate::config::{ConfigManager};

/// 业务组服务
pub struct BusinessGroupService {
    pub config_manager: ConfigManager,
}

impl BusinessGroupService {
    /// 创建新的业务组服务
    pub fn new(config_manager: ConfigManager) -> Self {
        Self {
            config_manager,
        }
    }
    
    /// 获取所有业务组
    pub fn get_all_business_groups(&self) -> Result<Vec<BusinessGroup>> {
        let config = self.config_manager.load_config()?;
        Ok(config.app_state.business_groups.clone())
    }
    
    /// 添加业务组
    pub fn add_business_group(&self, group: BusinessGroup) -> Result<()> {
        let mut config = self.config_manager.load_config()?;
        config.app_state.business_groups.push(group);
        self.config_manager.save_config(&config)
    }
    
    /// 更新业务组
    pub fn update_business_group(&self, group: BusinessGroup) -> Result<()> {
        let mut config = self.config_manager.load_config()?;
        
        if let Some(index) = config.app_state.business_groups.iter().position(|g| g.id == group.id) {
            config.app_state.business_groups[index] = group;
            self.config_manager.save_config(&config)
        } else {
            anyhow::bail!("业务组不存在: {}", group.id)
        }
    }
    
    /// 删除业务组
    pub fn delete_business_group(&self, group_id: &str) -> Result<()> {
        let mut config = self.config_manager.load_config()?;
        
        config.app_state.business_groups.retain(|g| g.id != group_id);
        self.config_manager.save_config(&config)
    }
    
    /// 获取业务组
    pub fn get_business_group(&self, group_id: &str) -> Result<Option<BusinessGroup>> {
        let config = self.config_manager.load_config()?;
        
        let group = config.app_state.business_groups
            .iter()
            .find(|g| g.id == group_id)
            .cloned();
        
        Ok(group)
    }
    
    /// 启动业务组
    pub fn start_business_group(&self, group_id: &str) -> Result<()> {
        let mut config = self.config_manager.load_config()?;
        
        if let Some(group) = config.app_state.business_groups.iter_mut().find(|g| g.id == group_id) {
            group.status = GroupStatus::Starting;
            // 这里可以添加实际的启动逻辑
            group.status = GroupStatus::Running;
            self.config_manager.save_config(&config)
        } else {
            anyhow::bail!("业务组不存在: {}", group_id)
        }
    }
    
    /// 停止业务组
    pub fn stop_business_group(&self, group_id: &str) -> Result<()> {
        let mut config = self.config_manager.load_config()?;
        
        if let Some(group) = config.app_state.business_groups.iter_mut().find(|g| g.id == group_id) {
            group.status = GroupStatus::Stopping;
            // 这里可以添加实际的停止逻辑
            group.status = GroupStatus::Stopped;
            self.config_manager.save_config(&config)
        } else {
            anyhow::bail!("业务组不存在: {}", group_id)
        }
    }
    
    /// 重启业务组
    pub fn restart_business_group(&self, group_id: &str) -> Result<()> {
        self.stop_business_group(group_id)?;
        self.start_business_group(group_id)
    }
}

/// 中间层容器服务
pub struct MiddlewareService {
    config_manager: ConfigManager,
}

impl MiddlewareService {
    /// 创建新的中间层容器服务
    pub fn new(config_manager: ConfigManager) -> Self {
        Self {
            config_manager,
        }
    }
    
    /// 添加中间层容器到业务组
    pub fn add_middleware_to_group(&self, group_id: &str, middleware: MiddlewareContainer) -> Result<()> {
        let mut config = self.config_manager.load_config()?;
        
        if let Some(group) = config.app_state.business_groups.iter_mut().find(|g| g.id == group_id) {
            group.middlewares.push(middleware);
            self.config_manager.save_config(&config)
        } else {
            anyhow::bail!("业务组不存在: {}", group_id)
        }
    }
    
    /// 更新中间层容器
    pub fn update_middleware(&self, group_id: &str, middleware: MiddlewareContainer) -> Result<()> {
        let mut config = self.config_manager.load_config()?;
        
        if let Some(group) = config.app_state.business_groups.iter_mut().find(|g| g.id == group_id) {
            if let Some(index) = group.middlewares.iter().position(|m| m.id == middleware.id) {
                group.middlewares[index] = middleware;
                self.config_manager.save_config(&config)
            } else {
                anyhow::bail!("中间层容器不存在: {}", middleware.id)
            }
        } else {
            anyhow::bail!("业务组不存在: {}", group_id)
        }
    }
    
    /// 删除中间层容器
    pub fn delete_middleware(&self, group_id: &str, middleware_id: &str) -> Result<()> {
        let mut config = self.config_manager.load_config()?;
        
        if let Some(group) = config.app_state.business_groups.iter_mut().find(|g| g.id == group_id) {
            group.middlewares.retain(|m| m.id != middleware_id);
            self.config_manager.save_config(&config)
        } else {
            anyhow::bail!("业务组不存在: {}", group_id)
        }
    }
    
    /// 启动中间层容器
    pub fn start_middleware(&self, group_id: &str, middleware_id: &str) -> Result<()> {
        let mut config = self.config_manager.load_config()?;
        
        if let Some(group) = config.app_state.business_groups.iter_mut().find(|g| g.id == group_id) {
            if let Some(middleware) = group.middlewares.iter_mut().find(|m| m.id == middleware_id) {
                middleware.status = ContainerStatus::Starting;
                // 这里可以添加实际的启动逻辑
                middleware.status = ContainerStatus::Running;
                self.config_manager.save_config(&config)
            } else {
                anyhow::bail!("中间层容器不存在: {}", middleware_id)
            }
        } else {
            anyhow::bail!("业务组不存在: {}", group_id)
        }
    }
    
    /// 停止中间层容器
    pub fn stop_middleware(&self, group_id: &str, middleware_id: &str) -> Result<()> {
        let mut config = self.config_manager.load_config()?;
        
        if let Some(group) = config.app_state.business_groups.iter_mut().find(|g| g.id == group_id) {
            if let Some(middleware) = group.middlewares.iter_mut().find(|m| m.id == middleware_id) {
                middleware.status = ContainerStatus::Stopping;
                // 这里可以添加实际的停止逻辑
                middleware.status = ContainerStatus::Stopped;
                self.config_manager.save_config(&config)
            } else {
                anyhow::bail!("中间层容器不存在: {}", middleware_id)
            }
        } else {
            anyhow::bail!("业务组不存在: {}", group_id)
        }
    }
    
    /// 重启中间层容器
    pub fn restart_middleware(&self, group_id: &str, middleware_id: &str) -> Result<()> {
        self.stop_middleware(group_id, middleware_id)?;
        self.start_middleware(group_id, middleware_id)
    }
}

/// 后端容器服务
pub struct BackendService {
    config_manager: ConfigManager,
}

impl BackendService {
    /// 创建新的后端容器服务
    pub fn new(config_manager: ConfigManager) -> Self {
        Self {
            config_manager,
        }
    }
    
    /// 添加后端容器到中间层
    pub fn add_backend_to_middleware(&self, group_id: &str, middleware_id: &str, backend: BackendContainer) -> Result<()> {
        let mut config = self.config_manager.load_config()?;
        
        if let Some(group) = config.app_state.business_groups.iter_mut().find(|g| g.id == group_id) {
            if let Some(middleware) = group.middlewares.iter_mut().find(|m| m.id == middleware_id) {
                middleware.backend_containers.push(backend);
                self.config_manager.save_config(&config)
            } else {
                anyhow::bail!("中间层容器不存在: {}", middleware_id)
            }
        } else {
            anyhow::bail!("业务组不存在: {}", group_id)
        }
    }
    
    /// 直接添加后端容器到业务组
    pub fn add_backend_to_group(&self, group_id: &str, backend: BackendContainer) -> Result<()> {
        let mut config = self.config_manager.load_config()?;
        
        if let Some(group) = config.app_state.business_groups.iter_mut().find(|g| g.id == group_id) {
            group.backend_containers.push(backend);
            self.config_manager.save_config(&config)
        } else {
            anyhow::bail!("业务组不存在: {}", group_id)
        }
    }
    
    /// 更新后端容器
    pub fn update_backend(&self, group_id: &str, middleware_id: Option<&str>, backend: BackendContainer) -> Result<()> {
        let mut config = self.config_manager.load_config()?;
        
        if let Some(group) = config.app_state.business_groups.iter_mut().find(|g| g.id == group_id) {
            match middleware_id {
                Some(middleware_id) => {
                    // 更新中间层下的后端容器
                    if let Some(middleware) = group.middlewares.iter_mut().find(|m| m.id == middleware_id) {
                        if let Some(index) = middleware.backend_containers.iter().position(|b| b.id == backend.id) {
                            middleware.backend_containers[index] = backend;
                            self.config_manager.save_config(&config)
                        } else {
                            anyhow::bail!("后端容器不存在: {}", backend.id)
                        }
                    } else {
                        anyhow::bail!("中间层容器不存在: {}", middleware_id)
                    }
                },
                None => {
                    // 更新业务组直接管理的后端容器
                    if let Some(index) = group.backend_containers.iter().position(|b| b.id == backend.id) {
                        group.backend_containers[index] = backend;
                        self.config_manager.save_config(&config)
                    } else {
                        anyhow::bail!("后端容器不存在: {}", backend.id)
                    }
                }
            }
        } else {
            anyhow::bail!("业务组不存在: {}", group_id)
        }
    }
    
    /// 删除后端容器
    pub fn delete_backend(&self, group_id: &str, middleware_id: Option<&str>, backend_id: &str) -> Result<()> {
        let mut config = self.config_manager.load_config()?;
        
        if let Some(group) = config.app_state.business_groups.iter_mut().find(|g| g.id == group_id) {
            match middleware_id {
                Some(middleware_id) => {
                    // 删除中间层下的后端容器
                    if let Some(middleware) = group.middlewares.iter_mut().find(|m| m.id == middleware_id) {
                        middleware.backend_containers.retain(|b| b.id != backend_id);
                        self.config_manager.save_config(&config)
                    } else {
                        anyhow::bail!("中间层容器不存在: {}", middleware_id)
                    }
                },
                None => {
                    // 删除业务组直接管理的后端容器
                    group.backend_containers.retain(|b| b.id != backend_id);
                    self.config_manager.save_config(&config)
                }
            }
        } else {
            anyhow::bail!("业务组不存在: {}", group_id)
        }
    }
    
    /// 启动后端容器
    pub fn start_backend(&self, group_id: &str, middleware_id: Option<&str>, backend_id: &str) -> Result<()> {
        let mut config = self.config_manager.load_config()?;
        
        if let Some(group) = config.app_state.business_groups.iter_mut().find(|g| g.id == group_id) {
            match middleware_id {
                Some(middleware_id) => {
                    // 启动中间层下的后端容器
                    if let Some(middleware) = group.middlewares.iter_mut().find(|m| m.id == middleware_id) {
                        if let Some(backend) = middleware.backend_containers.iter_mut().find(|b| b.id == backend_id) {
                            backend.status = ContainerStatus::Starting;
                            // 这里可以添加实际的启动逻辑
                            backend.status = ContainerStatus::Running;
                            self.config_manager.save_config(&config)
                        } else {
                            anyhow::bail!("后端容器不存在: {}", backend_id)
                        }
                    } else {
                        anyhow::bail!("中间层容器不存在: {}", middleware_id)
                    }
                },
                None => {
                    // 启动业务组直接管理的后端容器
                    if let Some(backend) = group.backend_containers.iter_mut().find(|b| b.id == backend_id) {
                        backend.status = ContainerStatus::Starting;
                        // 这里可以添加实际的启动逻辑
                        backend.status = ContainerStatus::Running;
                        self.config_manager.save_config(&config)
                    } else {
                        anyhow::bail!("后端容器不存在: {}", backend_id)
                    }
                }
            }
        } else {
            anyhow::bail!("业务组不存在: {}", group_id)
        }
    }
    
    /// 停止后端容器
    pub fn stop_backend(&self, group_id: &str, middleware_id: Option<&str>, backend_id: &str) -> Result<()> {
        let mut config = self.config_manager.load_config()?;
        
        if let Some(group) = config.app_state.business_groups.iter_mut().find(|g| g.id == group_id) {
            match middleware_id {
                Some(middleware_id) => {
                    // 停止中间层下的后端容器
                    if let Some(middleware) = group.middlewares.iter_mut().find(|m| m.id == middleware_id) {
                        if let Some(backend) = middleware.backend_containers.iter_mut().find(|b| b.id == backend_id) {
                            backend.status = ContainerStatus::Stopping;
                            // 这里可以添加实际的停止逻辑
                            backend.status = ContainerStatus::Stopped;
                            self.config_manager.save_config(&config)
                        } else {
                            anyhow::bail!("后端容器不存在: {}", backend_id)
                        }
                    } else {
                        anyhow::bail!("中间层容器不存在: {}", middleware_id)
                    }
                },
                None => {
                    // 停止业务组直接管理的后端容器
                    if let Some(backend) = group.backend_containers.iter_mut().find(|b| b.id == backend_id) {
                        backend.status = ContainerStatus::Stopping;
                        // 这里可以添加实际的停止逻辑
                        backend.status = ContainerStatus::Stopped;
                        self.config_manager.save_config(&config)
                    } else {
                        anyhow::bail!("后端容器不存在: {}", backend_id)
                    }
                }
            }
        } else {
            anyhow::bail!("业务组不存在: {}", group_id)
        }
    }
    
    /// 重启后端容器
    pub fn restart_backend(&self, group_id: &str, middleware_id: Option<&str>, backend_id: &str) -> Result<()> {
        self.stop_backend(group_id, middleware_id, backend_id)?;
        self.start_backend(group_id, middleware_id, backend_id)
    }
}

/// API服务
pub struct ApiService {
    api_client: Option<ApiClient>,
}

impl ApiService {
    /// 创建新的API服务
    pub fn new() -> Self {
        Self {
            api_client: None,
        }
    }
    
    /// 连接到API服务器
    pub fn connect_to_api(&mut self, base_url: &str, timeout: u64) -> Result<()> {
        let config = ApiClientConfig {
            base_url: base_url.to_string(),
            timeout,
        };
        
        let client = ApiClient::new(config)?;
        self.api_client = Some(client);
        Ok(())
    }
    
    /// 获取API客户端
    pub fn get_api_client(&self) -> Result<&ApiClient> {
        self.api_client.as_ref().context("未连接到API服务器")
    }
}

use eframe::{egui::{self, CentralPanel, SidePanel, TopBottomPanel, Window, RichText, ScrollArea, CollapsingHeader}, epaint::{Color32}};
use chrono::Utc;

use crate::models::{BusinessGroup, MiddlewareContainer, BackendContainer, GroupStatus, ContainerStatus, HealthStatus, SchedulerStrategy};
use crate::services::{BusinessGroupService, MiddlewareService, BackendService, ApiService};
use crate::config::{ConfigManager, Config};

/// 应用状态枚举
#[derive(Debug, Clone, PartialEq, Eq)]
enum AppTab {
    BusinessGroups,
    Middleware,
    Backend,
    Config,
    Monitor,
    Logs,
}

/// 应用结构体
pub struct App {
    /// 业务组服务
    business_group_service: BusinessGroupService,
    /// 中间层服务
    middleware_service: MiddlewareService,
    /// 后端服务
    backend_service: BackendService,
    /// API服务
    api_service: ApiService,
    /// 当前选中的标签页
    current_tab: AppTab,
    /// 业务组列表
    business_groups: Vec<BusinessGroup>,
    /// 当前选中的业务组ID
    selected_group_id: Option<String>,
    /// 当前选中的中间层ID
    selected_middleware_id: Option<String>,
    /// 当前选中的后端ID
    selected_backend_id: Option<String>,
    /// 新建业务组对话框是否打开
    show_new_group_dialog: bool,
    /// 新建业务组数据
    new_group: BusinessGroup,
    /// 新建中间层对话框是否打开
    show_new_middleware_dialog: bool,
    /// 新建中间层数据
    new_middleware: MiddlewareContainer,
    /// 新建后端对话框是否打开
    show_new_backend_dialog: bool,
    /// 新建后端数据
    new_backend: BackendContainer,
    /// 日志列表
    logs: Vec<String>,
    /// 配置管理
    config_manager: ConfigManager,
}

impl App {
    /// 创建新的应用实例
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 配置中文字体
        let mut fonts = egui::FontDefinitions::default();
        
        // 添加系统默认中文字体
        // 对于Windows系统，添加常用的中文字体
        let chinese_fonts = vec![
            "微软雅黑",
            "Microsoft YaHei",
            "SimHei",
            "黑体",
            "SimSun",
            "宋体",
        ];
        
        // 将中文字体添加到字体定义中
        for font in chinese_fonts {
            fonts.font_data.insert(
                font.to_string(),
                egui::FontData::from_static(include_bytes!(r"C:\Windows\Fonts\msyh.ttc")),
            );
            
            // 将中文字体添加到默认字体家族
            fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, font.to_string());
            fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().insert(0, font.to_string());
        }
        
        // 更新上下文的字体
        cc.egui_ctx.set_fonts(fonts);
        
        // 初始化配置管理器
        let config_manager = ConfigManager::new(ConfigManager::default_config_path());
        let business_group_service = BusinessGroupService::new(config_manager.clone());
        let middleware_service = MiddlewareService::new(config_manager.clone());
        let backend_service = BackendService::new(config_manager.clone());
        
        let business_groups = business_group_service.get_all_business_groups().unwrap_or_default();
        
        Self {
            business_group_service,
            middleware_service,
            backend_service,
            api_service: ApiService::new(),
            current_tab: AppTab::BusinessGroups,
            business_groups,
            selected_group_id: None,
            selected_middleware_id: None,
            selected_backend_id: None,
            show_new_group_dialog: false,
            new_group: BusinessGroup::default(),
            show_new_middleware_dialog: false,
            new_middleware: MiddlewareContainer::default(),
            show_new_backend_dialog: false,
            new_backend: BackendContainer::default(),
            logs: Vec::new(),
            config_manager,
        }
    }
    
    /// 加载业务组数据
    fn load_business_groups(&mut self) {
        self.business_groups = self.business_group_service.get_all_business_groups().unwrap_or_default();
    }
    
    /// 获取当前选中的业务组
    fn get_selected_group(&self) -> Option<&BusinessGroup> {
        if let Some(group_id) = &self.selected_group_id {
            self.business_groups.iter().find(|g| g.id == *group_id)
        } else {
            None
        }
    }
    
    /// 获取当前选中的中间层
    fn get_selected_middleware(&self) -> Option<(&BusinessGroup, &MiddlewareContainer)> {
        if let (Some(group_id), Some(middleware_id)) = (&self.selected_group_id, &self.selected_middleware_id) {
            if let Some(group) = self.business_groups.iter().find(|g| g.id == *group_id) {
                if let Some(middleware) = group.middlewares.iter().find(|m| m.id == *middleware_id) {
                    return Some((group, middleware));
                }
            }
        }
        None
    }
    
    /// 获取当前选中的后端
    fn get_selected_backend(&self) -> Option<(&BusinessGroup, &MiddlewareContainer, &BackendContainer)> {
        if let (Some(group_id), Some(middleware_id), Some(backend_id)) = (&self.selected_group_id, &self.selected_middleware_id, &self.selected_backend_id) {
            if let Some(group) = self.business_groups.iter().find(|g| g.id == *group_id) {
                if let Some(middleware) = group.middlewares.iter().find(|m| m.id == *middleware_id) {
                    if let Some(backend) = middleware.backend_containers.iter().find(|b| b.id == *backend_id) {
                        return Some((group, middleware, backend));
                    }
                }
            }
        }
        None
    }
    
    /// 渲染顶部菜单栏
    fn render_menu_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.menu_button("文件", |ui| {
                if ui.button("新建业务组").clicked() {
                    self.show_new_group_dialog = true;
                    ui.close_menu();
                }
                if ui.button("保存配置").clicked() {
                    // 简化保存逻辑
                    let business_groups = self.business_group_service.get_all_business_groups().unwrap();
                    let config = Config {
                        app_state: crate::models::AppState {
                            business_groups,
                            selected_group_id: None,
                            selected_middleware_id: None,
                            selected_backend_id: None,
                        },
                        last_opened: Utc::now().to_string(),
                        theme: "dark".to_string(),
                        auto_save: true,
                        save_interval: 30,
                    };
                    self.config_manager.save_config(&config).unwrap();
                    ui.close_menu();
                }
                if ui.button("退出").clicked() {
                    // 退出应用
                    std::process::exit(0);
                }
            });
            
            ui.menu_button("编辑", |ui| {
                if ui.button("添加中间层").clicked() {
                    self.show_new_middleware_dialog = true;
                    ui.close_menu();
                }
                if ui.button("添加后端").clicked() {
                    self.show_new_backend_dialog = true;
                    ui.close_menu();
                }
            });
            
            ui.menu_button("视图", |ui| {
                if ui.button("业务组").clicked() {
                    self.current_tab = AppTab::BusinessGroups;
                    ui.close_menu();
                }
                if ui.button("中间层").clicked() {
                    self.current_tab = AppTab::Middleware;
                    ui.close_menu();
                }
                if ui.button("后端").clicked() {
                    self.current_tab = AppTab::Backend;
                    ui.close_menu();
                }
                if ui.button("配置").clicked() {
                    self.current_tab = AppTab::Config;
                    ui.close_menu();
                }
                if ui.button("监控").clicked() {
                    self.current_tab = AppTab::Monitor;
                    ui.close_menu();
                }
                if ui.button("日志").clicked() {
                    self.current_tab = AppTab::Logs;
                    ui.close_menu();
                }
            });
            
            ui.menu_button("帮助", |ui| {
                if ui.button("关于").clicked() {
                    ui.close_menu();
                }
            });
        });
    }
    
    /// 渲染左侧导航面板
    fn render_side_panel(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("加密服务管理器");
            ui.separator();
            
            if ui.selectable_label(self.current_tab == AppTab::BusinessGroups, "业务组").clicked() {
                self.current_tab = AppTab::BusinessGroups;
            }
            if ui.selectable_label(self.current_tab == AppTab::Middleware, "中间层").clicked() {
                self.current_tab = AppTab::Middleware;
            }
            if ui.selectable_label(self.current_tab == AppTab::Backend, "后端").clicked() {
                self.current_tab = AppTab::Backend;
            }
            if ui.selectable_label(self.current_tab == AppTab::Config, "配置").clicked() {
                self.current_tab = AppTab::Config;
            }
            if ui.selectable_label(self.current_tab == AppTab::Monitor, "监控").clicked() {
                self.current_tab = AppTab::Monitor;
            }
            if ui.selectable_label(self.current_tab == AppTab::Logs, "日志").clicked() {
                self.current_tab = AppTab::Logs;
            }
            
            ui.separator();
            
            ui.heading("业务组列表");
            ScrollArea::vertical().show(ui, |ui| {
                for group in &self.business_groups {
                    let is_selected = self.selected_group_id == Some(group.id.clone());
                    if ui.selectable_label(is_selected, &group.name).clicked() {
                        self.selected_group_id = Some(group.id.clone());
                        self.selected_middleware_id = None;
                        self.selected_backend_id = None;
                        self.current_tab = AppTab::BusinessGroups;
                    }
                }
            });
        });
    }
    
    /// 渲染业务组标签页
    fn render_business_groups_tab(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("业务组管理");
                if ui.button("新建业务组").clicked() {
                    self.show_new_group_dialog = true;
                }
            });
            ui.separator();
            
            // 复制选中的组ID，避免借用冲突
            let selected_group_id = self.selected_group_id.clone();
            
            if let Some(selected_group_id) = selected_group_id {
                // 重新获取组数据，避免借用冲突
                if let Some(group) = self.business_group_service.get_business_group(&selected_group_id).unwrap() {
                    ui.heading(&group.name);
                    
                    // 保存组ID用于闭包中使用
                    let group_id = group.id.clone();
                    
                    ui.horizontal(|ui| {
                        ui.label("状态:");
                        ui.label(Self::get_status_text(&group.status));
                        
                        ui.add_space(10.0);
                        
                        if ui.button("启动").clicked() {
                            self.business_group_service.start_business_group(&group_id).unwrap();
                            self.load_business_groups();
                        }
                        if ui.button("停止").clicked() {
                            self.business_group_service.stop_business_group(&group_id).unwrap();
                            self.load_business_groups();
                        }
                        if ui.button("重启").clicked() {
                            self.business_group_service.restart_business_group(&group_id).unwrap();
                            self.load_business_groups();
                        }
                        if ui.button("删除").clicked() {
                            self.business_group_service.delete_business_group(&group_id).unwrap();
                            self.selected_group_id = None;
                            self.load_business_groups();
                        }
                    });
                    
                    ui.add_space(10.0);
                    
                    CollapsingHeader::new("中间层容器").show(ui, |ui| {
                        ScrollArea::vertical().show(ui, |ui| {
                            for middleware in &group.middlewares {
                                let middleware_id = middleware.id.clone();
                                let group_id_clone = group_id.clone();
                                
                                ui.collapsing(&middleware.name, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label("状态:");
                                        ui.label(Self::get_container_status_text(&middleware.status));
                                        ui.label("健康状态:");
                                        ui.label(Self::get_health_status_text(&middleware.health));
                                    });
                                    
                                    ui.horizontal(|ui| {
                                        if ui.button("编辑").clicked() {
                                            self.selected_middleware_id = Some(middleware_id.clone());
                                            self.current_tab = AppTab::Middleware;
                                        }
                                        if ui.button("删除").clicked() {
                                            self.middleware_service.delete_middleware(&group_id_clone, &middleware_id).unwrap();
                                            self.load_business_groups();
                                        }
                                    });
                                });
                            }
                            
                            if ui.button("添加中间层").clicked() {
                                self.show_new_middleware_dialog = true;
                            }
                        });
                    });
                    
                    // 显示直接由业务组管理的后端容器
                    CollapsingHeader::new("直接管理的后端容器").show(ui, |ui| {
                        ScrollArea::vertical().show(ui, |ui| {
                            for backend in &group.backend_containers {
                                let backend_id = backend.id.clone();
                                let group_id_clone = group_id.clone();
                                
                                ui.collapsing(&backend.name, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label("URL:");
                                        ui.label(&backend.url);
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label("类型:");
                                        ui.label(&backend.instance_type);
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label("状态:");
                                        ui.label(Self::get_container_status_text(&backend.status));
                                        ui.label("健康状态:");
                                        ui.label(Self::get_health_status_text(&backend.health));
                                    });
                                    
                                    ui.horizontal(|ui| {
                                        if ui.button("编辑").clicked() {
                                            self.selected_backend_id = Some(backend_id.clone());
                                            self.current_tab = AppTab::Backend;
                                        }
                                        if ui.button("删除").clicked() {
                                            self.backend_service.delete_backend(&group_id_clone, None, &backend_id).unwrap();
                                            self.load_business_groups();
                                        }
                                    });
                                });
                            }
                        });
                    });
                }
            } else {
                ui.label("请选择一个业务组或创建新业务组");
            }
        });
    }
    
    /// 渲染中间层标签页
    fn render_middleware_tab(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("中间层管理");
            ui.separator();
            
            // 复制选中的ID，避免借用冲突
            let selected_group_id = self.selected_group_id.clone();
            let selected_middleware_id = self.selected_middleware_id.clone();
            
            if let (Some(selected_group_id), Some(selected_middleware_id)) = (selected_group_id, selected_middleware_id) {
                // 重新获取业务组数据
                if let Some(group) = self.business_group_service.get_business_group(&selected_group_id).unwrap() {
                    // 重新获取中间层数据
                    if let Some(middleware) = group.middlewares.iter().find(|m| m.id == selected_middleware_id) {
                        ui.horizontal(|ui| {
                            ui.label("业务组:");
                            ui.label(&group.name);
                        });
                        
                        ui.add_space(10.0);
                        
                        ui.horizontal(|ui| {
                            ui.label("名称:");
                            ui.label(&middleware.name);
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("访问URL:");
                            ui.label(&middleware.url);
                        });
                        
                        ui.vertical(|ui| {
                            ui.label("Docker Run参数:");
                            ui.label(&middleware.docker_run_params);
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Agent状态:");
                            ui.label(if middleware.agent_installed { "已安装" } else { "未安装" });
                        });
                        
                        // 保存ID用于闭包中使用
                        let group_id = group.id.clone();
                        let middleware_id = middleware.id.clone();
                        
                        ui.horizontal(|ui| {
                            ui.label("状态:");
                            ui.label(Self::get_container_status_text(&middleware.status));
                            
                            if ui.button("启动").clicked() {
                                self.middleware_service.start_middleware(&group_id, &middleware_id).unwrap();
                                self.load_business_groups();
                            }
                            if ui.button("停止").clicked() {
                                self.middleware_service.stop_middleware(&group_id, &middleware_id).unwrap();
                                self.load_business_groups();
                            }
                            if ui.button("重启").clicked() {
                                self.middleware_service.restart_middleware(&group_id, &middleware_id).unwrap();
                                self.load_business_groups();
                            }
                        });
                        
                        ui.add_space(10.0);
                        
                        CollapsingHeader::new("调度策略").show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("策略:");
                                let strategy = middleware.config.crud_api.strategy.clone();
                                ui.label(match strategy {
                                    SchedulerStrategy::Single => "单容器模式",
                                    SchedulerStrategy::ReadWriteSplit => "读写分离模式",
                                    SchedulerStrategy::LoadBalance => "负载均衡模式",
                                });
                            });
                        });
                        
                        CollapsingHeader::new("后端容器").show(ui, |ui| {
                            ScrollArea::vertical().show(ui, |ui| {
                                for backend in &middleware.backend_containers {
                                    let backend_id = backend.id.clone();
                                    let group_id_clone = group_id.clone();
                                    let middleware_id_clone = middleware_id.clone();
                                    
                                    ui.collapsing(&backend.name, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label("URL:");
                                            ui.label(&backend.url);
                                        });
                                        ui.horizontal(|ui| {
                                            ui.label("类型:");
                                            ui.label(&backend.instance_type);
                                        });
                                        ui.horizontal(|ui| {
                                            ui.label("状态:");
                                            ui.label(Self::get_container_status_text(&backend.status));
                                            ui.label("健康状态:");
                                            ui.label(Self::get_health_status_text(&backend.health));
                                        });
                                        
                                        ui.horizontal(|ui| {
                                            if ui.button("编辑").clicked() {
                                                self.selected_backend_id = Some(backend_id.clone());
                                                self.current_tab = AppTab::Backend;
                                            }
                                            if ui.button("删除").clicked() {
                                                self.backend_service.delete_backend(&group_id_clone, Some(&middleware_id_clone as &str), &backend_id).unwrap();
                                                self.load_business_groups();
                                            }
                                        });
                                    });
                                }
                                
                                if ui.button("添加后端").clicked() {
                                    self.show_new_backend_dialog = true;
                                }
                            });
                        });
                    }
                }
            } else {
                ui.label("请选择一个业务组和中间层");
            }
        });
    }
    
    /// 渲染后端标签页
    fn render_backend_tab(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("后端管理");
            ui.separator();
            
            // 复制选中的ID，避免借用冲突
            let selected_group_id = self.selected_group_id.clone();
            let selected_middleware_id = self.selected_middleware_id.clone();
            let selected_backend_id = self.selected_backend_id.clone();
            
            if let (Some(selected_group_id), Some(selected_middleware_id), Some(selected_backend_id)) = 
                (selected_group_id, selected_middleware_id, selected_backend_id) {
                
                // 重新获取业务组数据
                if let Some(group) = self.business_group_service.get_business_group(&selected_group_id).unwrap() {
                    // 重新获取中间层数据
                    if let Some(middleware) = group.middlewares.iter().find(|m| m.id == selected_middleware_id) {
                        // 重新获取后端数据
                        if let Some(backend) = middleware.backend_containers.iter().find(|b| b.id == selected_backend_id) {
                            ui.heading(&backend.name);
                            
                            ui.horizontal(|ui| {
                                ui.label("URL:");
                                ui.label(&backend.url);
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label("类型:");
                                ui.label(&backend.instance_type);
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label("超时时间 (毫秒):");
                                ui.label(&backend.timeout.to_string());
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label("重试次数:");
                                ui.label(&backend.retries.to_string());
                            });
                            
                            // 保存ID用于闭包中使用
                            let group_id = group.id.clone();
                            let middleware_id = middleware.id.clone();
                            let backend_id = backend.id.clone();
                            
                            ui.horizontal(|ui| {
                                ui.label("状态:");
                                ui.label(Self::get_container_status_text(&backend.status));
                                
                                if ui.button("启动").clicked() {
                                    self.backend_service.start_backend(
                                        &group_id,
                                        Some(&middleware_id as &str),
                                        &backend_id
                                    ).unwrap();
                                    self.load_business_groups();
                                }
                                if ui.button("停止").clicked() {
                                    self.backend_service.stop_backend(
                                        &group_id,
                                        Some(&middleware_id as &str),
                                        &backend_id
                                    ).unwrap();
                                    self.load_business_groups();
                                }
                                if ui.button("重启").clicked() {
                                    self.backend_service.restart_backend(
                                        &group_id,
                                        Some(&middleware_id as &str),
                                        &backend_id
                                    ).unwrap();
                                    self.load_business_groups();
                                }
                            });
                        }
                    }
                }
            } else {
                ui.label("请选择一个业务组、中间层和后端");
            }
        });
    }
    
    /// 渲染配置标签页
    fn render_config_tab(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("配置管理");
            ui.separator();
            
            ui.horizontal(|ui| {
                if ui.button("保存配置").clicked() {
                    let config = Config {
                        app_state: self.business_group_service.config_manager.load_config().unwrap().app_state,
                        last_opened: Utc::now().to_string(),
                        theme: "dark".to_string(),
                        auto_save: true,
                        save_interval: 30,
                    };
                    self.config_manager.save_config(&config).unwrap();
                }
                if ui.button("导入配置").clicked() {
                    // TODO: 实现导入配置功能
                }
                if ui.button("导出配置").clicked() {
                    // TODO: 实现导出配置功能
                }
            });
            
            ui.separator();
            
            ui.heading("应用配置");
            ScrollArea::vertical().show(ui, |ui| {
                ui.label("这里显示应用配置详情");
            });
        });
    }
    
    /// 渲染监控标签页
    fn render_monitor_tab(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("监控中心");
            ui.separator();
            
            ui.heading("业务组状态");
            ScrollArea::vertical().show(ui, |ui| {
                for group in &self.business_groups {
                    ui.collapsing(&group.name, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("状态:");
                            ui.label(Self::get_status_text(&group.status));
                        });
                        
                        for middleware in &group.middlewares {
                            ui.collapsing(&middleware.name, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("状态:");
                                    ui.label(Self::get_container_status_text(&middleware.status));
                                    ui.label("健康状态:");
                                    ui.label(Self::get_health_status_text(&middleware.health));
                                });
                                
                                for backend in &middleware.backend_containers {
                                    ui.horizontal(|ui| {
                                        ui.label("  - ");
                                        ui.label(&backend.name);
                                        ui.label(":");
                                        ui.label(Self::get_container_status_text(&backend.status));
                                        ui.label(Self::get_health_status_text(&backend.health));
                                    });
                                }
                            });
                        }
                    });
                }
            });
        });
    }
    
    /// 渲染日志标签页
    fn render_logs_tab(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("日志中心");
            ui.separator();
            
            ScrollArea::vertical().show(ui, |ui| {
                for log in &self.logs {
                    ui.label(log);
                }
            });
        });
    }
    
    /// 渲染新建业务组对话框
    fn render_new_group_dialog(&mut self, ctx: &egui::Context) {
        // 复制对话框状态，避免借用冲突
        let mut show_dialog = self.show_new_group_dialog;
        
        Window::new("新建业务组")
            .open(&mut show_dialog)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("名称:");
                        ui.text_edit_singleline(&mut self.new_group.name);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("描述:");
                        ui.text_edit_multiline(&mut self.new_group.description);
                    });
                    
                    ui.horizontal(|ui| {
                        if ui.button("确定").clicked() {
                            self.business_group_service.add_business_group(self.new_group.clone()).unwrap();
                            self.load_business_groups();
                            self.new_group = BusinessGroup::default();
                            self.show_new_group_dialog = false;
                        }
                        if ui.button("取消").clicked() {
                            self.new_group = BusinessGroup::default();
                            self.show_new_group_dialog = false;
                        }
                    });
                });
            });
        
        // 更新对话框状态
        self.show_new_group_dialog = show_dialog;
    }
    
    /// 渲染新建中间层对话框
    fn render_new_middleware_dialog(&mut self, ctx: &egui::Context) {
        // 复制对话框状态，避免借用冲突
        let mut show_dialog = self.show_new_middleware_dialog;
        let selected_group_id = self.selected_group_id.clone();
        
        Window::new("新建中间层容器")
            .open(&mut show_dialog)
            .resizable(false)
            .show(ctx, |ui| {
                if let Some(group_id) = &selected_group_id {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("名称:");
                            ui.text_edit_singleline(&mut self.new_middleware.name);
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("访问URL:");
                            ui.text_edit_singleline(&mut self.new_middleware.url);
                        });
                        
                        ui.vertical(|ui| {
                            ui.label("Docker Run参数:");
                            ui.text_edit_multiline(&mut self.new_middleware.docker_run_params);
                        });
                        
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.new_middleware.agent_installed, "是否安装Agent");
                        });
                        
                        ui.horizontal(|ui| {
                            if ui.button("确定").clicked() {
                                self.middleware_service.add_middleware_to_group(group_id, self.new_middleware.clone()).unwrap();
                                self.load_business_groups();
                                self.new_middleware = MiddlewareContainer::default();
                                self.show_new_middleware_dialog = false;
                            }
                            if ui.button("取消").clicked() {
                                self.new_middleware = MiddlewareContainer::default();
                                self.show_new_middleware_dialog = false;
                            }
                        });
                    });
                } else {
                    ui.label("请先选择一个业务组");
                    if ui.button("关闭").clicked() {
                        self.show_new_middleware_dialog = false;
                    }
                }
            });
        
        // 更新对话框状态
        self.show_new_middleware_dialog = show_dialog;
    }
    
    /// 渲染新建后端对话框
    fn render_new_backend_dialog(&mut self, ctx: &egui::Context) {
        // 复制对话框状态和选中的ID，避免借用冲突
        let mut show_dialog = self.show_new_backend_dialog;
        let selected_group_id = self.selected_group_id.clone();
        let selected_middleware_id = self.selected_middleware_id.clone();
        
        // 添加一个选项，让用户选择是添加到业务组还是中间层
        let mut add_to_middleware = selected_middleware_id.is_some();
        
        Window::new("新建后端容器")
            .open(&mut show_dialog)
            .resizable(false)
            .show(ctx, |ui| {
                if let Some(group_id) = &selected_group_id {
                    ui.vertical(|ui| {
                        // 添加到业务组还是中间层的选项
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut add_to_middleware, "添加到中间层");
                            if add_to_middleware && selected_middleware_id.is_none() {
                                ui.label("请先选择一个中间层");
                            }
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("名称:");
                            ui.text_edit_singleline(&mut self.new_backend.name);
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("URL:");
                            ui.text_edit_singleline(&mut self.new_backend.url);
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("类型:");
                            let mut instance_type = self.new_backend.instance_type.clone();
                            if ui.radio(instance_type == "read", "读实例").clicked() {
                                instance_type = "read".to_string();
                            }
                            if ui.radio(instance_type == "write", "写实例").clicked() {
                                instance_type = "write".to_string();
                            }
                            if ui.radio(instance_type == "mixed", "混合实例").clicked() {
                                instance_type = "mixed".to_string();
                            }
                            self.new_backend.instance_type = instance_type;
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("超时时间 (毫秒):");
                            ui.text_edit_singleline(&mut self.new_backend.timeout.to_string());
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("重试次数:");
                            ui.text_edit_singleline(&mut self.new_backend.retries.to_string());
                        });
                        
                        ui.horizontal(|ui| {
                            if ui.button("确定").clicked() {
                                if add_to_middleware {
                                    // 添加到中间层
                                    if let Some(middleware_id) = &selected_middleware_id {
                                        self.backend_service.add_backend_to_middleware(group_id, middleware_id, self.new_backend.clone()).unwrap();
                                    }
                                } else {
                                    // 直接添加到业务组
                                    self.backend_service.add_backend_to_group(group_id, self.new_backend.clone()).unwrap();
                                }
                                self.load_business_groups();
                                self.new_backend = BackendContainer::default();
                                self.show_new_backend_dialog = false;
                            }
                            if ui.button("取消").clicked() {
                                self.new_backend = BackendContainer::default();
                                self.show_new_backend_dialog = false;
                            }
                        });
                    });
                } else {
                    ui.label("请先选择一个业务组");
                    if ui.button("关闭").clicked() {
                        self.show_new_backend_dialog = false;
                    }
                }
            });
        
        // 更新对话框状态
        self.show_new_backend_dialog = show_dialog;
    }
    
    /// 获取状态文本
    fn get_status_text(status: &GroupStatus) -> RichText {
        match status {
            GroupStatus::Running => RichText::new("运行中").color(Color32::GREEN),
            GroupStatus::Stopped => RichText::new("已停止").color(Color32::GRAY),
            GroupStatus::Starting => RichText::new("启动中").color(Color32::YELLOW),
            GroupStatus::Stopping => RichText::new("停止中").color(Color32::from_rgb(255, 165, 0)),
            GroupStatus::Error => RichText::new("错误").color(Color32::RED),
        }
    }
    
    /// 获取容器状态文本
    fn get_container_status_text(status: &ContainerStatus) -> RichText {
        match status {
            ContainerStatus::Running => RichText::new("运行中").color(Color32::GREEN),
            ContainerStatus::Stopped => RichText::new("已停止").color(Color32::GRAY),
            ContainerStatus::Starting => RichText::new("启动中").color(Color32::YELLOW),
            ContainerStatus::Stopping => RichText::new("停止中").color(Color32::from_rgb(255, 165, 0)),
            ContainerStatus::Error => RichText::new("错误").color(Color32::RED),
        }
    }
    
    /// 获取健康状态文本
    fn get_health_status_text(status: &HealthStatus) -> RichText {
        match status {
            HealthStatus::Healthy => RichText::new("健康").color(Color32::GREEN),
            HealthStatus::Unhealthy => RichText::new("不健康").color(Color32::RED),
            HealthStatus::Unknown => RichText::new("未知").color(Color32::GRAY),
            HealthStatus::Checking => RichText::new("检查中").color(Color32::YELLOW),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 顶部菜单栏
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.render_menu_bar(ui);
        });
        
        // 左侧导航面板
        SidePanel::left("side_panel").show(ctx, |ui| {
            self.render_side_panel(ui);
        });
        
        // 主内容区域
        CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                AppTab::BusinessGroups => self.render_business_groups_tab(ui),
                AppTab::Middleware => self.render_middleware_tab(ui),
                AppTab::Backend => self.render_backend_tab(ui),
                AppTab::Config => self.render_config_tab(ui),
                AppTab::Monitor => self.render_monitor_tab(ui),
                AppTab::Logs => self.render_logs_tab(ui),
            }
        });
        
        // 底部状态栏
        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("当前选中: {:?}", self.current_tab));
                ui.add_space(10.0);
                ui.label(format!("业务组数量: {}", self.business_groups.len()));
            });
        });
        
        // 对话框
        self.render_new_group_dialog(ctx);
        self.render_new_middleware_dialog(ctx);
        self.render_new_backend_dialog(ctx);
    }
}

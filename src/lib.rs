use systray::Application;
use std::process;
use std::fmt;

#[derive(Debug)]
pub struct SimpleError(String);

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for SimpleError {}

/// 启动 BurnCloud 托盘应用
///
/// 这是唯一需要调用的函数，会创建并运行托盘应用
///
/// # Example
/// ```
/// use burncloud_client_tray::start_tray;
/// start_tray().unwrap();
/// ```
pub fn start_tray() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = Application::new()?;

    // 直接使用 res/burncloud.ico 作为默认图标
    let ico_path = std::path::Path::new("./res/burncloud.ico");

    // 尝试设置图标，如果失败则使用默认方式
    match app.set_icon_from_file(&ico_path.to_string_lossy()) {
        Ok(_) => {},
        Err(_) => {
            // 如果设置图标失败，尝试不设置图标或使用系统默认图标
            println!("Warning: Failed to set custom icon, using default");
        }
    }

    // 添加启动界面菜单项
    app.add_menu_item(&"启动界面".to_string(), |_| -> Result<(), SimpleError> {
        if let Err(e) = webbrowser::open("http://127.0.0.1:8080") {
            eprintln!("Failed to open browser: {}", e);
        }
        Ok(())
    })?;

    // 添加分隔符
    app.add_menu_separator()?;

    // 添加退出菜单项
    app.add_menu_item(&"退出程序".to_string(), |_| -> Result<(), SimpleError> {
        process::exit(0);
    })?;

    // 等待
    app.wait_for_message()?;

    Ok(())
}
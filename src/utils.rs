use colored::Colorize;
use std::process::{Command, Stdio};
use std::error::Error;
use tokio::net::TcpListener as TokioTcpListener;
use std::time::Duration;
use tokio::time::sleep;

pub async fn manage_port(port: u16) -> Result<(), Box<dyn Error>> {
  // 尝试杀死进程并检查端口是否被占用
  if let Err(err) = kill_process_using_port(port).await {
      eprintln!("Error: {}", err);
  }

  // 检查端口是否被占用
  if !is_port_available(port).await {
      eprintln!("\nPort {} is already in use!\n", port);
      if let Err(err) = kill_process_using_port(port).await {
          eprintln!("Error: {}", err);
      }
  }

  Ok(())
}

async fn is_port_available(port: u16) -> bool {
  match TokioTcpListener::bind(("127.0.0.1", port)).await {
      Ok(_) => true,
      Err(_) => false,
  }
}

fn find_pid_by_port(port: u16) -> Option<u32> {
  let output = Command::new("netstat")
      .args(&["-aon"])
      .output()
      .ok()?;

  let stdout = String::from_utf8_lossy(&output.stdout);
  for line in stdout.lines() {
      if line.contains(&port.to_string()) {
          let parts: Vec<&str> = line.split_whitespace().collect();
          if let Some(pid_str) = parts.last() {
              return pid_str.parse().ok();
          }
      }
  }
  None
}
fn find_pid_by_port_unix(port: u16) -> Option<u32> {
  let output = Command::new("lsof")
      .args(&["-ti", format!(":{}", port).as_str()])
      .output()
      .ok()?;

  let stdout = String::from_utf8_lossy(&output.stdout);
  stdout.trim().parse().ok()
}

async fn kill_process_using_port(port: u16) -> Result<(), Box<dyn std::error::Error>> {
  // 尝试绑定端口以检测是否被占用
  match TokioTcpListener::bind(("127.0.0.1", port)).await {
    Ok(_) => {
        Ok(())
    },
    Err(_) => {
      println!("Port {} is in use, attempting to find and kill the process.", port.to_string().as_str().green());
      let pid = if cfg!(target_os = "windows") {
          find_pid_by_port(port).ok_or("No process found using the specified port")?
      } else {
          find_pid_by_port_unix(port).ok_or("No process found using the specified port")?
      };

      // 根据操作系统选择不同的终止进程命令
      let output = if cfg!(target_os = "windows") {
          Command::new("taskkill")
              .args(&["/F", "/PID", &pid.to_string()])
              .stdout(Stdio::null())
              .stderr(Stdio::piped()) // 获取标准错误输出
              .output()?
      } else {
          Command::new("kill")
              .args(&["-9", &pid.to_string()])
              .stdout(Stdio::null())
              .stderr(Stdio::piped()) // 获取标准错误输出
              .output()?
      };

      if !output.status.success() {
          let stderr = String::from_utf8_lossy(&output.stderr);
          eprintln!("  Failed to kill process with PID {}: {}", pid.to_string().as_str().yellow(), stderr.to_string().as_str().red());
      } else {
          println!("{} {}", "  Successfully killed process with PID".green(), pid);
      }

      // 等待一段时间，确保端口释放
      let mut delay = Duration::from_millis(500);
      let max_delay = Duration::from_secs(10); // 最大等待时间
      while !is_port_available(port).await {
          println!("Port {} is still in use, rechecking...", port);
          sleep(delay).await;
          delay = delay * 2;
          if delay > max_delay {
              delay = max_delay;
          }
      }

      println!("Port {} is now available.", port);
      Ok(())
    }
  }
}
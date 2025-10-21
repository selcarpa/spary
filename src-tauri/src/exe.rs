use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Exe {
    path: PathBuf,
    args: Vec<String>,
    child: Arc<Mutex<Option<Child>>>,
    pub pid: Arc<Mutex<Option<u32>>>,
}

impl Exe {
    pub fn new<P: Into<PathBuf>, S: Into<String>>(path: P, args: Vec<S>) -> Self {
        Self {
            path: path.into(),
            args: args.into_iter().map(|s| s.into()).collect(),
            child: Arc::new(Mutex::new(None)),
            pid: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start<F, E, X>(
        &mut self,
        mut on_stdout: F,
        mut on_stderr: E,
        mut on_exit: X,
    ) -> std::io::Result<()>
    where
        F: FnMut(String) + Send + 'static,
        E: FnMut(String) + Send + 'static,
        X: FnMut(i32) + Send + 'static,
    {
        let mut cmd = Command::new(&self.path);
        cmd.args(&self.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd.spawn()?;

        // ✅ 保存 PID
        let pid = child.id();
        {
            *self.pid.lock().unwrap() = Some(pid);
        }

        // ✅ 异步读取 stdout
        if let Some(out) = child.stdout.take() {
            let mut reader = BufReader::new(out);
            let mut line = String::new();
            thread::spawn(move || {
                while let Ok(n) = reader.read_line(&mut line) {
                    if n == 0 {
                        break;
                    }
                    on_stdout(line.clone());
                    line.clear();
                }
            });
        }

        if let Some(err) = child.stderr.take() {
            let mut reader = BufReader::new(err);
            let mut line = String::new();
            thread::spawn(move || {
                while let Ok(n) = reader.read_line(&mut line) {
                    if n == 0 {
                        break;
                    }
                    on_stderr(line.clone());
                    line.clear();
                }
            });
        }

        let child_arc = self.child.clone();
        *child_arc.lock().unwrap() = Some(child);

        let pid_clone = self.pid.clone();
        thread::spawn(move || {
            if let Some(mut c) = child_arc.lock().unwrap().take() {
                match c.wait() {
                    Ok(status) => {
                        let code = status.code().unwrap_or(-1);
                        *pid_clone.lock().unwrap() = None;
                        on_exit(code);
                    }
                    Err(e) => {
                        eprintln!("Error waiting for process: {e}");
                        *pid_clone.lock().unwrap() = None;
                        on_exit(-1);
                    }
                }
            }
        });

        Ok(())
    }

    pub fn kill(&self) -> std::io::Result<()> {
        let mut guard = self.child.lock().unwrap();
        if let Some(child) = guard.as_mut() {
            child.kill()?;
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No running process",
            ))
        }
    }

    pub fn is_alive(&self) -> bool {
        let mut guard = self.child.lock().unwrap();
        if let Some(child) = guard.as_mut() {
            match child.try_wait() {
                Ok(Some(_)) => false, // 已退出
                Ok(None) => true,     // 仍在运行
                Err(_) => false,
            }
        } else {
            false
        }
    }

    pub fn get_pid(&self) -> Option<u32> {
        *self.pid.lock().unwrap()
    }
}

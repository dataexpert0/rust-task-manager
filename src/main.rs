#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Windows Crate imports
use windows::{
    Win32::Foundation::*,
    Win32::System::ProcessStatus::*,
    Win32::System::Threading::*,
};

use eframe::egui;
use std::{ffi::OsString};
use std::os::windows::ffi::OsStringExt;
use std::mem;
use windows::Win32::System::Threading::{TerminateProcess, PROCESS_TERMINATE};

#[derive(Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub memory: String,
}

struct MyApp
{
    processes: Vec<ProcessInfo>,
    pid_to_kill: String,
    status_message: String,
}

impl Default for MyApp
{
    fn default() -> Self
    {
        Self {
            processes: Vec::new(),
            pid_to_kill: String::new(),
            status_message: "Pronto.".to_string(),
        }
    }
}

pub fn get_process_list() -> std::result::Result<Vec<ProcessInfo>, String> {
    let mut process_pids: [u32; 1024] = [0; 1024];
    let mut bytes_returned: u32 = 0;

    let result = unsafe {
        EnumProcesses(
            process_pids.as_mut_ptr(),
            std::mem::size_of_val(&process_pids) as u32,
            &mut bytes_returned,
        )
    };

    if result.is_err() { return Err("Falha enum".to_string()); }

    let num_pids = bytes_returned as usize / std::mem::size_of::<u32>();
    let mut processes = Vec::new();

    for &pid in &process_pids[..num_pids] {
        if pid == 0 { continue; }
        
        let access_flags = PROCESS_QUERY_INFORMATION | PROCESS_VM_READ;
        let process_handle_result = unsafe { OpenProcess(access_flags, false, pid) };

        if let Ok(handle) = process_handle_result {
            let name = get_process_name(handle);
            let memory = working_set_size(pid);
            
            unsafe { let _ = CloseHandle(handle); };

            processes.push(ProcessInfo { pid, name, memory });
        }
    }
    Ok(processes)
}

fn get_process_name(process_handle: HANDLE) -> String {
    let mut path_buf: [u16; 260] = [0; 260];
    let path_len: u32;

    unsafe {
        path_len = GetModuleFileNameExW(process_handle, HMODULE(0), &mut path_buf[..]);
    }

    if path_len == 0 {
        return "[Acesso Negado ou Processo Fechou]".to_string();
    }

    let path_slice = &path_buf[..path_len as usize];
    let os_string = OsString::from_wide(path_slice);

    match os_string.into_string() {
        Ok(s) => std::path::Path::new(&s)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        Err(_) => "[Nome inválido]".to_string(),
    }
}

fn working_set_size(pid: u32) -> String{
    unsafe 
    {
        let process_handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            FALSE,
            pid,
        );

        match process_handle {
            Ok(handle) => {
            let mut pmc: PROCESS_MEMORY_COUNTERS = mem::zeroed();
            pmc.cb = mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;

            let ok = K32GetProcessMemoryInfo(handle, &mut pmc, pmc.cb as u32);

            let _ = CloseHandle(handle);

            if ok.as_bool() {
                let kb = pmc.WorkingSetSize / 1024;
                return format!("{} KB", kb);
            } else {
                return "[Falha ao obter uso de memória]".to_string();
            }
            }
            Err(_) => {
            return "[Sem permissao.]".to_string();
            }
        }
    }
}

fn kill_process(pid: u32) -> bool 
{
    unsafe
    {
        let process_handle = OpenProcess(
            PROCESS_TERMINATE,
            FALSE,
            pid,
        );

        if let Ok(handle) = process_handle
        {
            let success = TerminateProcess(handle, 1);
            let _ = CloseHandle(handle);

            return success.is_ok();
        }
        false
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            
            ui.heading("ActionSupportPRO - Main App");
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("🔄 Atualizar Lista").clicked() {
                    match get_process_list() {
                        Ok(list) => self.processes = list,
                        Err(e) => self.status_message = format!("Erro: {}", e),
                    }
                }

                ui.label("| PID para encerrar:");
                ui.text_edit_singleline(&mut self.pid_to_kill);

                if ui.button("💀 Kill").clicked() {
                    if let Ok(pid) = self.pid_to_kill.trim().parse::<u32>() {
                        if kill_process(pid) {
                            self.status_message = format!("Processo {} eliminado com sucesso.", pid);
                            if let Ok(list) = get_process_list() { self.processes = list; }
                        } else {
                            self.status_message = format!("Falha ao matar PID {}. (Acesso Negado?)", pid);
                        }
                    } else {
                        self.status_message = "PID inválido!".to_string();
                    }
                }
            });

            ui.colored_label(egui::Color32::LIGHT_BLUE, &self.status_message);
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("process_grid").striped(true).show(ui, |ui| {
                    ui.strong("PID");
                    ui.strong("Nome");
                    ui.strong("Memória");
                    ui.end_row();

                    for process in &self.processes {
                        ui.label(process.pid.to_string());
                        ui.label(&process.name);
                        ui.label(&process.memory);
                        ui.end_row();
                    }
                });
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {

    let options = eframe::NativeOptions::default();
    eframe::run_native("Action Support PRO", options, Box::new(|_cc| Box::new(MyApp::default())),)
}
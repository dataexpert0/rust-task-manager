// Windows Crate imports
use windows::{
    Win32::Foundation::*,
    Win32::System::ProcessStatus::*,
    Win32::System::Threading::*,
};

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

#[derive(Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
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

    if result.is_err() {
        return Err("Falha ao enumerar processos.".to_string());
    }

    let num_pids = bytes_returned as usize / std::mem::size_of::<u32>();
    
    let mut processes = Vec::new();

    for &pid in &process_pids[..num_pids] {
        if pid == 0 {
            continue;
        }

        let access_flags = PROCESS_QUERY_INFORMATION | PROCESS_VM_READ;
        let process_handle_result = unsafe { OpenProcess(access_flags, false, pid) };

        let process_handle: HANDLE = match process_handle_result {
            Ok(handle) => handle,
            Err(_) => continue, 
        };

        let process_name = get_process_name(process_handle);

        processes.push(ProcessInfo {
            pid,
            name: process_name,
        });

        unsafe { let _ = CloseHandle(process_handle); }; // Warning tratada: ignorar erro ao fechar handle
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

fn main() {
    println!("Task Manager by dataexpert01");

    match get_process_list() {
        Ok(processes) => {
            println!("Processos encontrados: {}", processes.len());
            println!("{:<10} | {:<50}", "PID", "Nome do Processo");
            println!("{:-<10} | {:-<50}", "", "");

            for p in processes {
                println!("{:<10} | {:<50}", p.pid, p.name);
            }
        }
        Err(e) => {
            println!("Erro: {}", e);
        }
    }
}
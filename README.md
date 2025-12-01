# ActionSupportPRO: Um Gerenciador de Processos em Rust para Windows

**Autor:** [dataexpert0]
**Data:** [06/11/2025]

---

## 1. Resumo (Abstract)

Action Support PRO é um utilitário de sistema leve, desenvolvido na linguagem de programação Rust, com o objetivo de monitorar e interagir com processos do sistema operacional Windows. O projeto utiliza invocações diretas à API do Windows (WinAPI) através da crate `windows-rs`, garantindo alta performance e segurança de memória. Esta ferramenta serve como uma prova de conceito para a viabilidade do Rust no desenvolvimento de software de sistema de baixo nível, oferecendo uma alternativa moderna às ferramentas tradicionais baseadas em C/C++.

## 2. Introdução e Motivação

O gerenciamento de processos é uma função central de qualquer sistema operacional. Ferramentas existentes, embora funcionais, são frequentemente construídas em linguagens que carecem de garantias de segurança de memória (como C++), levando a vulnerabilidades de segurança (ex: *buffer overflows*).

A linguagem Rust apresenta um paradigma de "segurança de memória sem coletor de lixo" (Garbage Collector), tornando-a ideal para softwares de sistema. A motivação deste projeto é explorar essa capacidade, criando uma ferramenta que seja:
* **Segura:** Imune por design a classes inteiras de bugs de memória.
* **Performática:** Comparável a ferramentas nativas, sem a sobrecarga de um *runtime*.
* **Moderna:** Utilizando as melhores práticas de desenvolvimento de software.

## 🛠️ Tecnologias Utilizadas

- **Linguagem:** [Rust](https://www.rust-lang.org/)
- **GUI:** [Egui](https://github.com/emilk/egui) com `eframe`
- **Sistema:** [Windows Crate](https://crates.io/crates/windows) (Win32 API)
    - `Win32::System::Threading`
    - `Win32::System::ProcessStatus`

## 3. Arquitetura e Metodologia

O projeto é construído em torno da crate `windows-rs`, mantida pela Microsoft, que fornece *bindings* Rust idiomáticos para a API do Windows.

A arquitetura central (ramo `main`) é um aplicativo de Interface de Linha de Comando (CLI) que demonstra a lógica principal, separada em três componentes:

1.  **Coleta de Dados (`get_process_list`):**
    * Utiliza `EnumProcesses` para obter um vetor de PIDs (Process Identifiers).
    * Itera sobre cada PID, abrindo um *handle* com `OpenProcess` solicitando as permissões `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ`.
    * Trata falhas de permissão (ex: processos do sistema) de forma graciosa.

2.  **Resolução de Nomes (`get_process_name`):**
    * Utiliza `GetModuleFileNameExW` para extrair o caminho completo do executável do processo.
    * Realiza a conversão de `wchar_t` (UTF-16), padrão do Windows, para `String` (UTF-8), padrão do Rust, usando `OsString::from_wide`.
    * Extrai apenas o nome do arquivo-base do caminho completo.

3.  **Gerenciamento de Recursos:**
    * Demonstra o gerenciamento manual e seguro de recursos do Windows, garantindo que `CloseHandle` seja chamado para cada *handle* aberto, prevenindo vazamentos de recursos.

## 4. Funcionalidades (Features)

* [X] Listagem de todos os processos em execução (PID e Nome).
* [X] Obtenção de uso de memória (Working Set Size).
* [X] Capacidade de finalizar (kill) processos por PID.
* [X] Interface Gráfica (GUI) para interação do usuário.

## 5. Instruções de Compilação e Uso

### Pré-requisitos
* [Rust (stable)](https://rustup.rs/)
* Target `x86_64-pc-windows-msvc` (padrão no Windows)

### Compilação
```bash
# 1. Clone o repositório
git clone https://github.com/dataexpert0/rust-task-manager.git
cd rust-task-manager.git

# 2. Compile em modo de release (otimizado)
cargo build --release

# Mini Gerenciador de Processos em Rust 🦀

Um pequeno utilitário de linha de comando feito em Rust que usa a
API nativa do Windows (WinAPI) para listar processos em execução, 
seus PIDs e o uso de memória (Working Set).

## 🎯 Objetivo

Este projeto foi um exercício de aprendizado para entender como 
interagir com a WinAPI de forma segura usando Rust, especialmente 
a crate `windows-rs`.

## 📚 O que eu aprendi

* Como usar `unsafe` de forma controlada.
* Chamar funções da WinAPI como `EnumProcesses`, `OpenProcess` e `GetProcessMemoryInfo`.
* Gerenciar `HANDLE`s do Windows e a importância de `CloseHandle`.
* Converter strings do Windows (UTF-16) para strings do Rust (UTF-8).
* O padrão da WinAPI de preencher structs (como o campo `cb`).

## 🚀 Como Rodar

1.  Clone o repositório.
2.  Execute `cargo run --release`.

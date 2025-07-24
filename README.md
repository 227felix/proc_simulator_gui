# Instruction Set Simulator für unseren eigenen Mikrorechner

## Dateien
"tauri/src" beinhaltet das Frontend der Tauri-Anwendung, die als GUI für den Simulator dient.

"tauri/src-tauri" enthält die Backend-Logik der Tauri-Anwendung. "tauri/src-tauri/proc.rs" und "tauri/src-tauri/src/my_def.rs" beeinhalten die Logik des Simulators.

## Build
Der Simulator ist in Rust geschrieben und nutzt Tauri für die GUI.
Zum Bauen wird "cargo tauri dev" verwendet, um die Anwendung im Entwicklungsmodus zu starten.

Siehe https://v2.tauri.app/start/

## Performance
Die Tabellen im Frontend sind sehr schlecht implementiert und daher ist die Performance bei 65k Einträgen für den RAM und ROM nicht sehr gut.

## Linux
Keine Ahnung warum aber manchmal kommt ein Fehler ->
WEBKIT_DISABLE_DMABUF_RENDERER=1 ./procsim

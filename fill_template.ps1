$crate = Read-Host "To fill the demo, tell me your egui project crate name: "
$name = Read-Host "To fill the demo, tell me your name (for author in Cargo.toml): "
$email = Read-Host "To fill the demo, tell me your e-mail address (also for Cargo.toml): "

Write-Host "Patching files..."

(Get-Content "Cargo.toml") -replace "eframe_demo", $crate | Set-Content "Cargo.toml"
(Get-Content "src\main.rs") -replace "eframe_demo", $crate | Set-Content "src\main.rs"
(Get-Content "index.html") -replace "eframe demo", $crate -replace "eframe_demo", $crate | Set-Content "index.html"
(Get-Content "assets\sw.js") -replace "eframe_demo", $crate | Set-Content "assets\sw.js"
(Get-Content "Cargo.toml") -replace "Emil Ernerfeldt", $name -replace "emil.ernerfeldt@gmail.com", $email | Set-Content "Cargo.toml"

Write-Host "Done."

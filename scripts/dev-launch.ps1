$cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"

if (Test-Path $cargoBin) {
  $env:PATH += ";" + $cargoBin
}

Set-Location "D:\VScode project\NekoNeo"
corepack pnpm tauri dev

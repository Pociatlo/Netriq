param([string]$EfiPath)

$BiosPath = "$PSScriptRoot\edk2-x86_64-code.fd"

if (-Not (Test-Path $BiosPath)) {
    Write-Host "[ERROR] UEFI Firmware not found!" -ForegroundColor Red
    Write-Host "Please download 'edk2-x86_64-code.fd' and place it in the root directory ($PSScriptRoot)." -ForegroundColor Yellow
    Write-Host "See README.md for details." -ForegroundColor Yellow
    exit 1
}

qemu-system-x86_64 `
  -drive if=pflash,format=raw,readonly=on,file=$BiosPath `
  -drive format=raw,file=fat:rw:$PSScriptRoot\target\x86_64-unknown-uefi\debug\ `
  -net none `
  -nographic `
  -kernel $EfiPath
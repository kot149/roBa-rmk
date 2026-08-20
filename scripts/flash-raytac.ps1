param(
    [Parameter(Mandatory=$true)]
    [string]$Uf2File
)

$BootloaderPattern = '^UF2 Bootloader\s+0\.9\.2\s*$'
$SoftDevicePattern = '^SoftDevice:\s+S140\s+6\.1\.1\s*$'

function Test-IsRaytacLoader {
    param([string]$DriveLetter)

    $drivePath = $DriveLetter + ":\"
    $infoFile = Join-Path $drivePath "INFO_UF2.TXT"

    if (-not (Test-Path $drivePath) -or -not (Test-Path $infoFile)) {
        return $false
    }

    try {
        $info = Get-Content -LiteralPath $infoFile -Raw -ErrorAction Stop
        return $info -match "(?im)$BootloaderPattern" -and
            $info -match "(?im)$SoftDevicePattern"
    }
    catch {
        return $false
    }
}

function Write-Firmware {
    param([string]$TargetDrive, [string]$SourceFile)

    if (-not (Test-IsRaytacLoader -DriveLetter $TargetDrive)) {
        throw "Drive $TargetDrive no longer reports the required Raytac UF2 metadata."
    }

    $targetPath = Join-Path ($TargetDrive + ":\") (Split-Path $SourceFile -Leaf)
    Write-Host "Copying Raytac firmware to drive $TargetDrive..."
    Copy-Item -Path $SourceFile -Destination $targetPath -Force
    Write-Host "Flash completed!"
}

if (-not (Test-Path $Uf2File)) {
    Write-Error "File '$Uf2File' not found."
    exit 1
}

Write-Host "Firmware file: $Uf2File"
Write-Host "Required INFO_UF2.TXT metadata: UF2 Bootloader 0.9.2, SoftDevice S140 6.1.1"

$initialDrives = Get-PSDrive -PSProvider FileSystem
foreach ($drive in $initialDrives) {
    if (Test-IsRaytacLoader -DriveLetter $drive.Name) {
        Write-Host "Raytac UF2 loader found on drive $($drive.Name)"
        Write-Firmware -TargetDrive $drive.Name -SourceFile $Uf2File
        exit 0
    }
}

Write-Host "No compatible Raytac UF2 loader found."
Write-Host "Waiting for a Raytac UF2 loader drive... (Press 'q' to cancel)"

try {
    while ($true) {
        if ([Console]::KeyAvailable) {
            $key = [Console]::ReadKey($true)
            if ($key.KeyChar -eq 'q' -or $key.KeyChar -eq 'Q') {
                Write-Host "`nCancelled."
                exit 0
            }
        }

        Start-Sleep -Milliseconds 100
        $currentDrives = Get-PSDrive -PSProvider FileSystem
        $newDrives = $currentDrives | Where-Object {
            $drive = $_
            -not ($initialDrives | Where-Object { $_.Name -eq $drive.Name })
        }

        foreach ($newDrive in $newDrives) {
            Write-Host "New drive detected: $($newDrive.Name)"
            if (Test-IsRaytacLoader -DriveLetter $newDrive.Name) {
                Write-Host "Compatible Raytac UF2 loader detected on drive $($newDrive.Name)"
                Write-Firmware -TargetDrive $newDrive.Name -SourceFile $Uf2File
                exit 0
            }
            Write-Host "Drive $($newDrive.Name) does not report the required Raytac metadata, skipping..."
        }

        if ($newDrives) {
            $initialDrives = $currentDrives
        }
    }
}
catch {
    Write-Error "An error occurred: $_"
    exit 1
}

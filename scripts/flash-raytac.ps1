param(
    [Parameter(Mandatory=$true)]
    [string]$Uf2File
)

$BoardIdPattern = '(?im)^\s*Board-ID\s*:\s*nRF52840-MDBT50Q_RX-verD\s*$'
$BootloaderPattern = '(?im)^\s*UF2 Bootloader\s+(?:0\.5\.1|0\.9\.2)(?:\s|$)'
$SoftDevicePattern = '(?im)^\s*SoftDevice\s*:\s*S140(?:\s+version)?\s+6\.1\.1(?:\s|$)'

function Get-Uf2Roots {
    foreach ($drive in [System.IO.DriveInfo]::GetDrives()) {
        try {
            if ($drive.IsReady) {
                $drive.RootDirectory.FullName
            }
        }
        catch {
            # A drive can disappear while the device is being mounted.
        }
    }
}

function Get-InfoText {
    param([string]$RootPath)

    $infoFile = Join-Path $RootPath "INFO_UF2.TXT"
    if (-not (Test-Path -LiteralPath $infoFile)) {
        return $null
    }

    try {
        return (Get-Content -LiteralPath $infoFile -ErrorAction Stop | Out-String)
    }
    catch {
        return $null
    }
}

function Test-IsRaytacLoader {
    param([string]$RootPath)

    $info = Get-InfoText -RootPath $RootPath
    if ($null -eq $info) {
        return $false
    }

    return $info -match $BoardIdPattern -and
        $info -match $BootloaderPattern -and
        $info -match $SoftDevicePattern
}

function Find-RaytacLoader {
    param(
        [string[]]$Roots,
        [switch]$Report
    )

    foreach ($root in $Roots) {
        if (Test-IsRaytacLoader -RootPath $root) {
            return $root
        }

        if ($Report -and $null -ne (Get-InfoText -RootPath $root)) {
            Write-Host "INFO_UF2.TXT found on $root, but the Raytac metadata did not match; skipping..."
        }
    }

    return $null
}

function Write-Firmware {
    param([string]$TargetRoot, [string]$SourceFile)

    if (-not (Test-IsRaytacLoader -RootPath $TargetRoot)) {
        throw "Drive $TargetRoot no longer reports the required Raytac UF2 metadata."
    }

    $targetPath = Join-Path $TargetRoot (Split-Path $SourceFile -Leaf)
    Write-Host "Copying Raytac firmware to drive $TargetRoot..."
    Copy-Item -LiteralPath $SourceFile -Destination $targetPath -Force
    Write-Host "Flash completed!"
}

if (-not (Test-Path -LiteralPath $Uf2File)) {
    Write-Error "File '$Uf2File' not found."
    exit 1
}

Write-Host "Firmware file: $Uf2File"
Write-Host "Required INFO_UF2.TXT metadata: Board-ID nRF52840-MDBT50Q_RX-verD, UF2 Bootloader 0.5.1 or 0.9.2, SoftDevice S140 6.1.1"

$initialRoots = @(Get-Uf2Roots)
$targetRoot = Find-RaytacLoader -Roots $initialRoots -Report
if ($null -ne $targetRoot) {
    Write-Host "Raytac UF2 loader found at $targetRoot"
    Write-Firmware -TargetRoot $targetRoot -SourceFile $Uf2File
    exit 0
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

        Start-Sleep -Milliseconds 250
        $currentRoots = @(Get-Uf2Roots)
        $targetRoot = Find-RaytacLoader -Roots $currentRoots
        if ($null -ne $targetRoot) {
            Write-Host "Compatible Raytac UF2 loader detected at $targetRoot"
            Write-Firmware -TargetRoot $targetRoot -SourceFile $Uf2File
            exit 0
        }

        $newRoots = @($currentRoots | Where-Object { $_ -notin $initialRoots })
        foreach ($root in $newRoots) {
            Write-Host "New drive detected: $root"
            if (-not (Test-IsRaytacLoader -RootPath $root)) {
                Write-Host "Drive $root does not report the required Raytac metadata, skipping..."
            }
        }
        $initialRoots = $currentRoots
    }
}
catch {
    Write-Error "An error occurred: $_"
    exit 1
}

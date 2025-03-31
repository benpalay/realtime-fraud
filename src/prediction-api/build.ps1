param (
    [string]$task = "help",
    [string]$id = ""
)

function Dev {
    Write-Output "Starting dev mode..."
    cargo watch -x run
}

function Run {
    Write-Output "Running project..."
    cargo run
}

function Help {
    Write-Output "Available commands:"
    Write-Output "  ./build.ps1 dev                 # Runs 'cargo watch -x run'"
    Write-Output "  ./build.ps1 run                 # Runs 'cargo run'"
    Write-Output "  ./build.ps1 health              # Check health endpoint"
    Write-Output "  ./build.ps1 predictions -id 123 # Get predictions for ID 123"
}

function Health {
    curl http://localhost:3000/health
}

function Predictions {
    param([string]$predictionId)
    
    if ([string]::IsNullOrEmpty($predictionId)) {
        Write-Output "Error: ID parameter is required. Usage: ./build.ps1 predictions -id <id>"
        return
    }
    
    curl "http://localhost:3000/predictions?id=$predictionId"
}

switch ($task) {
    "dev" { Dev }
    "run" { Run }
    "health" { Health }
    "predictions" { Predictions -predictionId $id }
    "help" { Help }
    default { Help }
}
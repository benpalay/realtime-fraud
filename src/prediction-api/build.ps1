param (
    [string]$task = "help"
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
    Write-Output "  ./build.ps1 dev   # Runs 'cargo watch -x run'"
    Write-Output "  ./build.ps1 run   # Runs 'cargo run'"
}

function Health {
    curl http://localhost:3000/health
}

function Predictions {
    curl http://localhost:3000/predictions
}

switch ($task) {
    "dev" { Dev }
    "run" { Run }
    "health" { Health }
    "predictions" { Predictions }
    "help" { Help }
    default { Help }
}

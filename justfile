# List all available just commands
_default:
    @just --list

# Remove all build artifacts and output directories
[group('utility')]
clean:
    rm -rf result-*
    rm -rf target
    rm -rf result
    rm -rf data
    rm -rf .direnv

# Run all Nix build checks
[group('utility')]
check:
    nix flake check --keep-going

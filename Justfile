# If you're not cafkafk and she isn't dead, don't run this!
# 
# usage: release major, release minor, release patch
@release version: 
    cargo bump '{{version}}'
    git cliff -t $(grep '^version' Cargo.toml | head -n 1 | grep -E '([0-9]+)\.([0-9]+)\.([0-9]+)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+[0-9A-Za-z-]+)?' -o) > CHANGELOG.md
    cargo check
    nix build -L ./#clippy
    git checkout -b cafk-release-$(grep '^version' Cargo.toml | head -n 1 | grep -E '([0-9]+)\.([0-9]+)\.([0-9]+)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+[0-9A-Za-z-]+)?' -o)
    git commit -asm "chore: release $(grep '^version' Cargo.toml | head -n 1 | grep -E '([0-9]+)\.([0-9]+)\.([0-9]+)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+[0-9A-Za-z-]+)?' -o)"
    git push
    echo "waiting 10 seconds for github to catch up..."
    sleep 10
    gh pr create --draft --title "chore: release $(grep '^version' Cargo.toml | head -n 1 | grep -E '([0-9]+)\.([0-9]+)\.([0-9]+)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+[0-9A-Za-z-]+)?' -o)" --body "This PR was auto-generated by our lovely just file" --reviewer cafkafk 


#!/bin/bash
#
# PyElevate Changelog Generator
# Generates changelog from git commits with proper formatting
#

set -e

OUTPUT_FILE="${1:-CHANGELOG_GENERATED.md}"
COMMIT_LIMIT="${2:-100}"

generate_changelog() {
    local output="$OUTPUT_FILE"
    
    {
        echo "# PyElevate Changelog"
        echo ""
        echo "Generated: $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
        echo ""
        
        # Get version from Cargo.toml
        local version=$(grep '^version' Cargo.toml | head -1 | grep -oP '(?<=")\d+\.\d+\.\d+(?=")')
        echo "**Current Version:** $version"
        echo ""
        
        # Count contributors
        local contributors=$(git log --format='%an' | sort -u | wc -l)
        echo "**Total Contributors:** $contributors"
        echo ""
        
        echo "## Recent Changes"
        echo ""
        
        # Feature commits
        echo "### Features ✨"
        git log --pretty=format:"- **%h** - %s" --grep='feat' -n 20 2>/dev/null || echo "No recent feature commits"
        echo ""
        
        # Bug fix commits
        echo "### Bug Fixes 🐛"
        git log --pretty=format:"- **%h** - %s" --grep='fix' -n 20 2>/dev/null || echo "No recent bug fixes"
        echo ""
        
        # Documentation commits
        echo "### Documentation 📖"
        git log --pretty=format:"- **%h** - %s" --grep='docs' -n 20 2>/dev/null || echo "No recent documentation updates"
        echo ""
        
        # Performance commits
        echo "### Performance 🚀"
        git log --pretty=format:"- **%h** - %s" --grep='perf' -n 20 2>/dev/null || echo "No recent performance updates"
        echo ""
        
        echo "## All Recent Commits"
        echo ""
        git log --pretty=format:"- **%h** (%an, %ad) - %s" --date=short -n "$COMMIT_LIMIT" 2>/dev/null || echo "No commits found"
        echo ""
        
        echo "## Contributors"
        echo ""
        git log --format='%an' | sort | uniq -c | sort -rn | awk '{print "- " $2 " (" $1 " commits)"}' | head -20
        
    } > "$output"
    
    echo "✅ Changelog generated: $output"
    wc -l "$output"
}

generate_changelog

#!/bin/bash

# Prompt user for a commit message
echo "Enter commit message: "
read commit_message

# Add all changes
git add .

# Commit changes with the provided message
git commit -m "$commit_message"

# Push to the current branch
git push origin $(git rev-parse --abbrev-ref HEAD)

echo "Changes pushed successfully!"

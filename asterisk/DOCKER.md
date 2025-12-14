1. Install pre-reqs
    ```
    brew install docker colima docker-buildx
    brew services start colima    # Start colima Docker runtime automatically
    ```
2. Add CLI plugin support for buildx
    ```
    # Add to ~/.docker/config.json
    {
        "cliPluginsExtraDirs": [
            "/opt/homebrew/lib/docker/cli-plugins"
        ]
    }
    ```
3. Build Dockerfile
    ```
    docker build --tag phonepbx:latest .
    ```
4. Run container
    ```
    docker run --detach phonepbx:latest
    ```

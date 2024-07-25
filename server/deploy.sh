#!/bin/bash

# Define variables
IMAGE_NAME="zoltraak-web-image"
STACK_NAME="zoltraak"
DOCKER_COMPOSE_FILE="docker-compose.yml"
TARBALL="zoltraak-web-image.tar"

# Parse flags
REMOVE_OLD=false
DISTRIBUTE=false

while [[ "$#" -gt 0 ]]; do
    case $1 in
    --remove-old) REMOVE_OLD=true ;;
    --distribute) DISTRIBUTE=true ;;
    *)
        echo "Unknown parameter passed: $1"
        exit 1
        ;;
    esac
    shift
done

if [ "$REMOVE_OLD" = true ]; then
    # Remove all services in the stack
    echo "Removing existing stack services..."
    docker stack rm $STACK_NAME
    # Wait for services to be removed
    sleep 10
fi

# Build the Docker image
echo "Building Docker image..."
docker build -t $IMAGE_NAME .

if [ "$DISTRIBUTE" = true ]; then
    # Save the Docker image to a tarball
    echo "Saving Docker image to tarball..."
    docker save -o $TARBALL $IMAGE_NAME

    # List worker nodes
    NODES=$(docker node ls --format "{{.Hostname}}" | grep -v $(hostname))

    # Distribute the image tarball to all nodes
    for NODE in $NODES; do
        echo "Copying image tarball to $NODE..."
        scp $TARBALL $NODE:~/
        echo "Loading image on $NODE..."
        ssh $NODE "docker load -i ~/$TARBALL"
    done

    # Load the image on the manager node
    echo "Loading image on manager node..."
    docker load -i $TARBALL

    # Clean up the tarball
    echo "Cleaning up tarball..."
    rm $TARBALL
    for NODE in $NODES; do
        ssh $NODE "rm ~/$TARBALL"
    done
fi

# Deploy the Docker stack
echo "Deploying Docker stack..."
docker stack deploy -c $DOCKER_COMPOSE_FILE $STACK_NAME --detach=false

# Output status
echo "Deployment complete. Checking stack services..."
docker stack services $STACK_NAME

echo "Deployment script completed."

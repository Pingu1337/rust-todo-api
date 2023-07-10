echo  "Building and pushing todo-api image to registry.null.care"
docker build -t registry.null.care/todo-api:latest ../todo-api/
docker push registry.null.care/todo-api:latest

echo "Building and pushing todo-client image to registry.null.care"
docker build -t registry.null.care/todo-client:latest ../todo-client/
docker push registry.null.care/todo-client:latest
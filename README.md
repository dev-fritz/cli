# Service Control

## Description
  - This service is responsible for managing the lifecycle of the services. It is responsible for starting, stopping, and restarting the services.
  - It is also responsible for managing the dependencies between the services. It ensures that the services are started in the correct order and that the dependencies are met.

## Features
  - Start a service
  - Stop a service
  - Restart a service
  - Check the status of a service
  - Check the dependencies of a service
  - Check the services that depend on a service
  - Check the services that a service depends on
  
## Usage
  - To start a service:
    ```
    service-control start <service-name>
    ```
  - To stop a service:
    ```
    service-control stop <service-name>
    ```
  - To restart a service:
    ```
    service-control restart <service-name>
    ```
  - To add a service:
    ```
    service-control add  --name <service-name> --start <start_command> -- stop <stop_command> --restart <restart_command>
    ```
  - To remove a service:
    ```
    service-control remove <service-name>
    ```
    
## Json Format
  - The services are stored in a json file. The json file has the following format:
    ```
    {
      "services": [
        {
          "id": 1,
          "name": "service1",
          "start_command": "start_command1",
          "stop_command": "stop_command1",
          "restart_command": "restart_command1"
        },
        {
          "id": 2,
          "name": "service2",
          "start_command": "start_command2",
          "stop_command": "stop_command2",
          "restart_command": "restart_command2"
        },
        {
          "id": 3,
          "name": "service3",
          "start_command": "start_command3",
          "stop_command": "stop_command3",
          "restart_command": "restart_command3"
        }
      ]
    }
    ```
  - The json file is stored in the following location:
    ```
    /home/user/.cli/services.json
    ```
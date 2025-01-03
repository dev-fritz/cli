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
  
## How use
  - To start a service:
    ```
    cli start -n <service-name> -i <service-id>
    ```
  - To stop a service:
    ```
    cli stop -n <service-name> -i <service-id>
    ```
  - To restart a service:
    ```
    cli restart -n <service-name> -i <service-id>
    ```
  - To add a service:
    ```
    cli add  --name <service-name> --start <start_command> -- stop <stop_command> --restart <restart_command>
    ```
  - To remove a service:
    ```
    cli remove -n <service-name> -i <service-id>
    ```
    
## Json Format
  - The services are stored in a json file. The json file has the following format:
    ```
    {
      "services": [
        {
          "id": 1,
          "name": "service1",
          "start": "start_command1",
          "stop": "stop_command1",
          "restart": "restart_command1"
        },
        {
          "id": 2,
          "name": "service2",
          "start": "start_command2",
          "stop": "stop_command2",
          "restart": "restart_command2"
        },
        {
          "id": 3,
          "name": "service3",
          "start": "start_command3",
          "stop": "stop_command3",
          "restart": "restart_command3"
        }
      ]
    }
    ```
  - The json file is stored in the following location:
    ```
    /home/user/.cli/services.json
    ```
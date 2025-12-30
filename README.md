# Using Rust without in a Docker container without installing any dependency


1. Create a new application without installing any rust dependency. This approact does not need to create an image. It gets the latest version of the image located in docker registry

`` docker run -it --rm -v host/path:/usr/src/myapp -w /usr/src/myapp rust cargo new application_name``

Definition by commands
- ``-it``: Interactive mode is set in case there the container is requesting input by user
- ``--rm``: Remove the container and volume once the execution ends
- ``-v`` volume creation helps to sync the changes in the container and in the machine directory usefull to reflect changes either in the container or in the machine
- ``-w`` work path sets the location where the executionn will be located, it's required in order to set execute commands in the right folder within the container
- ``rust`` name of the container located in docker registry
- ``cargo run new application_name`` executes the command in the container


`` docker run --rm -v hotst/project/location/:/usr/src/myapp -w /usr/src/myapp rust cargo run ``


# Using rust with an utility image created in local

This approach requires to create a yaml file with rust in order to execute rust commands in the container

```yml
FROM rust
WORKDIR /usr/src/myapp
ENTRYPOINT [ "cargo" ] 
```
The command ``docker build -t cargo . `` will create a new image in local with the name ``cargo`` and can be used to execute rust command.

The follwing command will be executed using the image created. 

``docker run -it --rm -v host/path:/usr/src/myapp cargo run``



# Using docker compose file to run rust

On order to run using docker compose the yaml file created in the step before is required

The following yaml file has the configuration to setup docker container 

```yaml
services:
  rusthelloworld:
    tty: true
    build:
      context: .
    volumes:
      - ./hello_world:/usr/src/myapp
    working_dir: /usr/src/myapp
    # command: ["run"] in case you want to execute the same command always
```

In order to execute only the service created in the docker compose file and skip the rest of the services if there is any use the following command

``docker-compose run -rm rusthelloworld run``: The last command ``run`` is when there is not command assigned in yaml file

To remove unused images, networkd and build cache, you can run
``docker-compose down --rmi all --volumes --remove-orphans``

Clean up all unused Docker resources system-wide
``docker system prune -a``
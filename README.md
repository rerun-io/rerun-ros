# Rerun ROS bridge
A bridge for connecting Rerun with ROS 2 written in Rust.

## Table of Contents

- [Introduction](#introduction)
- [Building the Bridge](#building-the-bridge)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Introduction

The Rerun ROS Bridge is a tool designed to facilitate communication between the Rerun framework and ROS 2 systems. This bridge is implemented in Rust.

## Building the Bridge

The bridge uses Pixi to install the dependencies and set up the environment. Please check out the following steps to build the bridge:

1. Clone the repository:
    ```sh
    git clone https://github.com/rerun-io/rerun-ros.git
    cd rerun-ros
    ```

2. Install Pixi:
    ```sh
    curl -fsSL https://pixi.sh/install.sh | bash
    ```

3. Use Pixi to install dependencies and set up the environment:
    ```sh
    pixi install
    ```

4. Build the project:
    ```sh
    pixi run build
    ```

## Usage

To use the Rerun ROS Bridge, follow these steps:

1. Run the bridge:
    ```sh
    pixi run ros2 rerun_ros rerun_ros -c config/example.toml
    ```

2. Configure your ROS 2 nodes to communicate with the bridge.

## Contributing

For now the converters are part of the build, but in the future we might add support for dynamically loaded plug-ins at run time. In order to add a new converter for a ROS type, the steps are:

 - Implement the `Converter` trait
 - Add your converter `struct` to the list of converters in XXX
 - Add a conversion configuration in `config/`

## License

This project is licensed under the MIT and Apache 2 License. See the [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) files for details.

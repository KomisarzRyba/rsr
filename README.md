<div align="center">
    <h1>RSR</h1>
    <h3>Reserializer for common data formats</h3>
    <h4>Developed with:</h4>
    <img src=https://skillicons.dev/icons?i=rust />
</div>

## 📖 Table of Contents

-   [📖 Table of Contents](#📖-table-of-contents)
-   [📍 Overview](#📍-overview)
-   [📦 Features](#📦-features)
-   [📂 Repository Structure](#📂-repository-structure)
-   [🤝 Contributing](#🤝-contributing)
-   [📄 License](#📄-license)

## 📍 Overview

The repository contains a Rust project for file reserialization, with support for JSON, YAML, and TOML formats. It provides functionality for parsing command-line arguments, and converting data between different formats. The project includes source files for main functionality, argument parsing, format handling, and content reserialization.

The project's value proposition lies in its ability to process and manipulate data in different file formats, making it a valuable tool for developers working with diverse data sources.

## 📦 Features

### Basic usage

```sh
rsr <INPUT_FILE> <OPTPUT_FORMAT> [OUTPUT_PATH]
```

The above will infer the format of the input file from its extension, and write a reserialized file to the output location. If output is not provided, the file will be saved as a sibling to the input file (with a proper extension).

You can always see the available formats by running `rsr --help`.

## 📂 Repository Structure

```sh
└── rsr/
    ├── Cargo.lock
    ├── Cargo.toml
    └── src/
        ├── args.rs
        ├── formats.rs
        ├── main.rs
        └── reserializers.rs
```

## 🤝 Contributing

Contributions are welcome! Feel free to fork the repo and submit a Pull Request or write an Issue.

### Contributing Guidelines

1. **Fork the Repository**: Start by forking the project repository to your GitHub account.
2. **Clone Locally**: Clone the forked repository to your local machine using a Git client.
    ```sh
    git clone <your-forked-repo-url>
    ```
3. **Create a New Branch**: Always work on a new branch, giving it a descriptive name.
    ```sh
    git checkout -b new-feature-x
    ```
4. **Make Your Changes**: Develop and test your changes locally.
5. **Commit Your Changes**: Commit with a clear and concise message describing your updates.
    ```sh
    git commit -m 'Implemented new feature x.'
    ```
6. **Push to GitHub**: Push the changes to your forked repository.
    ```sh
    git push origin new-feature-x
    ```
7. **Submit a Pull Request**: Create a PR against the original project repository. Clearly describe the changes and their motivations.

Once your PR is reviewed and approved, it will be merged into the main branch.

## 📄 License

This project is protected under the [AGPL-3.0](https://www.gnu.org/licenses/agpl-3.0.en.html) License. For more details, refer to the [LICENSE](https://github.com/KomisarzRyba/rsr/blob/main/LICENSE) file.

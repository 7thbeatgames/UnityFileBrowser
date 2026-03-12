# Unity File Browser

[![openupm](https://img.shields.io/npm/v/com.7thbeat.filebrowser?label=openupm&registry_uri=https://package.openupm.com)](https://openupm.com/packages/com.7thbeat.filebrowser/)

A utility package to use file browser related things in Unity Runtime.

## Features

- File picker(utilizes [rfd](https://docs.rs/rfd/latest/rfd/index.html))
    - Single file
    - Multiple files
    - Single directory
    - Multiple directories
- Reveal a path(utilizes [opener](https://docs.rs/opener/latest/opener/))

## Installation

Option 1: Use [openupm](https://openupm.com/)
   - Run command: `openupm add com.7thbeat.filebrowser`

Option 2: Use git url to install (uses latest commit)
   - `Window` -> `Package Manager` in unity editor
   - Click the `+` button, and `Add package from Git URL`
   - Paste `git+https://github.com/7thbeatgames/UnityFileBrowser.git?path=Packages/com.7thbeat.filebrowser`

## Examples

See the description of each method for more details.

<details>
<summary>Basic file picker</summary>

```cs
var path = FileBrowser.PickFile(
    directory: Application.persistentDataPath,
    filterName: "Log file",
    filterExtensions: new[] { "log" },
    title: "Select a log file"
);
```

</details>

<details>
    <summary>Basic save file picker</summary>

```cs
var path = FileBrowser.SaveFile(
    directory: Application.persistentDataPath,
    filename: "hello.log",
    filterName: "Log file",
    filterExtensions: new[] { "log" },
    title: "save file!"
);
```

</details>

<details>
<summary><code>FileDialog</code> example (Advanced)</summary>

```cs
using var dialog = new FileDialog();
dialog.SetTitle("Example dialog!");
dialog.SetDirectory(Application.persistentDataPath);
dialog.AddFilter("Text file", new[] { "txt" });
dialog.AddFilter("JSON file", new[] { "json" }); // you can add multiple filters!

var path = dialog.PickFile();
```

</details>

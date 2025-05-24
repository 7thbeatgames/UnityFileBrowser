# Unity File Browser

A utility package to use file browser related things in unity runtime

## Features

- File picker(utilizes [rfd](https://docs.rs/rfd/latest/rfd/index.html))
    - Single file
    - Multiple files
    - Single directory
    - Multiple directories
- Reveal a path(utilizes [opener](https://docs.rs/opener/latest/opener/))

## Installation

1. use [openupm](https://openupm.com/)
   - TODO
2. use git url to install(use latest commit)
   - `Window` -> `Package Manager` in unity editor
   - click `+` button, Add package from Git URL
   - Enter `git+https://github.com/7thbeatgames/UnityFileBrowser.git?path=Packages/com.7thbeat.filebrowser`

## Examples

see the description of each method for more details

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
<summary>FileDialog example(Advanced)</summary>

```cs
using var dialog = new FileDialog();
dialog.SetTitle("Example dialog!");
dialog.SetDirectory(Application.persistentDataPath);
dialog.AddFilter("Text file", new[] { "txt" });
dialog.AddFilter("JSON file", new[] { "json" }); // you can add multiple filters!
var path = dialog.PickFile();
```

</details>

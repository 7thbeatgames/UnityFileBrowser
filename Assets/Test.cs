using System.IO;
using TMPro;
using UnityEngine;
using UnityFileDialog;

public class Test : MonoBehaviour
{
    public TMP_Text pickedFileText;

    public void RevealFile()
    {
        FileBrowser.Reveal(Path.Combine(Application.persistentDataPath));
    }

    public void PickFile()
    {
        using var dialog = new FileDialog();
        dialog.SetDirectory(Application.dataPath);
        dialog.AddFilter("얼끼얏호우", new[] { "dll" });
        var path = dialog.PickFile();
        pickedFileText.text = $"Picked file: {path ?? "NONE"}";
    }

    public void PickMultipleFiles()
    {
        using var dialog = new FileDialog();
        dialog.SetDirectory(Application.dataPath);
        var paths = dialog.PickFiles();
        var display = paths == null ? "NONE" : string.Join(", ", paths);
        pickedFileText.text = $"Picked files: {display}";
    }

    public void PickFolder()
    {
        using var dialog = new FileDialog();
        dialog.SetDirectory(Application.dataPath);
        var path = dialog.PickFolder();
        pickedFileText.text = $"Picked folder: {path ?? "NONE"}";
    }

    public void PickMultipleFolders()
    {
        using var dialog = new FileDialog();
        dialog.SetDirectory(Application.dataPath);
        dialog.SetCanCreateDirectories(false);
        var paths = dialog.PickFolders();
        var display = paths == null ? "NONE" : string.Join(", ", paths);
        pickedFileText.text = $"Picked folders: {display}";
    }

    public void SaveFile()
    {
        using var dialog = new FileDialog();
        dialog.SetDirectory(Application.dataPath);
        dialog.AddFilter("Text File", new[] { "txt" });
        dialog.SetFileName("sans.txt");
        dialog.SetTitle("Wow let's save the file!!");
        var path = dialog.SaveFile();
        pickedFileText.text = $"Saved file: {path ?? "NONE"}";
    }
}
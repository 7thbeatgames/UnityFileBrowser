using System.IO;
using TMPro;
using UnityEngine;
using UnityFileDialog;

public class Test : MonoBehaviour
{
    public TMP_Text pickedFileText;

    public void RevealFile()
    {
        Opener.RevealFile(Path.Combine(Application.persistentDataPath));
    }

    public void PickFile()
    {
        using var dialog = new FileDialog();
        dialog.SetDirectory(Application.dataPath);
        dialog.AddFilter("얼끼얏호우", new[] { "dll" });
        var path = dialog.PickFile();
        pickedFileText.text = $"Picked file: {path ?? "NONE"}";
    }

    public void SaveFile()
    {
        using var dialog = new FileDialog();
        dialog.SetDirectory(Application.dataPath);
        dialog.AddFilter("Text File", new[] { "txt" });
        dialog.SetFileName("sans.txt");
        var path = dialog.SaveFile();
        pickedFileText.text = $"Saved file: {path ?? "NONE"}";
    }
}
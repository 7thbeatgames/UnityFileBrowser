using Interop;

namespace UnityFileDialog
{
    public static class Opener
    {
        public static bool RevealFile(string path) => NativeFunctions.opener_reveal_file(path);
    }
}
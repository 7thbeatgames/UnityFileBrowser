using System;
using System.Runtime.InteropServices;

namespace Interop
{
    [StructLayout(LayoutKind.Sequential)]
    internal struct CStringBuffer
    {
        public unsafe IntPtr* data;
        public int length;

        public unsafe string[] GetValue()
        {
            var result = new string[length];
            for (var i = 0; i < length; i++)
            {
                var ptr = data[i];
                result[i] = Marshal.PtrToStringUTF8(ptr);
            }

            return result;
        }
    }
}
using System;
using System.Runtime.InteropServices;
using System.Threading.Tasks;

[StructLayout(LayoutKind.Sequential)]
struct _UniFFIRustBuffer {
    public int Capacity;
    public int Len;
    public IntPtr Data;
}

[StructLayout(LayoutKind.Sequential)]
struct _UniFFIRustCallStatus {
    public sbyte Code;
    public _UniFFIRustBuffer ErrorBuf;
}

public class InternalError : Exception {
    public InternalError(String error)
    : base(error)
    { }
}

class _UniFFILib {
    {%- for func in ci.iter_ffi_function_definitions() %}
        [DllImport("{{ config.cdylib_name() }}", CallingConvention = CallingConvention.Cdecl)]
        public extern static {{ func.return_type()|optional_ffi_type_name }} {{ func.name() }} (
            {%- for arg in func.arguments() %}
                {{ arg.type_().borrow()|ffi_type_name }} {{ arg.name() }},
            {%- endfor %}
            ref _UniFFIRustCallStatus callStatus
        );
    {%- endfor %}
}


class _UniFFIConverterU64  {
    static public ulong Lift(ulong a) {
        return a;
    }
}

class _UniFFIConverterBool {
    static public bool Lift(sbyte a) {
        return a > 0;
    }
}

class _UniFFIConverterString {
    static public string Lift(_UniFFIRustBuffer buffer) {
        if (buffer.Data == IntPtr.Zero) {
            return "";
        }

        string result = Marshal.PtrToStringUTF8(buffer.Data, buffer.Len);
        _UniFFIRustCallStatus _uniffiCallStatus = new _UniFFIRustCallStatus();
        _UniFFILib.ffi_{{ ci.ffi_namespace() }}_rustbuffer_free(buffer, ref _uniffiCallStatus);
        if (_uniffiCallStatus.Code != 0) {
            throw new InternalError("Can't free buffer");
        }
        return result;
    }
}

public class {{ config.class_name() }} {
    {%- for func in ci.function_definitions() %}
        static public {{ func.return_type()|optional_type_name }} {{ func.name().to_upper_camel_case() }} (
            {%- for arg in func.arguments() %}
                {%- if loop.index0 > 0 %},{%- endif %}
                {{ arg.type_().borrow()|type_name }} {{ arg.name().to_lower_camel_case() }}
            {%- endfor %}
            ) {
                _UniFFIRustCallStatus _uniffiCallStatus = new _UniFFIRustCallStatus();
                var _uniffiResult = _UniFFILib.{{ func.ffi_func().name() }}(
                    {%- for arg in func.arguments() %}
                        {{ arg.name().to_lower_camel_case() }},
                    {%- endfor %}
                    ref _uniffiCallStatus
                );
                switch (_uniffiCallStatus.Code)
                {
                case 0:
                    return {{ func.return_type()|optional_lift_function }}(_uniffiResult);
                case 1:
                    throw new Exception("todo");
                default:
                    throw new InternalError(_UniFFIConverterString.Lift(_uniffiCallStatus.ErrorBuf));
                }
                
            }
    {%- endfor %}
}